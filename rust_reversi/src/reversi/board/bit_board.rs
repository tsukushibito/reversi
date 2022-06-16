use crate::Squares;

use super::Board;

/// ボード
#[derive(Clone, Debug)]
pub struct BitBoard {
    black: u64,
    white: u64,
}

const HORIZONTAL_MASK: u64 = 0x7e7e7e7e7e7e7e7e;
const VERTICAL_MASK: u64 = 0x00ffffffffffff00;
const DIAGONAL_MASK: u64 = 0x007e7e7e7e7e7e00;

fn left_shift(value: u64, shift_count: u32) -> u64 {
    value << shift_count
}

fn right_shift(value: u64, shift_count: u32) -> u64 {
    value >> shift_count
}

fn continuous_line(position: u64, mask: u64, shift: fn(u64, u32) -> u64, shift_count: u32) -> u64 {
    let mut result = mask & shift(position, shift_count);
    result |= mask & shift(result, shift_count);
    result |= mask & shift(result, shift_count);
    result |= mask & shift(result, shift_count);
    result |= mask & shift(result, shift_count);
    result |= mask & shift(result, shift_count);
    result
}

fn horizontal_continuous_line(position: u64, opponent: u64) -> u64 {
    let mask = opponent & HORIZONTAL_MASK;
    let mut r = continuous_line(position, mask, left_shift, 1);
    r = left_shift(r, 1);
    r
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
