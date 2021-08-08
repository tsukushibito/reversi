use crate::action::Action;
use crate::indexer::Indexer;

pub const BOARD_SIZE: usize = 8;
pub type Squares = [[Square; BOARD_SIZE]; BOARD_SIZE];

pub enum LineDirection {
    Left2Right,
    Top2Bottom,
    TopLeft2BottomRight,
    BottomLeft2TopRight,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Square {
    Empty = 1,
    Black = 2,
    White = 0,
}

pub struct Board {
    pub squares: Squares,
    indexer: Indexer,
}

impl Board {
    pub fn new() -> Board {
        let mut squares = [[Square::Empty; BOARD_SIZE]; BOARD_SIZE];
        squares[3][4] = Square::Black;
        squares[4][3] = Square::Black;
        squares[3][3] = Square::White;
        squares[4][4] = Square::White;

        let indexer = Indexer::new();
        Board {
            squares: squares,
            indexer: indexer,
        }
    }

    pub fn apply_action(&mut self, action: &Action) -> bool {
        // 左右方向の情報
        let l2r = self.get_line(action.row, action.col, LineDirection::Left2Right);
        let l2r_finfo = self.indexer.get_flip_info(action.color, &l2r, action.col);

        // 上下方向の情報
        let t2b = self.get_line(action.row, action.col, LineDirection::Top2Bottom);
        let t2b_finfo = self.indexer.get_flip_info(action.color, &t2b, action.row);

        let row = action.row as i32;
        let col = action.col as i32;
        // 左上から右下方向の情報
        let tl2br = self.get_line(action.row, action.col, LineDirection::TopLeft2BottomRight);
        let pos = if row - col >= 0 {
            action.col
        } else {
            action.row
        };
        let tl2br_finfo = self.indexer.get_flip_info(action.color, &tl2br, pos);

        // 左下から右上方向の情報
        let bl2tr = self.get_line(action.row, action.col, LineDirection::BottomLeft2TopRight);
        let pos = if row + col - BOARD_SIZE as i32 - 1 < 0 {
            action.col
        } else {
            BOARD_SIZE - 1 - action.row
        };
        let bl2tr_finfo = self.indexer.get_flip_info(action.color, &bl2tr, pos);

        // ひっくり返す石の数
        let flip_count = l2r_finfo.flip_count()
            + t2b_finfo.flip_count()
            + tl2br_finfo.flip_count()
            + bl2tr_finfo.flip_count();

        // ひっくり返せないなら無効なアクションなので受け付けない
        if flip_count == 0 {
            return false;
        }

        // アクションの箇所に石を置く
        self.squares[action.row][action.col] = action.color;

        // 左右方向
        for p in 1..=l2r_finfo.higher {
            let c = action.col + p as usize;
            self.squares[action.row][c] = action.color;
        }
        for p in 1..=l2r_finfo.lower {
            let c = action.col - p as usize;
            self.squares[action.row][c] = action.color;
        }

        // 上下方向
        for p in 1..=t2b_finfo.higher {
            let r = action.row + p as usize;
            self.squares[r][action.col] = action.color;
        }
        for p in 1..=t2b_finfo.lower {
            let r = action.row - p as usize;
            self.squares[r][action.col] = action.color;
        }

        // 左上右下方向
        for p in 1..=tl2br_finfo.higher {
            let r = action.row + p as usize;
            let c = action.col + p as usize;
            self.squares[r][c] = action.color;
        }
        for p in 1..=tl2br_finfo.lower {
            let r = action.row - p as usize;
            let c = action.col - p as usize;
            self.squares[r][c] = action.color;
        }

        // 左下右上方向
        for p in 1..=bl2tr_finfo.higher {
            let r = action.row - p as usize;
            let c = action.col + p as usize;
            self.squares[r][c] = action.color;
        }
        for p in 1..=bl2tr_finfo.lower {
            let r = action.row + p as usize;
            let c = action.col - p as usize;
            self.squares[r][c] = action.color;
        }

        true
    }

