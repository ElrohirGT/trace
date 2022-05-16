use std::rc::Rc;
use crate::{Window, WindowCommand, State, ParagraphChar, CharStatus, convert_string_to_chars};
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

pub fn create_main_menu_window<B: 'static + Backend>(_: &mut State) -> Option<Window<B>> {
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
                WindowCommand{ activator_key: KeyCode::Esc, action: Box::new(|_| None)},
            ),
            (
                KeyCode::Char('p'),
                WindowCommand::new_char_command('p', Box::new(create_empty_practice_window)),
            ),
            (
                KeyCode::Char('P'),
                WindowCommand::new_char_command('P', Box::new(create_empty_practice_window)),
            ),
        ]),
    })
}

pub fn practice_window<B: Backend>(state: Rc<State>) -> Box<dyn Fn(&mut Frame<B>)> {
    Box::new(move |f| {
        let spans: Vec<Span> = state.chars.iter().map(|c| c.to_span()).collect();
        let layout = Layout::default()
            .vertical_margin(f.size().height/4)
            .horizontal_margin(f.size().width/3)
            .constraints([Constraint::Percentage(1)].as_ref())
            .split(f.size());
        
        let title = Paragraph::new(vec![Spans::from(spans)])
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: false });

        f.render_widget(title, layout[0]);
    })
}
fn create_empty_practice_window<B: 'static + Backend>(state: &mut State) -> Option<Window<B>> {
    state.index = 0;
    state.error_count = 0;
    state.chars = get_random_practice_text();
    create_practice_window(state)
}
fn get_random_practice_text() -> Vec<ParagraphChar> {
    return convert_string_to_chars("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Suspendisse ornare ipsum sit amet purus tincidunt, sit amet finibus urna tincidunt. Quisque ut neque hendrerit diam pretium porttitor quis ut neque. Fusce facilisis nunc ut aliquet dignissim. Proin sed libero lorem. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Nam viverra purus tellus, vitae sodales magna placerat at. Sed mi est, luctus vel odio sit amet, auctor suscipit diam. Nam quis convallis leo.".to_string());
}
fn create_practice_window<B: 'static + Backend>(_: &mut State) -> Option<Window<B>> {
    fn handle_backspace_press<B: 'static + Backend>(state: &mut State) -> Option<Window<B>>{
        if state.index > 0 {//Going back to the previous inputted char, because the current is not inputted.
            state.index -= 1;
        }
        let current_char = &state.chars[state.index];
        let defaulted_char = match current_char.status {
            CharStatus::Correct => ParagraphChar::new(current_char.character, CharStatus::Default),
            CharStatus::Wrong => {
                state.error_count -= 1;
                ParagraphChar::new(current_char.character, CharStatus::Default)
            },
            CharStatus::Default => ParagraphChar::new(current_char.character, CharStatus::Default)
        };
        state.chars[state.index] = defaulted_char;
        create_practice_window(state)
    }

    let mut commands = HashMap::from([
        (
            KeyCode::Esc,
            WindowCommand {
                activator_key: KeyCode::Esc,
                action: Box::new(create_main_menu_window),
            }
        ),
        (
            KeyCode::Backspace,
            WindowCommand {
                activator_key: KeyCode::Backspace,
                action: Box::new(handle_backspace_press)
            }
        ),
    ]);

    let chars = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'ñ', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
    add_to_commands(&mut commands, &chars);
    
    let upper_chars: Vec<char> = chars.iter().map(|a| a.to_ascii_uppercase()).collect();
    add_to_commands(&mut commands, &upper_chars);

    let punctuation = vec![' ',',','.',':','"','-','@',';','<','>','+','-','_','(',')','=','*','/','¡','!','¿','?','#','$','%','&','°','\'','^','~','[',']','{','}'];
    add_to_commands(&mut commands, &punctuation);

    let numbers = vec!['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'];
    add_to_commands(&mut commands, &numbers);

    let extras = vec!['á', 'Á', 'é', 'É', 'í', 'Í', 'ó', 'Ó', 'ú', 'Ú', 'ä', 'Ä', 'ë', 'Ë', 'ï', 'Ï', 'ö', 'Ö', 'ü', 'Ü', 'ç'];
    add_to_commands(&mut commands, &extras);
    Some(Window {
        ui: practice_window,
        commands
    })
}

fn handle_char_press<B: 'static + Backend>(pressed_character: char) -> Box<dyn Fn(&mut State)->Option<Window<B>>> {
    Box::new(move |state| {
        let end_of_paragraph = state.index == state.chars.len();
        if !end_of_paragraph {
            let current_char = &state.chars[state.index];
            let is_correct = current_char.character == pressed_character;
            let status = if is_correct {CharStatus::Correct} else {CharStatus::Wrong};

            if !is_correct {
                state.error_count += 1;
            }

            let transformed_char = ParagraphChar::new(current_char.character, status);
            state.chars[state.index] = transformed_char;
            
            let done = state.index == state.chars.len() - 1;
            if done && state.error_count == 0 {
                return create_end_window(state);
            }
            state.index += 1;
        }
        return create_practice_window(state);
    })
}

fn add_to_commands<B: 'static + Backend>(commands: &mut HashMap<KeyCode, WindowCommand<B>>, char_array: &Vec<char>) {
    for elem in char_array {
        commands.insert(KeyCode::Char(*elem), WindowCommand{ activator_key: KeyCode::Char(*elem), action: handle_char_press(*elem) });
    }
}

fn end_window<B: Backend>(state: Rc<State>) -> Box<dyn Fn(&mut Frame<B>)> {
    Box::new(
        |f| {
            let paragraph = Paragraph::new("Thank you for playing!").alignment(Alignment::Center);
            f.render_widget(paragraph, f.size());
        }
    )
}

fn create_end_window<B: 'static + Backend>(_: &mut State) -> Option<Window<B>> {
    Some(Window {
        ui: end_window,
        commands: HashMap::from([
            (
                KeyCode::Char('e'),
                WindowCommand::new_char_command('e', Box::new(|_| None)),
            ),
            (
                KeyCode::Esc,
                WindowCommand{ activator_key: KeyCode::Esc, action: Box::new(|_| None)},
            ),
            (
                KeyCode::Char('r'),
                WindowCommand::new_char_command('r', Box::new(create_empty_practice_window)),
            )
        ])
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
