use crate::action::Action;
use crate::board::Squares;

pub trait Player {
    fn take_action(&mut self, depth: u32, squares: Squares) -> Action;
}
