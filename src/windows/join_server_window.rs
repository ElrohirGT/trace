use crate::windows::*;
use crate::State;
use crate::Window;
use crate::WindowCommand;
use crossterm::event::KeyCode;
use std::collections::HashMap;
use std::rc::Rc;
use tui::backend::Backend;
use tui::Frame;

fn mp_join_server_window<B: 'static + Backend>(_: Rc<State>) -> Box<dyn Fn(&mut Frame<B>)> {
    Box::new(|f| {})
}

pub fn create_mp_join_server_window<B: 'static + Backend>(_: &mut State) -> Option<Window<B>> {
    Some(Window {
        ui: Box::new(mp_join_server_window),
        commands: HashMap::from([(
            KeyCode::Esc,
            WindowCommand {
                activator_key: KeyCode::Esc,
                action: Box::new(create_multiplayer_menu_window),
            },
        )]),
    })
}
