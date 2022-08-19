use crate::board::Board;
use crate::index_to_position;
use crate::position_to_index;
use crate::Action;
use crate::ActionType;
use crate::PlayerColor;
use crate::Position;
use crate::Square;
use crate::Squares;
use crate::BOARD_SIZE;

fn data_to_positions(data: u64) -> Vec<Position> {
    let mut positions: Vec<Position> = Default::default();
    let bytes = data.to_le_bytes();
    for (r, byte) in bytes.iter().enumerate() {
        for c in 0..BOARD_SIZE {
            if byte & (1 << c) != 0 {
                positions.push(Position(r, c));
            }
        }
    }
    positions
}

fn position_to_data(position: &Position) -> u64 {
    1 << (position.0 * BOARD_SIZE + position.1)
}

fn data_to_squares(black: u64, white: u64) -> Squares {
    let mut squares = [Square::Empty; BOARD_SIZE * BOARD_SIZE];

    let black_bytes = black.to_le_bytes();
    for (r, byte) in black_bytes.iter().enumerate() {
        for c in 0..BOARD_SIZE {
            if byte & (1 << c) != 0 {
                let i = position_to_index(&Position(r, c));
                squares[i] = Square::Black;
            }
        }
    }

    let white_bytes = white.to_le_bytes();
    for (r, byte) in white_bytes.iter().enumerate() {
        for c in 0..BOARD_SIZE {
            if byte & (1 << c) != 0 {
                let i = position_to_index(&Position(r, c));
                squares[i] = Square::White;
            }
        }
    }

    squares
}

