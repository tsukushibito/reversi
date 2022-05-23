use crate::Board;
use std::rc::Rc;

struct GameTreeNode<T>
where
    T: Board<T>,
{
    board: Rc<T>,
}

impl<T> GameTreeNode<T>
where
    T: Board<T>,
{
    pub fn new(initial_board: T) -> GameTreeNode<T> {
        GameTreeNode {
            board: Rc::new(initial_board),
        }
    }

    fn evaluate(&self) -> i32 {
        0
    }

    fn next_child_node(&self) -> &GameTreeNode<T> {
        todo!()
    }
}

struct GameTree<T>
where
    T: Board<T>,
{
    root: GameTreeNode<T>,
}
