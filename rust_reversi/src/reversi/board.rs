use super::action::Action;
use super::indexer::FlipInfo;
use super::indexer::Indexer;
use std::rc::Rc;

pub const BOARD_SIZE: usize = 8;
pub type Squares = Vec<Vec<Square>>;

/// 方向
pub enum LineDirection {
    Left2Right,
    Top2Bottom,
    TopLeft2BottomRight,
    BottomLeft2TopRight,
}

/// マスの状態
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Square {
    Empty = 1,
    Black = 2,
    White = 0,
}

/// ボード
#[derive(Clone)]
pub struct Board {
    pub squares: Squares,
    indexer: Rc<Indexer>,
}

impl Board {
    /// 新規作成
    pub fn new_initial() -> Board {
        let mut squares = Squares::new();
        for r in 0..BOARD_SIZE {
            squares.push(Vec::new());
            for _ in 0..BOARD_SIZE {
                squares[r].push(Square::Empty);
            }
        }
        squares[3][4] = Square::Black;
        squares[4][3] = Square::Black;
        squares[3][3] = Square::White;
        squares[4][4] = Square::White;

        Board::new(squares, Rc::new(Indexer::new()))
    }

    pub fn new(squares: Squares, indexer: Rc<Indexer>) -> Board {
        Board {
            squares: squares,
            indexer: indexer,
        }
    }

    /// アクション受付
    /// 受け付けた結果のボードを返す
    pub fn apply_action(&self, action: &Action) -> Option<Board> {
        if action.pass {
            // パスできるかチェック
            if self.can_pass(action.color) {
                return Some(self.clone());
            } else {
                return None;
            }
        }

        if self.squares[action.row][action.col] != Square::Empty {
            // 空きマス以外には石を置けない
            return None;
        }

        // 各方向の情報取得
        let (l2r_finfo, t2b_finfo, tl2br_finfo, bl2tr_finfo) =
            self.get_flip_infos(action.color, action.row, action.col);

        // ひっくり返す石の数
        let flip_count = l2r_finfo.flip_count()
            + t2b_finfo.flip_count()
            + tl2br_finfo.flip_count()
            + bl2tr_finfo.flip_count();

        // ひっくり返せないなら無効なアクションなので受け付けない
        if flip_count == 0 {
            return None;
        }

        let mut squares = self.squares.clone();

        // アクションの箇所に石を置く
        squares[action.row][action.col] = action.color;

        // 左右方向
        for p in 1..=l2r_finfo.higher {
            let c = action.col + p as usize;
            squares[action.row][c] = action.color;
        }
        for p in 1..=l2r_finfo.lower {
            let c = action.col - p as usize;
            squares[action.row][c] = action.color;
        }

        // 上下方向
        for p in 1..=t2b_finfo.higher {
            let r = action.row + p as usize;
            squares[r][action.col] = action.color;
        }
        for p in 1..=t2b_finfo.lower {
            let r = action.row - p as usize;
            squares[r][action.col] = action.color;
        }

        // 左上右下方向
        for p in 1..=tl2br_finfo.higher {
            let r = action.row + p as usize;
            let c = action.col + p as usize;
            squares[r][c] = action.color;
        }
        for p in 1..=tl2br_finfo.lower {
            let r = action.row - p as usize;
            let c = action.col - p as usize;
            squares[r][c] = action.color;
        }

        // 左下右上方向
        for p in 1..=bl2tr_finfo.higher {
            let r = action.row - p as usize;
            let c = action.col + p as usize;
            squares[r][c] = action.color;
        }
        for p in 1..=bl2tr_finfo.lower {
            let r = action.row + p as usize;
            let c = action.col - p as usize;
            squares[r][c] = action.color;
        }

        Some(Board::new(squares, self.indexer.clone()))
    }

