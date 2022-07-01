use crate::{Action, GameStateDto};

use super::{AiPlayer, Player};

pub struct GuiPlayer {
    ai: AiPlayer,
    action_func: fn(&GameStateDto) -> Action,
}

impl GuiPlayer {
    pub fn new(action_func: fn(&GameStateDto) -> Action) -> GuiPlayer {
        GuiPlayer {
            ai: AiPlayer::new(4),
            action_func,
        }
    }
}

impl Player for GuiPlayer {
    fn take_action(&mut self, state: &GameStateDto) -> Action {
        let f = self.action_func;
        f(state)
    }
}
