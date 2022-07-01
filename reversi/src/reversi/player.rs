use crate::*;

mod ai_player;
mod console_io_player;

pub use ai_player::AiPlayer;
pub use console_io_player::ConsoleIoPlayer;

pub trait Player {
    fn take_action(&mut self, state: &GameStateDto) -> Action;
}
