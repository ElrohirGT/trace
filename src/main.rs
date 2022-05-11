use std::{io, thread, time::Duration};
use tui::{
    text::{Span, Text, Spans},
    style::{Style, Modifier, Color},
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Alignment},
    widgets::{Widget, Block, Borders, Paragraph},
    Terminal,
    Frame
};
use crossterm::{
    event:: {
        // self,
        DisableMouseCapture,
        EnableMouseCapture,
        Event,
        // KeyCode
    },
    execute,
    terminal::{
        enable_raw_mode,
        disable_raw_mode,
        EnterAlternateScreen,
        LeaveAlternateScreen
    }
};

fn main() -> Result<(), io::Error> {
    //Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(ui)?;

    thread::sleep(Duration::from_millis(5000));

    //Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    Ok(())
}

fn ui<B: Backend>(f: &mut Frame<B>) {
    let game_title = "▀█▀ █▀█ ▄▀█ █▀▀ █▀▀\n░█░ █▀▄ █▀█ █▄▄ ██▄";
    
    let mut form_size = f.size();
    form_size.x = form_size.width / 4;
    form_size.y = form_size.height / 4;
    form_size.width /= 2;
    form_size.height /= 2;

    let main_block = Block::default()
        .borders(Borders::ALL);
    f.render_widget(main_block, form_size);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .vertical_margin(form_size.height/8)
        .horizontal_margin(form_size.width/3)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(1)
        ].as_ref())
        .split(form_size);
    let title = Paragraph::new(Text::styled(game_title, Style::default().add_modifier(Modifier::BOLD).fg(Color::Red)))
        .alignment(Alignment::Center);
    f.render_widget(title, chunks[0]);
    
    let practice_button = create_ui_button("P", "ractice");
    f.render_widget(practice_button, chunks[1]);

    let exit_button = create_ui_button("E", "xit");
    f.render_widget(exit_button, chunks[2])
}

fn create_ui_button<'a>(activator: &'a str, rest: &'a str) -> Paragraph<'a> {
    let button_text = vec![
        Spans::from(vec![
            Span::styled(activator, Style::default().add_modifier(Modifier::UNDERLINED|Modifier::BOLD).fg(Color::Yellow)),
            Span::raw(rest)
        ])
    ];

    Paragraph::new(button_text)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL))
}