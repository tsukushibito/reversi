use crate::action::Action;
use crate::board::Squares;

pub trait Player {
    fn take_action(squares: Squares) -> Action;
}
