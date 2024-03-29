﻿use crate::board::indexer::FlipInfo;
use crate::board::indexer::Indexer;
use crate::board::Board;
use crate::*;
use std::rc::Rc;

/// ボード
#[derive(Clone)]
pub struct IndexBoard {
    squares: Squares,
    depth: u32,
    indexer: Rc<Indexer>,
}

impl IndexBoard {
    /// 新規作成
    pub fn new_initial(indexer: Rc<Indexer>) -> IndexBoard {
        let mut squares: Squares = [Square::Empty; BOARD_SIZE * BOARD_SIZE];
        squares[position_to_index(&Position(3, 4))] = Square::Black;
        squares[position_to_index(&Position(4, 3))] = Square::Black;
        squares[position_to_index(&Position(3, 3))] = Square::White;
        squares[position_to_index(&Position(4, 4))] = Square::White;

        IndexBoard::new(squares, 0, indexer)
    }

    pub fn new(squares: Squares, depth: u32, indexer: Rc<Indexer>) -> IndexBoard {
        IndexBoard {
            squares,
            depth,
            indexer,
        }
    }

    fn get_flip_infos(
        &self,
        color: &PlayerColor,
        pos: &Position,
    ) -> (&FlipInfo, &FlipInfo, &FlipInfo, &FlipInfo) {
        // 左右方向の情報
        let l2r = self.get_line(*pos, LineDirection::Left2Right);
        let l2r_finfo = self.indexer.get_flip_info(*color, &l2r, pos.1);

        // 上下方向の情報
        let t2b = self.get_line(*pos, LineDirection::Top2Bottom);
        let t2b_finfo = self.indexer.get_flip_info(*color, &t2b, pos.0);

        let r = pos.0 as i32;
        let c = pos.1 as i32;
        // 左上から右下方向の情報
        let tl2br = self.get_line(*pos, LineDirection::TopLeft2BottomRight);
        let p = if r - c >= 0 { pos.1 } else { pos.0 };
        let tl2br_finfo = self.indexer.get_flip_info(*color, &tl2br, p);

        // 左下から右上方向の情報
        let bl2tr = self.get_line(*pos, LineDirection::BottomLeft2TopRight);
        let pos = if r + c - BOARD_SIZE as i32 + 1 < 0 {
            pos.1
        } else {
            BOARD_SIZE - 1 - pos.0
        };
        let bl2tr_finfo = self.indexer.get_flip_info(*color, &bl2tr, pos);

        (l2r_finfo, t2b_finfo, tl2br_finfo, bl2tr_finfo)
    }

