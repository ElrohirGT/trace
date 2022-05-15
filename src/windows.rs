use std::rc::Rc;
use crate::{Window, WindowCommand, State, ParagraphChar};
use crossterm::event::KeyCode;
use std::collections::HashMap;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
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

pub fn create_main_menu_window<B: Backend>(_: &mut State) -> Option<Window<B>> {
    Some(Window {
        ui: main_menu_window,
        commands: HashMap::from([
            (
                KeyCode::Char('e'),
                WindowCommand::new_char_command('e', |_| None),
            ),
            (
                KeyCode::Char('p'),
                WindowCommand::new_char_command('p', create_empty_practice_window),
            ),
        ]),
    })
}

pub fn practice_window<B: Backend>(state: Rc<State>) -> Box<dyn Fn(&mut Frame<B>)> {
    Box::new(move |f| {
        let spans: Vec<Span> = state.chars.iter().map(|c| c.to_span()).collect();
        let title = Paragraph::new(vec![Spans::from(spans)])
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: false });

        f.render_widget(title, f.size());
    })
}
fn create_empty_practice_window<B: Backend>(state: &mut State) -> Option<Window<B>> {
    state.chars = get_random_practice_text();
    create_practice_window(state)
}
fn get_random_practice_text() -> Vec<ParagraphChar> {
    return vec![
        ParagraphChar::Default('H'),
        ParagraphChar::Default('e'),
        ParagraphChar::Default('l'),
        ParagraphChar::Default('l'),
        ParagraphChar::Default('o'),
    ]
}
fn create_practice_window<B: Backend>(state: &mut State) -> Option<Window<B>> {
    Some(Window {
        ui: practice_window,
        commands: HashMap::from([(
            KeyCode::Esc,
            WindowCommand {
                activator_key: KeyCode::Esc,
                action: create_main_menu_window,
            },
        )]),
    })
}

fn create_ui_button<'a>(activator: &'a str, rest: &'a str) -> Paragraph<'a> {
    let button_text = vec![Spans::from(vec![
        Span::styled(
            activator,
            Style::default()
                .add_modifier(Modifier::UNDERLINED | Modifier::BOLD)
                .fg(Color::Yellow),
        ),
        Span::raw(rest),
    ])];

    Paragraph::new(button_text)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL))
}
