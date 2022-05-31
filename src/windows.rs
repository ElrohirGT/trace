use crate::Padding;
use tui::backend::Backend;
use tui::layout::Constraint;
use tui::layout::Direction;
use tui::layout::Layout;
use tui::layout::Rect;
use tui::text::Text;
use tui::Frame;
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

pub mod statistics_window;
pub use statistics_window::*;

pub mod multiplayer_menu_window;
pub use multiplayer_menu_window::*;

pub mod create_server_window;
pub use create_server_window::*;

pub mod join_server_window;
pub use join_server_window::*;

pub mod error_window;
pub use error_window::*;

pub fn create_menu<'a, B: Backend>(
    f: &mut Frame<B>, container: Rect, title: &str, buttons: Vec<(&'a str, &'a str)>,
) {
    create_menu_with_pad(
        f,
        container,
        title,
        buttons,
        Padding {
            width: container.width / 3,
            height: container.height / 10,
        },
    );
}

pub fn create_menu_with_pad<'a, B: Backend>(
    f: &mut Frame<B>, container: Rect, title: &str, buttons: Vec<(&'a str, &'a str)>,
    padding: Padding,
) {
    let main_block = Block::default().borders(Borders::ALL);
    f.render_widget(main_block, container);

    let mut constraints = vec![Constraint::Percentage(20)];
    let mut buttons_constraints: Vec<Constraint> = (0..buttons.len())
        .map(|_| Constraint::Percentage((80.0 / buttons.len() as f64) as u16))
        .collect();
    constraints.append(&mut buttons_constraints);
    constraints.push(Constraint::Percentage(1)); // Padding just so the last element doesn't get stretched

    let title_par = Paragraph::new(Text::styled(
        title,
        Style::default().add_modifier(Modifier::BOLD).fg(Color::Red),
    ))
    .alignment(Alignment::Center);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .vertical_margin(padding.height)
        .horizontal_margin(padding.width)
        .constraints(constraints)
        .split(container);

    f.render_widget(title_par, chunks[0]);
    for i in 0..buttons.len() {
        create_centered_button(buttons[i].0, buttons[i].1, chunks[i+1], f);
    }
}

pub fn create_label_widget<'a>(label: &'a str, value: &'a str, color: Color) -> Paragraph<'a> {
    Paragraph::new(vec![Spans::from(vec![
        Span::from(label),
        Span::styled(value, Style::default().fg(color)),
    ])])
    .alignment(Alignment::Center)
}

pub fn create_centered_button<'a, B: Backend>(activator: &'a str, rest: &'a str, container: Rect, f: &mut Frame<B>) {
    let text = create_ui_button(activator, rest);
    let borders = Block::default().borders(Borders::ALL);
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .vertical_margin((container.height as f32 / 2.0) as u16)
        .constraints([Constraint::Percentage(1)])
        .split(container);
    f.render_widget(borders, container);
    f.render_widget(text, layout[0]);
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

    Paragraph::new(button_text).alignment(Alignment::Center)
}
