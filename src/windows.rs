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

pub fn create_menu<'a, B: Backend>(
    f: &mut Frame<B>, container: Rect, title: &str, buttons: Vec<(&'a str, &'a str)>,
) {
    create_menu_pad(
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

pub fn create_menu_pad<'a, B: Backend>(
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
    let mut pars = vec![title_par];
    let mut button_pars: Vec<Paragraph> = buttons
        .iter()
        .map(|(activator, rest)| create_ui_button(activator, rest))
        .collect();
    pars.append(&mut button_pars);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .vertical_margin(padding.height)
        .horizontal_margin(padding.width)
        .constraints(constraints)
        .split(container);

    let vcentered_chunks: Vec<Rect> = chunks
        .iter()
        .map(|&c| {
            Layout::default()
                .direction(Direction::Vertical)
                .vertical_margin(c.height / 2)
                .constraints([Constraint::Percentage(1)])
                .split(c)[0]
        })
        .collect();

    f.render_widget(pars[0].clone(), chunks[0]);
    for i in 1..pars.len() {
        let button_wrapper = Block::default().borders(Borders::ALL);
        f.render_widget(button_wrapper, chunks[i]);
        f.render_widget(pars[i].clone(), vcentered_chunks[i]);
    }
}

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

    Paragraph::new(button_text).alignment(Alignment::Center)
}
