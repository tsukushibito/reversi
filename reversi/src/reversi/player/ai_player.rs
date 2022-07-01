use crate::ai::search_game_tree;
use crate::ai::simple_evaluate;
use crate::ai::SearchType;
use crate::board::BitBoard;
use crate::game::GameEventParameter;
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
    fn take_action(&mut self, state: &GameEventParameter) -> Action {
        let board = BitBoard::new(state.board, state.depth, state.last_action);
        let color = state.turn;
        let result = search_game_tree(
            &board,
            &color,
            &simple_evaluate,
            &SearchType::NegaAlpha,
            self.search_depth,
        );

        result
            .action
            .unwrap_or_else(|| Action::new(color, ActionType::Pass))
    }
}
