use std::collections::HashMap;
use crossterm::event::KeyCode;
use tui::{
    backend::Backend,
    Frame,
};
pub mod windows;

pub struct WindowCommand<B: Backend> {
    pub activator_key: KeyCode,
    pub action: fn() -> Option<Window<B>>,
}

impl<B: Backend> WindowCommand<B> {
    pub fn new_char_command(
        activator: char,
        command: fn() -> Option<Window<B>>,
    ) -> WindowCommand<B> {
        WindowCommand {
            activator_key: KeyCode::Char(activator),
            action: command,
        }
    }
}

pub struct Window<B: Backend> {
    pub commands: HashMap<KeyCode, WindowCommand<B>>,
    pub ui: fn(&mut Frame<B>),
}