use crate::windows;
use crate::Window;
use crate::State;
use tui::backend::Backend;

pub fn create_mp_create_server_window<B: 'static + Backend>(_: &mut State) -> Option<Window<B>> {
    None
}