use crate::{board::Board, PlayerColor};

pub trait EvaluationFunction {
    fn evaluate<B>(&mut self, board: &B, color: &PlayerColor) -> i32
    where
        B: Board;
}
