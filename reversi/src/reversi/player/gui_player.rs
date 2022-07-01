use crate::{game::GameEventParameter, Action};

use super::{AiPlayer, Player};

pub struct GuiPlayer {
    ai: AiPlayer,
    action_func: fn(&GameEventParameter) -> Action,
}

impl GuiPlayer {
    pub fn new(action_func: fn(&GameEventParameter) -> Action) -> GuiPlayer {
        GuiPlayer {
            ai: AiPlayer::new(4),
            action_func,
        }
    }
}

impl Player for GuiPlayer {
    fn take_action(&mut self, state: &GameEventParameter) -> Action {
        let f = self.action_func;
        f(state)
    }
}
