use crate::add_to_commands;
use crate::generate_all_chars;
use crate::get_app_path;
use crate::windows::create_main_menu_window;
use crate::State;
use crate::Window;
use crate::WindowCommand;
use crossterm::event::KeyCode;
use std::collections::HashMap;
use std::rc::Rc;
use tui::backend::Backend;
use tui::layout::Alignment;
use tui::style::Color;
use tui::style::Style;
use tui::text::Span;
use tui::text::Spans;
use tui::widgets::Paragraph;
use tui::Frame;

fn user_window<B: 'static + Backend>(state: Rc<State>) -> Box<dyn Fn(&mut Frame<B>)> {
    Box::new(move |f| {
        let paragraph = Paragraph::new(vec![
            Spans::from("Please write your username:"),
            Spans::from(vec![Span::styled(
                state.player.user_name.clone(),
                Style::default().fg(Color::Yellow),
            )]),
        ])
        .alignment(Alignment::Center);
        f.render_widget(paragraph, f.size());
    })
}

fn handle_char_press<B: 'static + Backend>(
    c: char,
) -> Box<dyn Fn(&mut State) -> Option<Window<B>>> {
    Box::new(move |state: &mut State| {
        state.player.user_name.push(c);
        create_user_window(state)
    })
}

pub fn create_user_window<B: 'static + Backend>(_: &mut State) -> Option<Window<B>> {
    let chars = generate_all_chars();
    let mut commands = HashMap::new();
    add_to_commands(&mut commands, &chars, Box::new(handle_char_press));

    fn handle_backspace_press<B: 'static + Backend>(state: &mut State) -> Option<Window<B>> {
        state.player.user_name.pop();
        create_user_window(state)
    }
    commands.insert(
        KeyCode::Backspace,
        WindowCommand {
            activator_key: KeyCode::Backspace,
            action: Box::new(handle_backspace_press),
        },
    );
    commands.insert(
        KeyCode::Enter,
        WindowCommand {
            activator_key: KeyCode::Enter,
            action: Box::new(|state| {
                let path = get_app_path(".user");
                match std::fs::write(path, state.player.user_name.to_string()) {
                    Ok(_) => create_main_menu_window(state),
                    Err(_) => create_user_window(state),
                }
            }),
        },
    );

    Some(Window {
        ui: Box::new(user_window),
        commands,
    })
}
