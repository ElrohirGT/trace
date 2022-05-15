use std::rc::Rc;
use std::collections::HashMap;
use crossterm::event::KeyCode;
use tui::{
    backend::Backend,
    style::{Style, Color},
    Frame,
    text::Span
};
pub mod windows;

#[derive(Clone)]
pub enum ParagraphChar {
    Correct(char),
    Wrong(char),
    Default(char)
}

impl ParagraphChar {
    pub fn to_span(&self) -> Span {
        match self {
            ParagraphChar::Correct(c) => Span::styled(c.to_string(), Style::default().fg(Color::Green)),
            ParagraphChar::Wrong(c) => Span::styled(c.to_string(), Style::default().fg(Color::Red)),
            ParagraphChar::Default(c) => Span::styled(c.to_string(), Style::default().fg(Color::DarkGray))
        }
    }
}

#[derive(Clone)]
pub struct State {
    chars: Vec<ParagraphChar>
}

impl State {
    pub fn new() -> State {
        State {
            chars: vec![]
        }
    }
    pub fn from(s: String) -> State {
        let mut chars = vec![];
        for elem in s.chars() {
            chars.push(ParagraphChar::Default(elem));
        }
        State {
            chars
        }
    }
}

pub struct WindowCommand<B: Backend> {
    pub activator_key: KeyCode,
    pub action:  fn(&mut State) -> Option<Window<B>>,
}

impl<B: Backend> WindowCommand<B> {
    pub fn new_char_command(
        activator: char,
        command: fn(&mut State) -> Option<Window<B>>,
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