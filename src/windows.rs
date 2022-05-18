use tui::{
    layout::Alignment,
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
};

pub mod main_menu_window;
pub use main_menu_window::*;

pub mod practice_window;
pub use practice_window::*;

pub mod end_window;
pub use end_window::*;

pub mod user_window;
pub use user_window::*;

pub fn create_label_widget<'a>(label: &'a str, value: &'a str, color: Color) -> Paragraph<'a> {
    Paragraph::new(vec![Spans::from(vec![
        Span::from(label),
        Span::styled(value, Style::default().fg(color)),
    ])])
    .alignment(Alignment::Center)
}

pub fn create_ui_button<'a>(activator: &'a str, rest: &'a str) -> Paragraph<'a> {
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
