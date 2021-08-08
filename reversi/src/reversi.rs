use crate::board::Board;
use crate::player::Player;

pub struct Reversi<T>
where
    T: Player,
{
    board: Board,
    black_player: T,
    white_player: T,
    depth: u32,
}
