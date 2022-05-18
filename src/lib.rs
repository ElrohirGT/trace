use std::path::Path;
use std::path::PathBuf;
use chrono::prelude::*;
use crossterm::event::KeyCode;
use serde_derive::{Deserialize, Serialize};
use std::{collections::HashMap, rc::Rc};
use tui::{
    backend::Backend,
    style::{Color, Style},
    text::Span,
    Frame,
};

pub mod windows;

#[derive(Deserialize, Serialize, Clone)]
pub struct TraceRun {
    wpm: f64,
    accuracy: f64,
    total_points: f64
}

impl TraceRun {
    pub fn new(wpm: f64, accuracy: f64, total_points: f64) -> TraceRun {
        TraceRun {
            wpm,
            accuracy,
            total_points
        }
    }
}

pub fn save_track_record(runs: &Vec<TraceRun>) -> Result<(), std::io::Error> {
    let path = get_app_path(".track_record.json");
    let json = serde_json::to_string(&runs)?;
    std::fs::write(path, json)
}

pub fn get_track_record() -> Vec<TraceRun> {
    let path = get_app_path(".track_record.json");
    match std::fs::read(path) {
        Ok(bytes) => match serde_json::from_slice(&bytes) {
            Ok(runs) => runs,
            Err(_) => Vec::new()
        },
        Err(_) => Vec::new()
    }
}

#[derive(Deserialize, Clone)]
pub struct AppParagraph {
    content: String,
    title: String,
    author: String,
    date: String,
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
pub struct State {
    user_name: String,
    chars: Vec<ParagraphChar>,
    paragraph: AppParagraph,
    initial_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
    current_error_count: usize,
    total_error_count: usize,
    word_count: usize,
    index: usize,
}

impl State {
    pub fn new() -> State {
        State {
            chars: vec![],
            user_name: String::new(),
            paragraph: AppParagraph::new(),
            initial_time: Utc::now(),
            end_time: Utc::now(),
            current_error_count: 0,
            total_error_count: 0,
            word_count: 0,
            index: 0,
        }
    }
    pub fn reset(&mut self) {
        self.chars = vec![];
        self.paragraph = AppParagraph::new();
        self.initial_time = Utc::now();
        self.end_time = Utc::now();
        self.current_error_count = 0;
        self.total_error_count = 0;
        self.word_count = 0;
        self.index = 0;
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
    commands: &mut HashMap<KeyCode, WindowCommand<B>>,
    char_array: &Vec<char>,
    cmd: Box<dyn Fn(char)-> Box<dyn Fn(&mut State) -> Option<Window<B>>>>
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