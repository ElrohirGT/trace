use crate::windows::*;
use crate::{get_app_path, State, Window, WindowCommand};
use crossterm::event::KeyCode;

use std::{collections::HashMap, rc::Rc};
use tui::{backend::Backend, layout::Constraint, layout::Layout, Frame};

pub fn main_menu_window<B: Backend>(_: Rc<State>) -> Box<dyn Fn(&mut Frame<B>)> {
    Box::new(|f| {
        let game_title = "▀█▀ █▀█ ▄▀█ █▀▀ █▀▀\n░█░ █▀▄ █▀█ █▄▄ ██▄";

        let container = Layout::default()
            .horizontal_margin(f.size().width / 4)
            .vertical_margin(f.size().height / 4)
            .constraints([Constraint::Percentage(1)].as_ref())
            .split(f.size());

        let buttons = vec![
            ("P", "ractice"),
            ("S", "tatistics"),
            ("M", "ultiplayer"),
            ("E", "xit"),
        ];

        create_menu(f, container[0], game_title, buttons);
    })
}

pub fn create_main_menu_window<B: 'static + Backend>(state: &mut State) -> Option<Window<B>> {
    let path = get_app_path(".user");
    let user_name = match std::fs::read(path) {
        Ok(bytes) => match std::str::from_utf8(&bytes) {
            Ok(s) => s.to_string(),
            Err(_) => return create_user_window(state),
        },
        Err(_) => return create_user_window(state),
    };
    state.user_name = user_name;
    Some(Window {
        ui: main_menu_window,
        commands: HashMap::from([
            (
                KeyCode::Char('e'),
                WindowCommand::new_char_command('e', Box::new(|_| None)),
            ),
            (
                KeyCode::Char('E'),
                WindowCommand::new_char_command('E', Box::new(|_| None)),
            ),
            (
                KeyCode::Esc,
                WindowCommand {
                    activator_key: KeyCode::Esc,
                    action: Box::new(|_| None),
                },
            ),
            (
                KeyCode::Char('p'),
                WindowCommand::new_char_command('p', Box::new(create_empty_practice_window)),
            ),
            (
                KeyCode::Char('P'),
                WindowCommand::new_char_command('P', Box::new(create_empty_practice_window)),
            ),
            (
                KeyCode::Char('s'),
                WindowCommand::new_char_command('s', Box::new(create_statistics_window)),
            ),
            (
                KeyCode::Char('S'),
                WindowCommand::new_char_command('S', Box::new(create_statistics_window)),
            ),
            (
                KeyCode::Char('m'),
                WindowCommand::new_char_command('m', Box::new(create_multiplayer_menu_window)),
            ),
            (
                KeyCode::Char('M'),
                WindowCommand::new_char_command('M', Box::new(create_multiplayer_menu_window)),
            ),
        ]),
    })
}
