use crate::ai::search_game_tree;
use crate::ai::simple_evaluate;
use crate::ai::SearchType;
use crate::board::BitBoard;
use crate::player::Player;
use crate::Action;
use crate::ActionType;
use crate::PlayerColor;
use crate::Squares;

pub struct AiPlayer {
    search_depth: usize,
}

impl AiPlayer {
    pub fn new(search_depth: usize) -> AiPlayer {
        AiPlayer { search_depth }
    }
}

impl Player for AiPlayer {
    fn take_action(&mut self, depth: u32, squares: &Squares) -> Action {
        let board = BitBoard::new(*squares);
        let color = if depth % 2 == 0 {
            PlayerColor::Black
        } else {
            PlayerColor::White
        };

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
