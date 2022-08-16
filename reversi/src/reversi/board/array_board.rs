use crate::board::Board;
use crate::position_to_index;
use crate::Action;
use crate::ActionType;
use crate::PlayerColor;
use crate::Position;
use crate::Square;
use crate::Squares;
use crate::BOARD_SIZE;

const DIRECTIONS: [(i32, i32); BOARD_SIZE] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

/// ボード
#[derive(Clone, Debug)]
pub struct ArrayBoard {
    squares: Squares,
    depth: u32,
}

impl ArrayBoard {
    /// 新規作成
    pub fn new_initial() -> ArrayBoard {
        let mut squares: Squares = [Square::Empty; BOARD_SIZE * BOARD_SIZE];
        squares[position_to_index(&Position(3, 4))] = Square::Black;
        squares[position_to_index(&Position(4, 3))] = Square::Black;
        squares[position_to_index(&Position(3, 3))] = Square::White;
        squares[position_to_index(&Position(4, 4))] = Square::White;

        ArrayBoard::new(squares, 0)
    }

    pub fn new(squares: Squares, depth: u32) -> ArrayBoard {
        ArrayBoard { squares, depth }
    }

    fn get_flip_count(&self, color: &PlayerColor, pos: &Position, dir: &(i32, i32)) -> i32 {
        let color = match color {
            PlayerColor::Black => Square::Black,
            PlayerColor::White => Square::White,
        };
        let mut r = pos.0 as i32 + dir.0;
        let mut c = pos.1 as i32 + dir.1;
        loop {
            let target_pos = Position(r as usize, c as usize);
            if !is_valid_pos(&target_pos) {
                break;
            }
            let index = position_to_index(&target_pos);
            let s = self.squares[index];
            if s == color || s == Square::Empty {
                break;
            }
            r += dir.0;
            c += dir.1;
        }

        let target_pos = Position(r as usize, c as usize);
        if is_valid_pos(&target_pos) && self.squares[position_to_index(&target_pos)] == color {
            let d = get_distance(pos, &target_pos);
            if d >= 2 {
                return d as i32 - 1;
            }
        }

        0
    }
}

impl Board for ArrayBoard {
    fn apply_action(&self, action: &Action) -> Option<ArrayBoard> {
        match action.action {
            ActionType::Pass => {
                // パスできるかチェック
                let movables = self.get_movable_positions(&action.color);
                if movables.is_empty() {
                    Some(ArrayBoard::new(self.squares, self.depth + 1))
                } else {
                    None
                }
            }
            ActionType::Move(position) => {
                let index = position_to_index(&position);
                if self.squares[index] != Square::Empty {
                    // 空きマス以外には石を置けない
                    return None;
                }
                let movables = self.get_movable_positions(&action.color);
                if movables
                    .iter()
                    .any(|p| p.0 == position.0 && p.1 == position.1)
                {
                    let mut squares = self.squares;

                    let square_color = match action.color {
                        PlayerColor::Black => Square::Black,
                        PlayerColor::White => Square::White,
                    };

                    // アクションの箇所に石を置く
                    squares[index] = square_color;
                    for dir in DIRECTIONS {
                        let flip = self.get_flip_count(&action.color, &position, &dir);
                        let mut pos = (position.0 as i32, position.1 as i32);
                        for _ in 0..flip {
                            pos.0 += dir.0;
                            pos.1 += dir.1;
                            let i = position_to_index(&Position(pos.0 as usize, pos.1 as usize));
                            squares[i] = square_color;
                        }
                    }

                    Some(ArrayBoard::new(squares, self.depth + 1))
                } else {
                    None
                }
            }
        }
    }

    fn get_movable_positions(&self, color: &PlayerColor) -> Vec<Position> {
        let mut positions = Vec::new();
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                let pos = Position(row, col);
                let index = position_to_index(&pos);
                if self.squares[index] != Square::Empty {
                    continue;
                }
                for dir in DIRECTIONS {
                    let flip = self.get_flip_count(color, &pos, &dir);
                    if flip > 0 {
                        positions.push(pos);
                        break;
                    }
                }
            }
        }
        positions
    }

    fn square_count(&self, color: Square) -> u32 {
        let mut count = 0;
        for s in &self.squares {
            if *s == color {
                count += 1;
            }
        }
        count
    }

    fn squares(&self) -> &Squares {
        &self.squares
    }

    fn duplicate(&self) -> ArrayBoard {
        self.clone()
    }

    fn depth(&self) -> u32 {
        self.depth
    }
}

fn is_valid_pos(pos: &Position) -> bool {
    let size = BOARD_SIZE;
    pos.0 < size && pos.1 < size
}

fn get_distance(p0: &Position, p1: &Position) -> u32 {
    u32::max(
        (p0.0 as i32 - p1.0 as i32).unsigned_abs(),
        (p0.1 as i32 - p1.1 as i32).unsigned_abs(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_action() {
        let board = ArrayBoard::new_initial();

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
}
