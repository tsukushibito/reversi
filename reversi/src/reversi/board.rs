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
    fn apply_action(&self, action: &Action) -> Option<Self>
    where
        Self: Sized;
    fn get_movable_positions(&self, color: &PlayerColor) -> Vec<Position>;
    fn is_game_over(&self) -> bool;
    fn square_count(&self, color: Square) -> u32;
    fn black_count(&self) -> u32;
    fn white_count(&self) -> u32;
    fn empty_count(&self) -> u32;
    fn squares(&self) -> &Squares;
    fn duplicate(&self) -> Self;
}
