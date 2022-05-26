use crate::windows;
use crate::State;
use crate::Window;
use tui::backend::Backend;

pub fn create_mp_create_server_window<B: 'static + Backend>(_: &mut State) -> Option<Window<B>> {
    None
}
