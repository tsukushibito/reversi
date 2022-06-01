use crate::evaluator::simple_evaluator;
use crate::game_tree::GameTreeNode;
use crate::index_board::IndexBoard;
use crate::indexer::Indexer;
use crate::player::Player;
use crate::Action;
use crate::ActionType;
use crate::PlayerColor;
use crate::Squares;
use std::rc::Rc;

pub struct AiPlayer {
    indexer: Rc<Indexer>,
}

impl AiPlayer {
    pub fn new() -> AiPlayer {
        let indexer = Rc::new(Indexer::new());
        AiPlayer { indexer: indexer }
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

        let mut root = GameTreeNode::new(board, color, None);
        let (_, act) = root.evaluate(&simple_evaluator, 5);
        act.unwrap_or(Action::new(color, ActionType::Pass))
    }
}