    fn get_line(&self, pos: Position, dir: LineDirection) -> [Square; BOARD_SIZE] {
        let mut line = [Square::Empty; BOARD_SIZE];
        match dir {
            LineDirection::Left2Right => {
                let index = position_to_index(&Position(pos.0, 0));
                line.copy_from_slice(&self.squares[index..index + BOARD_SIZE]);
                line
            }
            LineDirection::Top2Bottom => {
                for (i, l) in line.iter_mut().enumerate() {
                    let index = position_to_index(&Position(i, pos.1));
                    *l = self.squares[index];
                }
                line
            }
            LineDirection::TopLeft2BottomRight => {
                let row = pos.0 as i32;
                let col = pos.1 as i32;
                let mut r = (row - col).max(0) as usize;
                let mut c = (col - row).max(0) as usize;
                for l in line.iter_mut() {
                    let index = position_to_index(&Position(r, c));
                    *l = self.squares[index];
                    r += 1;
                    c += 1;
                    if r >= BOARD_SIZE || c >= BOARD_SIZE {
                        break;
                    }
                }
                line
            }
            LineDirection::BottomLeft2TopRight => {
                let row = pos.0 as i32;
                let col = pos.1 as i32;
                let mut r = (row + col).min(BOARD_SIZE as i32 - 1);
                let mut c = (row + col - (BOARD_SIZE as i32 - 1)).max(0);
                for l in line.iter_mut() {
                    let index = position_to_index(&Position(r as usize, c as usize));
                    *l = self.squares[index];
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

    fn can_pass(&self, color: &PlayerColor) -> bool {
        self.get_movable_positions(color).is_empty()
    }
}

impl Board for IndexBoard {
    fn apply_move(&self, move_: &Move) -> Option<IndexBoard> {
        match move_ {
            Move::Pass(color) => {
                // パスできるかチェック
                if self.can_pass(&color) {
                    Some(IndexBoard::new(
                        self.squares,
                        self.depth + 1,
                        self.indexer.clone(),
                    ))
                } else {
                    None
                }
            }
            Move::Position(color, position) => {
                let index = position_to_index(&position);
                if self.squares[index] != Square::Empty {
                    // 空きマス以外には石を置けない
                    return None;
                }

                // 各方向の情報取得
                let (l2r_finfo, t2b_finfo, tl2br_finfo, bl2tr_finfo) =
                    self.get_flip_infos(&color, &position);

                // ひっくり返す石の数
                let flip_count = l2r_finfo.flip_count()
                    + t2b_finfo.flip_count()
                    + tl2br_finfo.flip_count()
                    + bl2tr_finfo.flip_count();

                // ひっくり返せないなら無効なアクションなので受け付けない
                if flip_count == 0 {
                    return None;
                }

                let mut squares = self.squares;

                let square_color = match color {
                    PlayerColor::Black => Square::Black,
                    PlayerColor::White => Square::White,
                };

                // アクションの箇所に石を置く
                squares[index] = square_color;

                // 左右方向
                for p in 1..=l2r_finfo.higher {
                    let c = position.1 + p as usize;
                    let index = position_to_index(&Position(position.0, c));
                    squares[index] = square_color;
                }
                for p in 1..=l2r_finfo.lower {
                    let c = position.1 - p as usize;
                    let index = position_to_index(&Position(position.0, c));
                    squares[index] = square_color;
                }

                // 上下方向
                for p in 1..=t2b_finfo.higher {
                    let r = position.0 + p as usize;
                    let index = position_to_index(&Position(r, position.1));
                    squares[index] = square_color;
                }
                for p in 1..=t2b_finfo.lower {
                    let r = position.0 - p as usize;
                    let index = position_to_index(&Position(r, position.1));
                    squares[index] = square_color;
                }

                // 左上右下方向
                for p in 1..=tl2br_finfo.higher {
                    let r = position.0 + p as usize;
                    let c = position.1 + p as usize;
                    squares[position_to_index(&Position(r, c))] = square_color;
                }
                for p in 1..=tl2br_finfo.lower {
                    let r = position.0 - p as usize;
                    let c = position.1 - p as usize;
                    squares[position_to_index(&Position(r, c))] = square_color;
                }

                // 左下右上方向
                for p in 1..=bl2tr_finfo.higher {
                    let r = position.0 - p as usize;
                    let c = position.1 + p as usize;
                    squares[position_to_index(&Position(r, c))] = square_color;
                }
                for p in 1..=bl2tr_finfo.lower {
                    let r = position.0 + p as usize;
                    let c = position.1 - p as usize;
                    squares[position_to_index(&Position(r, c))] = square_color;
                }

                Some(IndexBoard::new(
                    squares,
                    self.depth + 1,
                    self.indexer.clone(),
                ))
            }
        }
    }

    fn get_movable_positions(&self, color: &PlayerColor) -> Vec<Position> {
        let mut positions: Vec<Position> = Vec::new();
        for r in 0..BOARD_SIZE {
            for c in 0..BOARD_SIZE {
                let (l2r, t2b, tl2br, bl2tr) = self.get_flip_infos(color, &Position(r, c));
                let count =
                    l2r.flip_count() + t2b.flip_count() + tl2br.flip_count() + bl2tr.flip_count();
                if count > 0 {
                    positions.push(Position(r, c));
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

    fn duplicate(&self) -> IndexBoard {
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
    fn test_get_line() {
        let indexer = Rc::new(Indexer::new());
        let board = IndexBoard::new_initial(indexer);

        let line0 = board.get_line(Position(0, 0), LineDirection::Left2Right);
        assert!(line0.iter().all(|s| *s == Square::Empty));

        let line1 = board.get_line(Position(0, 0), LineDirection::Top2Bottom);
        assert!(line1.iter().all(|s| *s == Square::Empty));

        let line2 = board.get_line(Position(0, 0), LineDirection::TopLeft2BottomRight);
        assert!(line2[0] == Square::Empty);
        assert!(line2[1] == Square::Empty);
        assert!(line2[2] == Square::Empty);
        assert!(line2[3] == Square::White);
        assert!(line2[4] == Square::White);
        assert!(line2[5] == Square::Empty);
        assert!(line2[6] == Square::Empty);
        assert!(line2[7] == Square::Empty);

        let line3 = board.get_line(Position(0, 0), LineDirection::BottomLeft2TopRight);
        assert!(line3.iter().all(|s| *s == Square::Empty));

        let line4 = board.get_line(
            Position(BOARD_SIZE - 1, 0),
            LineDirection::BottomLeft2TopRight,
        );
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
        let indexer = Rc::new(Indexer::new());
        let board = IndexBoard::new_initial(indexer);

        let m = Move::new_position(PlayerColor::Black, Position(0, 0));
        let r = board.apply_move(&m);
        assert!(r.is_none());

        let m = Move::new_position(PlayerColor::Black, Position(2, 3));
        let r = board.apply_move(&m);
        assert!(r.is_some());
        let next_board = r.unwrap();
        assert!(next_board.squares[position_to_index(&Position(2, 3))] == Square::Black);
        assert!(next_board.squares[position_to_index(&Position(3, 3))] == Square::Black);
        assert!(next_board.squares[position_to_index(&Position(4, 3))] == Square::Black);
        assert!(next_board.squares[position_to_index(&Position(3, 4))] == Square::Black);
        assert!(next_board.squares[position_to_index(&Position(4, 4))] == Square::White);

        let m = Move::new_position(PlayerColor::White, Position(2, 2));
        let r = next_board.apply_move(&m);
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
