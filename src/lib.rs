use std::rc::Rc;
use std::collections::HashMap;
use crossterm::event::KeyCode;
use tui::{
    backend::Backend,
    style::{Style, Color},
    Frame,
    text::Span
};
// use serde::{Deserialize};
use serde_derive::{Deserialize};

pub mod windows;

#[derive(Deserialize, Debug)]
pub struct AppParagraph {
    content: String,
    title: String,
    author: String,
    date: String
}

#[derive(Clone)]
pub enum CharStatus {
    Correct,
    Wrong,
    Default
}

#[derive(Clone)]
pub struct ParagraphChar {
    character: char,
    status: CharStatus
}

impl ParagraphChar {
    pub fn new(c: char, status: CharStatus) -> ParagraphChar {
        ParagraphChar {
            character: c,
            status
        }
    }
    pub fn to_span(&self) -> Span {
        match self.status {
            CharStatus::Correct => Span::styled(self.character.to_string(), Style::default().fg(Color::Green)),
            CharStatus::Wrong => {
                if self.character == ' '{
                    Span::styled(self.character.to_string(), Style::default().bg(Color::Red))
                }
                else{
                    Span::styled(self.character.to_string(), Style::default().fg(Color::Red))

                }
            },
            CharStatus::Default => Span::styled(self.character.to_string(), Style::default().fg(Color::DarkGray))
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
    error_count: u16,
    index: usize
}

impl State {
    pub fn new() -> State {
        State {
            chars: vec![],
            error_count: 0,
            index: 0
        }
    }
}

pub struct WindowCommand<B: Backend> {
    pub activator_key: KeyCode,
    pub action:  Box<dyn Fn(&mut State) -> Option<Window<B>>>,
}

impl<B: Backend> WindowCommand<B> {
    pub fn new_char_command(
        activator: char,
        command: Box<dyn Fn(&mut State) -> Option<Window<B>>>,
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