use chrono::prelude::*;
use crossterm::event::KeyCode;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::path::PathBuf;
use std::{collections::HashMap, rc::Rc};
use tui::{
    backend::Backend,
    style::{Color, Style},
    text::Span,
    Frame,
};

pub mod windows;

pub struct Padding {
    width: u16,
    height: u16,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct TraceRun {
    wpm: f64,
    accuracy: f64,
    total_points: f64,
    seconds: f64,
}

impl TraceRun {
    pub fn to_csv(&self) -> String {
        format!(
            "{:},{:},{:},{:}",
            self.wpm, self.accuracy, self.total_points, self.seconds
        )
    }
}

pub fn get_track_record() -> Vec<TraceRun> {
    let path = get_app_path(".runs.csv");
    match csv::Reader::from_path(path) {
        Ok(mut reader) => {
            let mut records = vec![];
            for result in reader.deserialize() {
                if let Ok(record) = result {
                    records.push(record)
                }
            }
            records
        }
        Err(_) => Vec::new(),
    }
}

#[derive(Deserialize, Clone, Default)]
pub struct AppParagraph {
    content: String,
    title: String,
    author: String,
    date: String
}

impl AppParagraph {
    pub fn new() -> AppParagraph {
        AppParagraph {
            content: "".to_string(),
            title: "".to_string(),
            author: "".to_string(),
            date: "".to_string(),
        }
    }
    pub fn get_paragraph_chars(&self) -> Vec<ParagraphChar> {
        self.content.chars().map(|c| ParagraphChar::new(c, CharStatus::Default)).collect()
    }
    pub fn get_word_count(&self) -> usize {
        self.content.split(' ').count()
    }
}

#[derive(Clone)]
pub enum CharStatus {
    Correct,
    Wrong,
    Default,
    Current,
}

#[derive(Clone)]
pub struct ParagraphChar {
    character: char,
    status: CharStatus,
}

impl ParagraphChar {
    pub fn new(c: char, status: CharStatus) -> ParagraphChar {
        ParagraphChar {
            character: c,
            status,
        }
    }
    pub fn to_span(&self) -> Span {
        match self.status {
            CharStatus::Correct => Span::styled(
                self.character.to_string(),
                Style::default().fg(Color::Green),
            ),
            CharStatus::Current => Span::styled(
                self.character.to_string(),
                Style::default().fg(Color::White).bg(Color::DarkGray),
            ),
            CharStatus::Wrong => {
                if self.character == ' ' {
                    Span::styled(self.character.to_string(), Style::default().bg(Color::Red))
                } else {
                    Span::styled(self.character.to_string(), Style::default().fg(Color::Red))
                }
            }
            CharStatus::Default => Span::styled(
                self.character.to_string(),
                Style::default().fg(Color::DarkGray),
            ),
        }
    }
}

pub fn convert_string_to_chars(s: String) -> Vec<ParagraphChar> {
    let mut vector = vec![];
    for elem in s.chars() {
        vector.push(ParagraphChar::new(elem, CharStatus::Default));
    }
    return vector;
}

#[derive(Clone)]
pub struct PlayerStatistics {
    initial_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
    current_error_count: usize,
    total_error_count: usize,
    word_count: usize
}

impl Default for PlayerStatistics {
    fn default() -> Self {
        PlayerStatistics {
            initial_time: Utc::now(),
            end_time: Utc::now(),
            current_error_count: 0,
            total_error_count: 0,
            word_count: 0
        }
    }
}

#[derive(Clone, Default)]
pub struct Player {
    user_name: String,
    statistics: PlayerStatistics,
    index: usize
}

impl Player {
    fn reset(&mut self) {
        self.index = 0;
        self.statistics = PlayerStatistics::default()
    }
}

#[derive(Clone, Default)]
pub struct State {
    player: Player,
    oponents: Vec<Player>,
    chars: Vec<ParagraphChar>,
    show_bar_charts: bool,
    paragraph: AppParagraph
}

impl State {
    pub fn reset(&mut self) {
        self.chars = vec![];
        self.paragraph = AppParagraph::new();
        self.show_bar_charts = false;
    }
    pub fn create_run(&self) -> TraceRun {
        let accuracy = (self.chars.len() - self.player.statistics.total_error_count) as f64 / self.chars.len() as f64;
        let duration = self.player.statistics.end_time - self.player.statistics.initial_time;
        let seconds = (duration.num_milliseconds() as f64) / 1000.0;

        let wpm = self.player.statistics.word_count as f64 / seconds * 60.0;
        let total_points = (wpm + accuracy * wpm) / 2.0;
        TraceRun {
            wpm,
            accuracy,
            total_points,
            seconds,
        }
    }
}

pub struct WindowCommand<B: Backend> {
    pub activator_key: KeyCode,
    pub action: Box<dyn Fn(&mut State) -> Option<Window<B>>>,
}

impl<B: Backend> WindowCommand<B> {
    pub fn new_char_command(
        activator: char, command: Box<dyn Fn(&mut State) -> Option<Window<B>>>,
    ) -> WindowCommand<B> {
        WindowCommand {
            activator_key: KeyCode::Char(activator),
            action: command,
        }
    }
}

pub struct Window<B: Backend> {
    pub commands: HashMap<KeyCode, WindowCommand<B>>,
    pub ui: fn(Rc<State>) -> Box<dyn Fn(&mut Frame<B>)>,
}

pub fn get_app_path(file_path: &str) -> PathBuf {
    let current_dir = std::env::current_dir().unwrap();
    Path::new(&current_dir).join(file_path)
}

pub fn generate_all_chars() -> Vec<char> {
    let mut chars = vec![
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];
    let mut upper_chars: Vec<char> = chars.iter().map(|a| a.to_ascii_uppercase()).collect();
    let mut punctuation = vec![
        ' ', ',', '.', ':', '"', '-', '@', ';', '<', '>', '+', '-', '_', '(', ')', '=', '*', '/',
        '¡', '!', '¿', '?', '#', '$', '%', '&', '°', '\'', '^', '~', '[', ']', '{', '}',
    ];
    let mut numbers = vec!['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'];
    let mut extras = vec![
        'á', 'Á', 'é', 'É', 'í', 'Í', 'ó', 'Ó', 'ú', 'Ú', 'ä', 'Ä', 'ë', 'Ë', 'ï', 'Ï', 'ö', 'Ö',
        'ü', 'Ü', 'ç', 'ñ', 'Ñ',
    ];

    chars.append(&mut upper_chars);
    chars.append(&mut punctuation);
    chars.append(&mut numbers);
    chars.append(&mut extras);

    chars
}

pub fn add_to_commands<B: 'static + Backend>(
    commands: &mut HashMap<KeyCode, WindowCommand<B>>, char_array: &Vec<char>,
    cmd: Box<dyn Fn(char) -> Box<dyn Fn(&mut State) -> Option<Window<B>>>>,
) {
    for elem in char_array {
        commands.insert(
            KeyCode::Char(*elem),
            WindowCommand {
                activator_key: KeyCode::Char(*elem),
                action: cmd(*elem),
            },
        );
    }
}
