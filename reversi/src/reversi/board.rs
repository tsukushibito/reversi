use crate::reversi::common::*;

mod array_board;
mod bit_board;
mod index_board;
mod indexer;

pub use array_board::ArrayBoard;
pub use bit_board::BitBoard;
pub use index_board::IndexBoard;
pub use indexer::Indexer;

pub trait Board {
    fn squares(&self) -> &Squares;
    fn depth(&self) -> u32;
    fn square_count(&self, color: Square) -> u32;
    fn duplicate(&self) -> Self;

    fn apply_action(&self, action: &Action) -> Option<Self>
    where
        Self: Sized;
    fn get_movable_positions(&self, color: &PlayerColor) -> Vec<Position>;

    fn is_game_over(&self) -> bool {
        self.get_movable_positions(&PlayerColor::Black).is_empty()
            && self.get_movable_positions(&PlayerColor::White).is_empty()
    }

    fn black_count(&self) -> u32 {
        self.square_count(Square::Black)
    }

    fn white_count(&self) -> u32 {
        self.square_count(Square::White)
    }

    fn empty_count(&self) -> u32 {
        self.square_count(Square::Empty)
    }

    fn turn(&self) -> PlayerColor {
        if self.depth() % 2 == 0 {
            PlayerColor::Black
        } else {
            PlayerColor::White
        }
    }
}
