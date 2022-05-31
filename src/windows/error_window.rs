use crate::State;
use crate::Window;
use crate::WindowCommand;
use crossterm::event::KeyCode;
use std::collections::HashMap;
use std::rc::Rc;
use tui::backend::Backend;
use tui::layout::Constraint;
use tui::layout::Layout;
use tui::style::Style;
use tui::widgets::Paragraph;
use tui::Frame;
use tui::{layout::Alignment, layout::Direction, style::Color, text::Span, text::Spans};

fn error_window<B: Backend>(error: String) -> Box<dyn Fn(Rc<State>) -> Box<dyn Fn(&mut Frame<B>)>> {
    Box::new(move |_| {
        let error = error.clone();
        Box::new(move |f| {
            let lines: Vec<Spans> = error
                .split('\n')
                .map(|line| Spans::from(vec![Span::styled(line, Style::default().fg(Color::Red))]))
                .collect();
            let message = Paragraph::new(lines).alignment(Alignment::Center);
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(1)].as_ref())
                .vertical_margin(f.size().height / 2 - 2)
                .split(f.size());
            
            f.render_widget(message, layout[0]);
        })
    })
}

pub fn create_error_window<B: 'static + Backend>(
    error: String, return_window: fn(&mut State) -> Option<Window<B>>,
) -> Option<Window<B>> {
    Some(Window {
        ui: error_window(error),
        commands: HashMap::from([(
            KeyCode::Esc,
            WindowCommand {
                activator_key: KeyCode::Esc,
                action: Box::new(return_window),
            },
        )]),
    })
}