    pub fn get_movable_positions(&self, color: Square) -> Vec<(usize, usize)> {
        let mut positions = Vec::new();
        for r in 0..BOARD_SIZE {
            for c in 0..BOARD_SIZE {
                let (l2r, t2b, tl2br, bl2tr) = self.get_flip_infos(color, r, c);
                let count =
                    l2r.flip_count() + t2b.flip_count() + tl2br.flip_count() + bl2tr.flip_count();
                if count > 0 {
                    positions.push((r, c));
                }
            }
        }
        positions
    }

    pub fn is_game_over(&self) -> bool {
        self.get_movable_positions(Square::Black).len()
            + self.get_movable_positions(Square::White).len()
            == 0
    }

    pub fn square_count(&self, color: Square) -> u32 {
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

    pub fn black_count(&self) -> u32 {
        self.square_count(Square::Black)
    }

    pub fn white_count(&self) -> u32 {
        self.square_count(Square::White)
    }

    fn get_flip_infos(
        &self,
        color: Square,
        row: usize,
        col: usize,
    ) -> (&FlipInfo, &FlipInfo, &FlipInfo, &FlipInfo) {
        // 左右方向の情報
        let l2r = self.get_line(row, col, LineDirection::Left2Right);
        let l2r_finfo = self.indexer.get_flip_info(color, &l2r, col);

        // 上下方向の情報
        let t2b = self.get_line(row, col, LineDirection::Top2Bottom);
        let t2b_finfo = self.indexer.get_flip_info(color, &t2b, row);

        let r = row as i32;
        let c = col as i32;
        // 左上から右下方向の情報
        let tl2br = self.get_line(row, col, LineDirection::TopLeft2BottomRight);
        let pos = if r - c >= 0 { col } else { row };
        let tl2br_finfo = self.indexer.get_flip_info(color, &tl2br, pos);

        // 左下から右上方向の情報
        let bl2tr = self.get_line(row, col, LineDirection::BottomLeft2TopRight);
        let pos = if r + c - BOARD_SIZE as i32 + 1 < 0 {
            col
        } else {
            BOARD_SIZE - 1 - row
        };
        let bl2tr_finfo = self.indexer.get_flip_info(color, &bl2tr, pos);

        (l2r_finfo, t2b_finfo, tl2br_finfo, bl2tr_finfo)
    }

    fn get_line(&self, row: usize, col: usize, dir: LineDirection) -> Vec<Square> {
        let mut line = Vec::new();
        for _ in 0..BOARD_SIZE {
            line.push(Square::Empty);
        }
        match dir {
            LineDirection::Left2Right => {
                line = self.squares[row].clone();
                line
            }
            LineDirection::Top2Bottom => {
                for i in 0..BOARD_SIZE {
                    line[i] = self.squares[i][col];
                }
                line
            }
            LineDirection::TopLeft2BottomRight => {
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

    fn can_pass(&self, color: Square) -> bool {
        let mut count = 0;
        for r in 0..BOARD_SIZE {
            for c in 0..BOARD_SIZE {
                if self.squares[r][c] != Square::Empty {
                    continue;
                }
                let (l2r_finfo, t2b_finfo, tl2br_finfo, bl2tr_finfo) =
                    self.get_flip_infos(color, r, c);
                count += l2r_finfo.flip_count()
                    + t2b_finfo.flip_count()
                    + tl2br_finfo.flip_count() * bl2tr_finfo.flip_count();
            }
        }

        count == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_line() {
        let board = Board::new_initial();

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
        let board = Board::new_initial();

        let act = Action::new_move(Square::Black, 0, 0);
        let r = board.apply_action(&act);
        assert!(r.is_none());

        let act = Action::new_move(Square::Black, 2, 3);
        let r = board.apply_action(&act);
        assert!(r.is_some());
        let next_board = r.unwrap();
        assert!(next_board.squares[2][3] == Square::Black);
        assert!(next_board.squares[3][3] == Square::Black);
        assert!(next_board.squares[4][3] == Square::Black);
        assert!(next_board.squares[3][4] == Square::Black);
        assert!(next_board.squares[4][4] == Square::White);

        let act = Action::new_move(Square::White, 2, 2);
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