    fn get_line(&self, row: usize, col: usize, dir: LineDirection) -> [Square; BOARD_SIZE] {
        match dir {
            LineDirection::Left2Right => self.squares[row],
            LineDirection::Top2Bottom => {
                let mut line = [Square::Empty; BOARD_SIZE];
                for i in 0..BOARD_SIZE {
                    line[i] = self.squares[i][col];
                }
                line
            }
            LineDirection::TopLeft2BottomRight => {
                let mut line = [Square::Empty; BOARD_SIZE];
                let row = row as i32;
                let col = col as i32;
                let mut r = (row - col).max(0) as usize;
                let mut c = (col - row).max(0) as usize;
                for i in 0..BOARD_SIZE {
                    line[i] = self.squares[r][c];
                    r += 1;
                    c += 1;
                    if r >= BOARD_SIZE || c >= BOARD_SIZE {
                        break;
                    }
                }
                line
            }
            LineDirection::BottomLeft2TopRight => {
                let mut line = [Square::Empty; BOARD_SIZE];
                let row = row as i32;
                let col = col as i32;
                let mut r = (row + col).min(BOARD_SIZE as i32 - 1);
                let mut c = (row + col - (BOARD_SIZE as i32 - 1)).max(0);
                for i in 0..BOARD_SIZE {
                    line[i] = self.squares[r as usize][c as usize];
                    r -= 1;
                    c += 1;
                    if r < 0 || c >= BOARD_SIZE as i32 {
                        break;
                    }
                }
                line
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_line() {
        let board = Board::new();

        let line0 = board.get_line(0, 0, LineDirection::Left2Right);
        assert!(line0.iter().all(|s| *s == Square::Empty));

        let line1 = board.get_line(0, 0, LineDirection::Top2Bottom);
        assert!(line1.iter().all(|s| *s == Square::Empty));

        let line2 = board.get_line(0, 0, LineDirection::TopLeft2BottomRight);
        assert!(line2[0] == Square::Empty);
        assert!(line2[1] == Square::Empty);
        assert!(line2[2] == Square::Empty);
        assert!(line2[3] == Square::White);
        assert!(line2[4] == Square::White);
        assert!(line2[5] == Square::Empty);
        assert!(line2[6] == Square::Empty);
        assert!(line2[7] == Square::Empty);

        let line3 = board.get_line(0, 0, LineDirection::BottomLeft2TopRight);
        assert!(line3.iter().all(|s| *s == Square::Empty));

        let line4 = board.get_line(BOARD_SIZE - 1, 0, LineDirection::BottomLeft2TopRight);
        assert!(line4[0] == Square::Empty);
        assert!(line4[1] == Square::Empty);
        assert!(line4[2] == Square::Empty);
        assert!(line4[3] == Square::Black);
        assert!(line4[4] == Square::Black);
        assert!(line4[5] == Square::Empty);
        assert!(line4[6] == Square::Empty);
        assert!(line4[7] == Square::Empty);
    }

    #[test]
    fn test_apply_action() {
        let mut board = Board::new();

        let act = Action::new(Square::Black, 0, 0, false);
        let r = board.apply_action(&act);
        assert!(!r);

        let act = Action::new(Square::Black, 2, 3, false);
        let r = board.apply_action(&act);
        assert!(r);
        assert!(board.squares[2][3] == Square::Black);
        assert!(board.squares[3][3] == Square::Black);
        assert!(board.squares[4][3] == Square::Black);
        assert!(board.squares[3][4] == Square::Black);
        assert!(board.squares[4][4] == Square::White);

        let act = Action::new(Square::White, 2, 2, false);
        let r = board.apply_action(&act);
        assert!(r);
        assert!(board.squares[2][3] == Square::Black);
        assert!(board.squares[4][3] == Square::Black);
        assert!(board.squares[3][4] == Square::Black);
        assert!(board.squares[2][2] == Square::White);
        assert!(board.squares[3][3] == Square::White);
        assert!(board.squares[4][4] == Square::White);
    }
}
