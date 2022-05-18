use chrono::prelude::*;
use crossterm::event::KeyCode;
use serde_derive::Deserialize;
use std::{collections::HashMap, rc::Rc};
use tui::{
    backend::Backend,
    style::{Color, Style},
    text::Span,
    Frame,
};

pub mod windows;

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
