use crate::board::Square;

pub struct Action {
    pub color: Square,
    pub row: usize,
    pub col: usize,
    pub pass: bool,
}

impl Action {
    pub fn new_move(color: Square, row: usize, col: usize) -> Action {
        Action {
            color: color,
            row: row,
            col: col,
            pass: false,
        }
    }

    pub fn new_pass(color: Square) -> Action {
        Action {
            color: color,
            row: 0,
            col: 0,
            pass: true,
        }
    }
}
