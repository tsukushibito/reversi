use crate::{game::GameEventParameter, Action};

mod ai_player;
mod console_io_player;
mod gui_player;

pub use ai_player::AiPlayer;
pub use console_io_player::ConsoleIoPlayer;
pub use gui_player::GuiPlayer;

pub trait Player {
    fn take_action(&mut self, state: &GameEventParameter) -> Action;
}
