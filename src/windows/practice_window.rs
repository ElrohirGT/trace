use crate::add_to_commands;
use crate::generate_all_chars;
use crate::get_app_path;
use crate::{
    windows::*, AppParagraph, CharStatus, ParagraphChar, State, Utc,
    Window, WindowCommand,
};
use crossterm::event::KeyCode;
use rand::prelude::SliceRandom;
use std::{collections::HashMap, rc::Rc};
use tui::{
    backend::Backend, layout::Alignment, layout::Constraint, layout::Direction, layout::Layout,
    style::Color, style::Modifier, style::Style, text::Span, text::Spans, widgets::Block,
    widgets::Borders, widgets::Gauge, widgets::Paragraph, widgets::Wrap, Frame,
};

pub fn practice_window<B: Backend>(state: Rc<State>) -> Box<dyn Fn(&mut Frame<B>)> {
    Box::new(move |f| {
        let spans: Vec<Span> = state.chars.iter().map(|c| c.to_span()).collect();
        let layout = Layout::default()
            .vertical_margin(f.size().height / 5)
            .horizontal_margin(f.size().width / 3)
            .constraints(
                [
                    Constraint::Percentage(50), //Paragraph space
                    Constraint::Percentage(10), //Live statistics space
                    Constraint::Percentage(40), //Paragraph information
                ]
                .as_ref(),
            )
            .split(f.size());
        let statistics = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(layout[1]);
        let progress_info = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(20), //First player progress bar
                    Constraint::Percentage(1),
                ]
                .as_ref(),
            )
            .split(layout[2]);

        let paragraph = Paragraph::new(vec![Spans::from(spans)])
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: false });
        f.render_widget(paragraph, layout[0]);

        let player_statistics = &state.player.statistics;
        let time_elapsed = Utc::now() - player_statistics.initial_time;
        let wpm = player_statistics.word_count as f64 / (time_elapsed.num_milliseconds() as f64 / 1000.0 / 60.0);
        let formatted_wpm = format!("{:.2}", wpm);
        let wpm_widget = create_label_widget("WPM: ", &formatted_wpm, Color::Yellow);
        f.render_widget(wpm_widget, statistics[0]);

        let accuracy =
            (state.chars.len() - player_statistics.total_error_count) as f64 / state.chars.len() as f64 * 100.0;
        let formatted_accuracy = format!("{:.2} %", accuracy);
        let accuracy_widget = create_label_widget("Accuracy: ", &formatted_accuracy, Color::Yellow);
        f.render_widget(accuracy_widget, statistics[1]);

        let progress = state.player.index as f64 / state.chars.len() as f64 * 100.0;
        let progress_widget = Gauge::default()
            .block(
                Block::default()
                    .borders(Borders::TOP)
                    .title(state.player.user_name.to_string())
                    .border_style(Style::default().fg(Color::DarkGray)),
            )
            .gauge_style(
                Style::default()
                    .fg(Color::LightCyan)
                    .bg(Color::Black)
                    .add_modifier(Modifier::ITALIC),
            )
            .percent(progress as u16);
        f.render_widget(progress_widget, progress_info[0]);
    })
}
pub fn create_empty_practice_window<B: 'static + Backend>(state: &mut State) -> Option<Window<B>> {
    state.player.reset();
    state.paragraph = match get_random_app_paragraph() {
        Ok(p) => p,
        Err(err) => return create_error_window(format!("Sorry an error ocurred while retrieving the database.csv\n{}", err), create_main_menu_window)
    };
    state.player.statistics.word_count = state.paragraph.get_word_count();
    state.chars = state.paragraph.get_paragraph_chars();
    create_practice_window(state)
}
fn get_random_app_paragraph() -> Result<AppParagraph, csv::Error> {
    let path = get_app_path("database.csv");
    csv::Reader::from_path(&path)
        .and_then(|mut reader| {
            let mut records: Vec<AppParagraph> = vec![];
            for result in reader.deserialize() {
                match result {
                    Ok(r) => records.push(r),
                    Err(r) => return Err(r),
                }
            }
            Ok(records)
        })
        .and_then(|paragraphs: Vec<AppParagraph>| {
            let random_par = paragraphs.choose(&mut rand::thread_rng());
            Ok(random_par
                .expect("Couldn't get a random paragraph!")
                .clone())
        })
}
fn create_practice_window<B: 'static + Backend>(_: &mut State) -> Option<Window<B>> {
    fn handle_backspace_press<B: 'static + Backend>(state: &mut State) -> Option<Window<B>> {
        if state.player.index != state.chars.len() {
            state.chars[state.player.index] =
                ParagraphChar::new(state.chars[state.player.index].character, CharStatus::Default);
        }
        if state.player.index > 0 {
            //Going back to the previous inputted char, because the current is not inputted.
            state.player.index -= 1;
        }
        let current_char = &state.chars[state.player.index];
        let defaulted_char = match current_char.status {
            CharStatus::Current => ParagraphChar::new(current_char.character, CharStatus::Current),
            CharStatus::Correct => ParagraphChar::new(current_char.character, CharStatus::Current),
            CharStatus::Wrong => {
                state.player.statistics.current_error_count -= 1;
                ParagraphChar::new(current_char.character, CharStatus::Current)
            }
            CharStatus::Default => ParagraphChar::new(current_char.character, CharStatus::Current),
        };
        state.chars[state.player.index] = defaulted_char;
        create_practice_window(state)
    }

    let mut commands = HashMap::from([
        (
            KeyCode::Esc,
            WindowCommand {
                activator_key: KeyCode::Esc,
                action: Box::new(create_main_menu_window),
            },
        ),
        (
            KeyCode::Backspace,
            WindowCommand {
                activator_key: KeyCode::Backspace,
                action: Box::new(handle_backspace_press),
            },
        ),
    ]);

    let chars = generate_all_chars();
    add_to_commands(&mut commands, &chars, Box::new(handle_char_press));
    Some(Window {
        ui: practice_window,
        commands,
    })
}

fn handle_char_press<B: 'static + Backend>(
    pressed_character: char,
) -> Box<dyn Fn(&mut State) -> Option<Window<B>>> {
    Box::new(move |state: &mut State| {
        let current_char = &state.chars[state.player.index];
        let is_correct = current_char.character == pressed_character;
        let status = if is_correct {
            CharStatus::Correct
        } else {
            CharStatus::Wrong
        };

        let transformed_char = ParagraphChar::new(current_char.character, status);
        state.chars[state.player.index] = transformed_char;

        state.player.index += 1;

        if !is_correct {
            state.player.statistics.current_error_count += 1;
            state.player.statistics.total_error_count += 1;
        }

        let end_of_paragraph = state.player.index == state.chars.len();

        if end_of_paragraph && state.player.statistics.current_error_count == 0 {
            state.player.statistics.end_time = Utc::now();
            create_end_window(state)
        } else {
            if !end_of_paragraph {
                let current_char = &state.chars[state.player.index];
                let transformed_char =
                    ParagraphChar::new(current_char.character, CharStatus::Current);
                state.chars[state.player.index] = transformed_char;
            }
            create_practice_window(state)
        }
    })
}
