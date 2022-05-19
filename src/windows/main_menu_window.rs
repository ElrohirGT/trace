use crate::windows::*;
use crate::{get_app_path, State, Window, WindowCommand};
use crossterm::event::KeyCode;

use std::{collections::HashMap, rc::Rc};
use tui::{
    backend::Backend, layout::Alignment, layout::Constraint, layout::Direction, layout::Layout,
    style::Color, style::Modifier, style::Style, text::Text, widgets::Block, widgets::Borders,
    widgets::Paragraph, Frame,
};

pub fn main_menu_window<B: Backend>(_: Rc<State>) -> Box<dyn Fn(&mut Frame<B>)> {
    Box::new(|f| {
        let game_title = "▀█▀ █▀█ ▄▀█ █▀▀ █▀▀\n░█░ █▀▄ █▀█ █▄▄ ██▄";

        let mut form_size = f.size();
        form_size.x = form_size.width / 4;
        form_size.y = form_size.height / 4;
        form_size.width /= 2;
        form_size.height /= 2;

        let main_block = Block::default().borders(Borders::ALL);
        f.render_widget(main_block, form_size);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .vertical_margin(form_size.height / 8)
            .horizontal_margin(form_size.width / 3)
            .constraints(
                [
                    Constraint::Percentage(33),
                    Constraint::Percentage(20),
                    Constraint::Percentage(20),
                    Constraint::Percentage(1),
                ]
                .as_ref(),
            )
            .split(form_size);
        let title = Paragraph::new(Text::styled(
            game_title,
            Style::default().add_modifier(Modifier::BOLD).fg(Color::Red),
        ))
        .alignment(Alignment::Center);
        f.render_widget(title, chunks[0]);
        let practice_button = create_ui_button("P", "ractice");
        f.render_widget(practice_button, chunks[1]);

        let exit_button = create_ui_button("E", "xit");
        f.render_widget(exit_button, chunks[2]);
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
                WindowCommand::new_char_command('s', Box::new(create_statistics_window))
            )
        ]),
    })
}
