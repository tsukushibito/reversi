use crate::Action;
use crate::Board;
use crate::PlayerColor;
use std::rc::Rc;

struct GameTreeNode<T>
where
    T: Board<T>,
{
    board: Rc<T>,
    player_color: PlayerColor,
    value: i32,
    children: Vec<GameTreeNode<T>>,
}

impl<T> GameTreeNode<T>
where
    T: Board<T>,
{
    pub fn new(board: T, color: PlayerColor) -> GameTreeNode<T> {
        GameTreeNode {
            board: Rc::new(board),
            player_color: color,
            value: 0,
            children: Default::default(),
        }
    }

    pub fn evaluate<F>(&mut self, evaluator: F, depth: usize) -> i32
    where
        F: Fn(&T, &PlayerColor) -> i32,
    {
        if depth == 0 {
            evaluator(&(*self.board), &self.player_color)
        } else {
            let movables = self.board.get_movable_positions(&self.player_color);
            0
        }
    }
}
