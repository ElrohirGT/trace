use tui::text::Text;
use crate::windows::*;
use crate::{State, Window, WindowCommand};
use crossterm::event::KeyCode;
use std::{collections::HashMap, rc::Rc};
use tui::widgets::Row;
use tui::widgets::Table;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

fn end_window<B: Backend>(state: Rc<State>) -> Box<dyn Fn(&mut Frame<B>)> {
    Box::new(move |f| {
        let accuracy =
            (state.chars.len() - state.total_error_count) as f64 / state.chars.len() as f64;
        let formatted_accuracy = format!("{:.2} %", accuracy * 100.0);
        let duration = state.end_time - state.initial_time;
        let seconds = (duration.num_milliseconds() as f64) / 1000.0;
        let formatted_seconds = format!("{:.2}", seconds);

        let word_count = state.word_count as f64;
        let wpm = word_count / seconds * 60.0;
        let formatted_wpm = format!("{:.2}", wpm);

        let total_points = (wpm + accuracy * wpm) / 2.0;
        let formatted_total_points = format!("{:.2}", total_points);
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(35),
                    Constraint::Percentage(50),
                    Constraint::Percentage(5),
                ]
                .as_ref(),
            )
            .margin(10)
            .split(f.size());
        let header_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(20),
                Constraint::Percentage(40),
                Constraint::Percentage(40),
            ].as_ref())
            .split(layout[0]);
        let info_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ].as_ref())
            .split(header_layout[2]);

        let control_buttons = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(20),
                    Constraint::Percentage(20),
                    Constraint::Percentage(20),
                    Constraint::Percentage(20),
                ]
                .as_ref(),
            )
            .split(layout[2]);

        let thanks = Paragraph::new("Thank you for playing!").alignment(Alignment::Center);
        f.render_widget(thanks, header_layout[0]);
        
        let title = Paragraph::new(Text::raw(&state.paragraph.title)).style(Style::default().fg(Color::LightCyan)).alignment(Alignment::Center);
        f.render_widget(title, header_layout[1]);

        let author = Paragraph::new(Text::raw(&state.paragraph.author)).style(Style::default().fg(Color::LightCyan)).alignment(Alignment::Center);
        f.render_widget(author, info_layout[0]);

        let date = Paragraph::new(Text::raw(&state.paragraph.date)).style(Style::default().fg(Color::LightCyan)).alignment(Alignment::Center);
        f.render_widget(date, info_layout[1]);

        let table = Table::new(vec![
            Row::new(vec!["#", "Name", "Points", "Time (s)", "Accuracy", "WPM"]).style(
                Style::default()
                    .fg(Color::LightCyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Row::new(vec![
                "1",
                "You",
                &formatted_total_points,
                &formatted_seconds,
                &formatted_accuracy,
                &formatted_wpm,
            ]),
        ])
        .block(Block::default().borders(Borders::LEFT | Borders::RIGHT))
        .widths(
            [
                Constraint::Percentage(5),
                Constraint::Percentage(35),
                Constraint::Percentage(15),
                Constraint::Percentage(15),
                Constraint::Percentage(15),
                Constraint::Percentage(15),
            ]
            .as_ref(),
        )
        .column_spacing(1);
        f.render_widget(table, layout[1]);

        let reset_button = create_ui_button("R", "eset");
        f.render_widget(reset_button, control_buttons[0]);
        let quit_button = create_ui_button("M", "enu");
        f.render_widget(quit_button, control_buttons[1]);
        let statistics_button = create_ui_button("E", "xit");
        f.render_widget(statistics_button, control_buttons[2]);
        let statistics_button = create_ui_button("S", "tatistics");
        f.render_widget(statistics_button, control_buttons[3]);
    })
}

pub fn create_end_window<B: 'static + Backend>(_: &mut State) -> Option<Window<B>> {
    Some(Window {
        ui: end_window,
        commands: HashMap::from([
            (
                KeyCode::Char('e'),
                WindowCommand::new_char_command('e', Box::new(|_| None)),
            ),
            (
                KeyCode::Esc,
                WindowCommand {
                    activator_key: KeyCode::Esc,
                    action: Box::new(|_| None),
                },
            ),
            (
                KeyCode::Char('r'),
                WindowCommand::new_char_command('r', Box::new(create_empty_practice_window)),
            ),
            (
                KeyCode::Char('m'),
                WindowCommand::new_char_command('m', Box::new(create_main_menu_window)),
            ), //TODO: Implement the statistics command
        ]),
    })
}
