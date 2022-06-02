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

        let mut root = GameTreeNode::new(board, color, None);
        let mut visited_count: usize = 0;
        let (_, act) = root.search(&simple_evaluator, self.search_depth, &mut visited_count);
        act.unwrap_or(Action::new(color, ActionType::Pass))
    }
}
