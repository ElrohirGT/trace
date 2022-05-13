use crossterm::{
    event::{
        read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyModifiers,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::collections::HashMap;
use std::io;
use trace::windows::{main_menu_window, practice_window, Window, WindowCommand};
use tui::backend::Backend;
use tui::{backend::CrosstermBackend, Terminal};

fn main() -> Result<(), io::Error> {
    //Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut window = Some(Window {
        ui: main_menu_window,
        commands: HashMap::from([
            (
                KeyCode::Char('e'),
                WindowCommand::new_char_command('e', || None),
            ),
            (
                KeyCode::Char('p'),
                WindowCommand::new_char_command('p', create_practice_window),
            ),
        ]),
    });
    loop {
        window = match window {
            Some(ref current_window) => {
                terminal.draw(&current_window.ui)?;
                let user_input = read()?;
                match user_input {
                    Event::Key(event) => match current_window.commands.get(&event.code) {
                        None => window,
                        Some(command) => match (command.action)() {
                            Some(new_window) => Some(new_window),
                            None => None,
                        },
                    },
                    Event::Mouse(_) => window,
                    Event::Resize(_, _) => window,
                }
            }
            None => None,
        };
        if let None = window {
            break;
        }
    }
    // thread::sleep(Duration::from_millis(5000));
    //Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

fn create_practice_window<B: Backend>() -> Option<Window<B>> {
    Some(Window {
        ui: practice_window,
        commands: HashMap::new(),
    })
}
