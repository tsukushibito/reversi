use crate::Squares;

use super::Board;

/// ボード
#[derive(Clone, Debug)]
pub struct BitBoard {
    black: u64,
    white: u64,
}

fn continuous_line<T>(position: u64, opponent: u64, mask: u64, shift: &T) -> u64
where
    T: Fn(u64) -> u64,
{
    let mask = opponent & mask;
    let mut result = mask & shift(position);
    result |= mask & shift(result);
    result |= mask & shift(result);
    result |= mask & shift(result);
    result |= mask & shift(result);
    result |= mask & shift(result);
    result
}

impl Board for BitBoard {
    fn apply_action(&self, action: &crate::Action) -> Option<Self>
    where
        Self: Sized,
    {
        todo!()
    }

    fn get_movable_positions(&self, color: &crate::PlayerColor) -> Vec<crate::Position> {
        todo!()
    }

    fn is_game_over(&self) -> bool {
        todo!()
    }

    fn square_count(&self, color: crate::Square) -> u32 {
        todo!()
    }

    fn black_count(&self) -> u32 {
        todo!()
    }

    fn white_count(&self) -> u32 {
        todo!()
    }

    fn empty_count(&self) -> u32 {
        todo!()
    }

    fn squares(&self) -> &Squares {
        todo!()
    }

    fn duplicate(&self) -> Self {
        todo!()
    }
}
