use crossterm::event::{Event, KeyCode};
use std::collections::HashMap;
use tui::widgets::Wrap;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct WindowCommand<B: Backend> {
    pub activator_key: KeyCode,
    // pub action: F
    pub action: fn() -> Option<Window<B>>, // pub command: Box<dyn Fn() -> Option<Window<B>>>
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

pub fn main_menu_window<B: Backend>(f: &mut Frame<B>) {
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
    f.render_widget(exit_button, chunks[2])
}

pub fn practice_window<B: Backend>(f: &mut Frame<B>) {
    // let chunks = Layout::default()
    //     .direction(Direction::Vertical)
    //     .constraints([Constraint::Percentage(100)].as_ref())
    //     .split(f.size());
    let title = Paragraph::new(Text::styled(
        "Texto de prueba!!!!!! Chucuchun chucuchun chucuchun",
        Style::default()
            .add_modifier(Modifier::BOLD)
            .fg(Color::Gray),
    ))
    .alignment(Alignment::Center)
    .wrap(Wrap { trim: false });

    f.render_widget(title, f.size());
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
