use crate::Action;
use crate::Board;
use std::rc::Rc;

struct GameTreeNode<T>
where
    T: Board<T>,
{
    board: Rc<T>,
    value: i32,
    children: Vec<GameTreeNode<T>>,
}

impl<T> GameTreeNode<T>
where
    T: Board<T>,
{
    pub fn new(board: T) -> GameTreeNode<T> {
        GameTreeNode {
            board: Rc::new(board),
            value: 0,
            children: Default::default(),
        }
    }

    pub fn evaluate<F>(&mut self, evaluator: F, depth: usize) -> i32
    where
        F: Fn(&T) -> i32,
    {
        if depth == 0 {
            evaluator(&(*self.board))
        } else {
            // let movables = self.board.get_movable_positions();
            0
        }
    }
}

struct GameTree<T, F>
where
    T: Board<T>,
    F: Fn(&T) -> i32,
{
    root: GameTreeNode<T>,
    evaluator: F,
}

impl<T, F> GameTree<T, F>
where
    T: Board<T>,
    F: Fn(&T) -> i32,
{
    pub fn new(board: T, evaluator: F) -> GameTree<T, F> {
        let root = GameTreeNode::new(board);
        GameTree {
            root: root,
            evaluator: evaluator,
        }
    }

    pub fn search_next_move(&mut self, search_depth: usize) -> Action {
        todo!()
    }
}
