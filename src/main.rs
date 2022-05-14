use crossterm::{
    event::{read, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use trace::windows::create_main_menu_window;
use tui::{backend::CrosstermBackend, Terminal};

fn main() -> Result<(), io::Error> {
    //Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut window = create_main_menu_window();
    loop {
        window = match window {
            None => break,
            Some(ref current_window) => {
                terminal.draw(&current_window.ui)?;
                let user_input = read()?;
                match user_input {
                    Event::Key(event) => match current_window.commands.get(&event.code) {
                        None => window,
                        Some(command) => (command.action)()
                    },
                    Event::Mouse(_) => window,
                    Event::Resize(_, _) => window,
                }
            }
        };
    }
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
