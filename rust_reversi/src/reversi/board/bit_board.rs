use crate::board::Board;
use crate::Position;
use crate::Square;
use crate::Squares;
use crate::BOARD_SIZE;

const fn position_to_data(position: Position) -> u64 {
    1 << (position.0 * BOARD_SIZE + position.1)
}

fn data_to_squares(black: u64, white: u64) -> Squares {
    let mut squares = [[Square::Empty; BOARD_SIZE]; BOARD_SIZE];

    let black_bytes = black.to_le_bytes();
    for (r, byte) in black_bytes.iter().enumerate() {
        for c in 0..BOARD_SIZE {
            if byte & (1 << c) != 0 {
                squares[r][c] = Square::Black;
            }
        }
    }

    let white_bytes = white.to_le_bytes();
    for (r, byte) in white_bytes.iter().enumerate() {
        for c in 0..BOARD_SIZE {
            if byte & (1 << c) != 0 {
                squares[r][c] = Square::White;
            }
        }
    }

    squares
}

fn squares_to_data(squares: &Squares) -> (u64, u64) {
    let mut black_data: u64 = 0;
    let mut white_data: u64 = 0;
    for (r, row) in squares.iter().enumerate() {
        for (c, s) in row.iter().enumerate() {
            match s {
                Square::Black => black_data |= position_to_data(Position(r, c)),
                Square::White => white_data |= position_to_data(Position(r, c)),
                _ => (),
            }
        }
    }

    (black_data, white_data)
}

const HORIZONTAL_MASK: u64 = 0x7e7e7e7e7e7e7e7e;
const VERTICAL_MASK: u64 = 0x00ffffffffffff00;
const DIAGONAL_MASK: u64 = 0x007e7e7e7e7e7e00;

trait BoardShift {
    fn shift(value: u64, shift_count: u32) -> u64;
}

struct LeftShift;
impl BoardShift for LeftShift {
    fn shift(value: u64, shift_count: u32) -> u64 {
        value << shift_count
    }
}

struct RightShift;
impl BoardShift for RightShift {
    fn shift(value: u64, shift_count: u32) -> u64 {
        value >> shift_count
    }
}

fn continuous_line<T>(data: u64, mask: u64, shift_count: u32) -> u64
where
    T: BoardShift,
{
    let mut result = mask & T::shift(data, shift_count);
    result |= mask & T::shift(result, shift_count);
    result |= mask & T::shift(result, shift_count);
    result |= mask & T::shift(result, shift_count);
    result |= mask & T::shift(result, shift_count);
    result |= mask & T::shift(result, shift_count);
    result
}

fn movable_position(player: u64, opponent: u64) -> u64 {
    fn dir_continuous_line(position: u64, opponent: u64, dir_mask: u64, shift_count: u32) -> u64 {
        let mask = opponent & dir_mask;
        let mut line1 = continuous_line::<LeftShift>(position, mask, shift_count);
        line1 = LeftShift::shift(line1, shift_count);
        let mut line2 = continuous_line::<RightShift>(position, mask, shift_count);
        line2 = RightShift::shift(line2, shift_count);
        line1 | line2
    }

    let horizontal = dir_continuous_line(player, opponent, HORIZONTAL_MASK, 1);
    let vertical = dir_continuous_line(player, opponent, VERTICAL_MASK, 8);
    let diagonal_ltrb = dir_continuous_line(player, opponent, DIAGONAL_MASK, 9);
    let diagonal_rtlb = dir_continuous_line(player, opponent, DIAGONAL_MASK, 7);

    (horizontal | vertical | diagonal_ltrb | diagonal_rtlb) & !(player | opponent)
}
/// ボード
#[derive(Clone, Debug)]
pub struct BitBoard {
    black: u64,
    white: u64,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_to_squares() {
        let mut black = position_to_data(Position(0, 0));
        black |= position_to_data(Position(0, 1));

        let mut white = position_to_data(Position(1, 0));
        white |= position_to_data(Position(1, 1));

        let squares = data_to_squares(black, white);
        assert!(squares[0][0] == Square::Black);
        assert!(squares[0][1] == Square::Black);
        assert!(squares[1][0] == Square::White);
        assert!(squares[1][1] == Square::White);
    }

    #[test]
    fn test_squares_to_data() {
        let mut black = position_to_data(Position(0, 0));
        black |= position_to_data(Position(0, 1));

        let mut white = position_to_data(Position(1, 0));
        white |= position_to_data(Position(1, 1));

        let squares = data_to_squares(black, white);
        let (black_data, white_data) = squares_to_data(&squares);
        assert_eq!(black_data, black);
        assert_eq!(white_data, white);
    }

    #[test]
    fn test_movable_position() {
        let mut squares: Squares = [[Square::Empty; BOARD_SIZE]; BOARD_SIZE];
        squares[3][4] = Square::Black;
        squares[4][3] = Square::Black;
        squares[3][3] = Square::White;
        squares[4][4] = Square::White;

        let (black, white) = squares_to_data(&squares);
        let pos = movable_position(black, white);
        let temp = data_to_squares(pos, 0);
        assert!(temp[2][3] == Square::Black);
        assert!(temp[3][2] == Square::Black);
        assert!(temp[4][5] == Square::Black);
        assert!(temp[5][4] == Square::Black);
        for r in 0..BOARD_SIZE {
            for c in 0..BOARD_SIZE {
                if r == 2 && c == 3 || r == 3 && c == 2 || r == 4 && c == 5 || r == 5 && c == 4 {
                    continue;
                }
                assert!(temp[r][c] == Square::Empty);
            }
        }
    }
}
