use crate::ai::SimpleEvaluator;
use crate::board::BitBoard;
use crate::board::Board;
use crate::game::GameState;
use crate::player::Player;
use crate::Action;
use crate::ActionType;

pub struct AiPlayer {
    search_depth: usize,
}

impl AiPlayer {
    pub fn new(search_depth: usize) -> AiPlayer {
        AiPlayer { search_depth }
    }
}

impl Player for AiPlayer {
    fn take_action(&self, state: &GameState) -> Action {
        todo!()
    }
}
