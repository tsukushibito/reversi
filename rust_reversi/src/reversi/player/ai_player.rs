use crate::ai::search_game_tree;
use crate::ai::simple_evaluate;
use crate::ai::SearchType;
use crate::board::IndexBoard;
use crate::board::Indexer;
use crate::player::Player;
use crate::Action;
use crate::ActionType;
use crate::PlayerColor;
use crate::Squares;
use std::rc::Rc;

pub struct AiPlayer {
    indexer: Rc<Indexer>,
    search_depth: usize,
}

impl AiPlayer {
    pub fn new(search_depth: usize) -> AiPlayer {
        let indexer = Rc::new(Indexer::new());
        AiPlayer {
            indexer: indexer,
            search_depth: search_depth,
        }
    }
}

impl Player for AiPlayer {
    fn take_action(&mut self, depth: u32, squares: &Squares) -> Action {
        let board = IndexBoard::new(*squares, self.indexer.clone());
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
            .unwrap_or(Action::new(color, ActionType::Pass))
    }
}
