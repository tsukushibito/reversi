use crate::board::Board;
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
}

impl ArrayBoard {
    /// 新規作成
    pub fn new_initial() -> ArrayBoard {
        let mut squares: Squares = [[Square::Empty; BOARD_SIZE]; BOARD_SIZE];
        squares[3][4] = Square::Black;
        squares[4][3] = Square::Black;
        squares[3][3] = Square::White;
        squares[4][4] = Square::White;

        ArrayBoard::new(squares)
    }

    pub fn new(squares: Squares) -> ArrayBoard {
        ArrayBoard { squares }
    }

    fn get_flip_count(&self, color: &PlayerColor, pos: &(usize, usize), dir: &(i32, i32)) -> i32 {
        let color = match color {
            PlayerColor::Black => Square::Black,
            PlayerColor::White => Square::White,
        };
        let mut r = pos.0 as i32 + dir.0;
        let mut c = pos.1 as i32 + dir.1;
        loop {
            if !is_valid_pos((r, c)) {
                break;
            }
            let s = self.squares[r as usize][c as usize];
            if s == color || s == Square::Empty {
                break;
            }
            r += dir.0;
            c += dir.1;
        }

        if is_valid_pos((r, c)) && self.squares[r as usize][c as usize] == color {
            let d = get_distance((pos.0 as i32, pos.1 as i32), (r, c));
            if d >= 2 {
                return d - 1;
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
                    Some(self.clone())
                } else {
                    None
                }
            }
            ActionType::Move(position) => {
                if self.squares[position.0][position.1] != Square::Empty {
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
                    squares[position.0][position.1] = square_color;
                    for dir in DIRECTIONS {
                        let flip =
                            self.get_flip_count(&action.color, &(position.0, position.1), &dir);
                        let mut pos = (position.0 as i32, position.1 as i32);
                        for _ in 0..flip {
                            pos.0 += dir.0;
                            pos.1 += dir.1;
                            squares[pos.0 as usize][pos.1 as usize] = square_color;
                        }
                    }

                    Some(ArrayBoard::new(squares))
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
                if self.squares[row][col] != Square::Empty {
                    continue;
                }
                for dir in DIRECTIONS {
                    let flip = self.get_flip_count(color, &(row, col), &dir);
                    if flip > 0 {
                        positions.push(Position(row, col));
                        break;
                    }
                }
            }
        }
        positions
    }

    fn is_game_over(&self) -> bool {
        self.get_movable_positions(&PlayerColor::Black).len()
            + self.get_movable_positions(&PlayerColor::White).len()
            == 0
    }

    fn square_count(&self, color: Square) -> u32 {
        let mut count = 0;
        for row in &self.squares {
            for s in row {
                if *s == color {
                    count += 1;
                }
            }
        }
        count
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

    fn squares(&self) -> &Squares {
        &self.squares
    }
    fn duplicate(&self) -> ArrayBoard {
        self.clone()
    }
}

fn is_valid_pos(pos: (i32, i32)) -> bool {
    let size = BOARD_SIZE as i32;
    pos.0 >= 0 && pos.0 < size && pos.1 >= 0 && pos.1 < size
}

fn get_distance(p0: (i32, i32), p1: (i32, i32)) -> i32 {
    i32::max((p0.0 - p1.0).abs(), (p0.1 - p1.1).abs())
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
        assert!(next_board.squares[2][3] == Square::Black);
        assert!(next_board.squares[3][3] == Square::Black);
        assert!(next_board.squares[4][3] == Square::Black);
        assert!(next_board.squares[3][4] == Square::Black);
        assert!(next_board.squares[4][4] == Square::White);

        let act = Action::new(PlayerColor::White, ActionType::Move(Position(2, 2)));
        let r = next_board.apply_action(&act);
        assert!(r.is_some());
        let next_board = r.unwrap();
        assert!(next_board.squares[2][3] == Square::Black);
        assert!(next_board.squares[4][3] == Square::Black);
        assert!(next_board.squares[3][4] == Square::Black);
        assert!(next_board.squares[2][2] == Square::White);
        assert!(next_board.squares[3][3] == Square::White);
        assert!(next_board.squares[4][4] == Square::White);
    }
}