fn squares_to_data(squares: &Squares) -> (u64, u64) {
    let mut black_data: u64 = 0;
    let mut white_data: u64 = 0;
    for (i, s) in squares.iter().enumerate() {
        let p = index_to_position(i);
        match s {
            Square::Black => black_data |= position_to_data(&p),
            Square::White => white_data |= position_to_data(&p),
            _ => (),
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
    fn dir_continuous_line(data: u64, opponent: u64, dir_mask: u64, shift_count: u32) -> u64 {
        let mask = opponent & dir_mask;
        let mut line1 = continuous_line::<LeftShift>(data, mask, shift_count);
        line1 |= LeftShift::shift(line1, shift_count);
        let mut line2 = continuous_line::<RightShift>(data, mask, shift_count);
        line2 |= RightShift::shift(line2, shift_count);
        line1 | line2
    }

    let horizontal = dir_continuous_line(player, opponent, HORIZONTAL_MASK, 1);
    let vertical = dir_continuous_line(player, opponent, VERTICAL_MASK, 8);
    let diagonal_ltrb = dir_continuous_line(player, opponent, DIAGONAL_MASK, 9);
    let diagonal_rtlb = dir_continuous_line(player, opponent, DIAGONAL_MASK, 7);

    (horizontal | vertical | diagonal_ltrb | diagonal_rtlb) & !(player | opponent)
}

fn flip_data(player: u64, opponent: u64, position: u64) -> u64 {
    fn dir_flip(player: u64, opponent: u64, position: u64, dir_mask: u64, shift_count: u32) -> u64 {
        let mut result: u64 = 0;
        let mask = opponent & dir_mask;

        let line1 = continuous_line::<LeftShift>(position, mask, shift_count);
        let temp = player & LeftShift::shift(line1, shift_count);
        if temp != 0 {
            result |= line1;
        }

        let line2 = continuous_line::<RightShift>(position, mask, shift_count);
        let temp = player & RightShift::shift(line2, shift_count);
        if temp != 0 {
            result |= line2;
        }

        result
    }

    let horizontal = dir_flip(player, opponent, position, HORIZONTAL_MASK, 1);
    let vertical = dir_flip(player, opponent, position, VERTICAL_MASK, 8);
    let diagonal_ltrb = dir_flip(player, opponent, position, DIAGONAL_MASK, 9);
    let diagonal_rtlb = dir_flip(player, opponent, position, DIAGONAL_MASK, 7);

    horizontal | vertical | diagonal_ltrb | diagonal_rtlb
}

fn flip(player: u64, opponent: u64, position: u64) -> (u64, u64) {
    let flip_data = flip_data(player, opponent, position);
    (player ^ position ^ flip_data, opponent ^ flip_data)
}

/// ボード
#[derive(Clone, Debug)]
pub struct BitBoard {
    black: u64,
    white: u64,
    squares: Squares,
    depth: u32,
}

impl BitBoard {
    /// Creates a new [`BitBoard`].
    pub fn new(squares: &Squares, depth: u32) -> Self {
        let (black, white) = squares_to_data(squares);
        let mut ss: Squares = [Square::Empty; BOARD_SIZE * BOARD_SIZE];
        ss.clone_from_slice(squares);
        Self {
            black,
            white,
            squares: ss,
            depth,
        }
    }

    pub fn new_from_data(black: u64, white: u64, depth: u32) -> Self {
        let squares = data_to_squares(black, white);
        Self {
            black,
            white,
            squares,
            depth,
        }
    }

    pub fn new_initial() -> Self {
        let mut squares: Squares = [Square::Empty; BOARD_SIZE * BOARD_SIZE];
        squares[position_to_index(&Position(3, 4))] = Square::Black;
        squares[position_to_index(&Position(4, 3))] = Square::Black;
        squares[position_to_index(&Position(3, 3))] = Square::White;
        squares[position_to_index(&Position(4, 4))] = Square::White;

        Self::new(&squares, 0)
    }
}

impl Board for BitBoard {
    fn apply_action(&self, action: &Action) -> Option<Self>
    where
        Self: Sized,
    {
        match action.action {
            ActionType::Pass => {
                if self.get_movable_positions(&action.color).is_empty() {
                    Some(BitBoard::new_from_data(
                        self.black,
                        self.white,
                        self.depth + 1,
                    ))
                } else {
                    None
                }
            }
            ActionType::Move(position) => {
                let (player, opponent) = if action.color == PlayerColor::Black {
                    (self.black, self.white)
                } else {
                    (self.white, self.black)
                };

                let movable = movable_position(player, opponent);
                let pos = position_to_data(&position);

                if pos & movable == 0 {
                    return None;
                }

                let (next_player, next_opponent) = flip(player, opponent, pos);
                let (next_black, next_white) = if action.color == PlayerColor::Black {
                    (next_player, next_opponent)
                } else {
                    (next_opponent, next_player)
                };
                Some(BitBoard::new_from_data(
                    next_black,
                    next_white,
                    self.depth + 1,
                ))
            }
        }
    }

    fn get_movable_positions(&self, color: &PlayerColor) -> Vec<Position> {
        let (player, opponent) = if *color == PlayerColor::Black {
            (self.black, self.white)
        } else {
            (self.white, self.black)
        };

        let movable = movable_position(player, opponent);
        data_to_positions(movable)
    }

    fn square_count(&self, color: Square) -> u32 {
        match color {
            Square::Black => self.black.count_ones(),
            Square::White => self.white.count_ones(),
            Square::Empty => (self.black | self.white).count_zeros(),
        }
    }

    fn squares(&self) -> &Squares {
        &self.squares
    }

    fn duplicate(&self) -> Self {
        self.clone()
    }

    fn depth(&self) -> u32 {
        self.depth
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_to_squares() {
        let mut black = position_to_data(&Position(0, 0));
        black |= position_to_data(&Position(0, 1));

        let mut white = position_to_data(&Position(1, 0));
        white |= position_to_data(&Position(1, 1));

        let squares = data_to_squares(black, white);
        assert!(squares[position_to_index(&Position(0, 0))] == Square::Black);
        assert!(squares[position_to_index(&Position(0, 1))] == Square::Black);
        assert!(squares[position_to_index(&Position(1, 0))] == Square::White);
        assert!(squares[position_to_index(&Position(1, 1))] == Square::White);
    }

    #[test]
    fn test_squares_to_data() {
        let mut black = position_to_data(&Position(0, 0));
        black |= position_to_data(&Position(0, 1));

        let mut white = position_to_data(&Position(1, 0));
        white |= position_to_data(&Position(1, 1));

        let squares = data_to_squares(black, white);
        let (black_data, white_data) = squares_to_data(&squares);
        assert_eq!(black_data, black);
        assert_eq!(white_data, white);
    }

    #[test]
    fn test_movable_position() {
        let mut squares: Squares = [Square::Empty; BOARD_SIZE * BOARD_SIZE];
        squares[position_to_index(&Position(3, 4))] = Square::Black;
        squares[position_to_index(&Position(4, 3))] = Square::Black;
        squares[position_to_index(&Position(3, 3))] = Square::White;
        squares[position_to_index(&Position(4, 4))] = Square::White;

        let (black, white) = squares_to_data(&squares);
        let pos = movable_position(black, white);
        let temp = data_to_squares(pos, 0);
        assert!(temp[position_to_index(&Position(2, 3))] == Square::Black);
        assert!(temp[position_to_index(&Position(3, 2))] == Square::Black);
        assert!(temp[position_to_index(&Position(4, 5))] == Square::Black);
        assert!(temp[position_to_index(&Position(5, 4))] == Square::Black);

        #[allow(clippy::needless_range_loop)]
        for r in 0..BOARD_SIZE {
            for c in 0..BOARD_SIZE {
                if r == 2 && c == 3 || r == 3 && c == 2 || r == 4 && c == 5 || r == 5 && c == 4 {
                    continue;
                }
                assert!(temp[position_to_index(&Position(r, c))] == Square::Empty);
            }
        }
    }

    #[test]
    fn test_apply_action() {
        let board = BitBoard::new_initial();

        let act = Action::new(PlayerColor::Black, ActionType::Move(Position(0, 0)));
        let r = board.apply_action(&act);
        assert!(r.is_none());

        let act = Action::new(PlayerColor::Black, ActionType::Move(Position(2, 3)));
        let r = board.apply_action(&act);
        assert!(r.is_some());
        let next_board = r.unwrap();
        assert!(next_board.squares[position_to_index(&Position(2, 3))] == Square::Black);
        assert!(next_board.squares[position_to_index(&Position(3, 3))] == Square::Black);
        assert!(next_board.squares[position_to_index(&Position(4, 3))] == Square::Black);
        assert!(next_board.squares[position_to_index(&Position(3, 4))] == Square::Black);
        assert!(next_board.squares[position_to_index(&Position(4, 4))] == Square::White);

        let act = Action::new(PlayerColor::White, ActionType::Move(Position(2, 2)));
        let r = next_board.apply_action(&act);
        assert!(r.is_some());
        let next_board = r.unwrap();
        assert!(next_board.squares[position_to_index(&Position(2, 3))] == Square::Black);
        assert!(next_board.squares[position_to_index(&Position(4, 3))] == Square::Black);
        assert!(next_board.squares[position_to_index(&Position(3, 4))] == Square::Black);
        assert!(next_board.squares[position_to_index(&Position(2, 2))] == Square::White);
        assert!(next_board.squares[position_to_index(&Position(3, 3))] == Square::White);
        assert!(next_board.squares[position_to_index(&Position(4, 4))] == Square::White);
    }

    #[test]
    fn test_square_count() {
        let board = BitBoard::new_initial();

        assert_eq!(board.black_count(), 2);
        assert_eq!(board.white_count(), 2);
        assert_eq!(board.empty_count(), 60);
    }
}
