use crate::board::Square;

pub struct Action {
    pub color: Square,
    pub row: usize,
    pub col: usize,
    pub pass: bool,
}

impl Action {
    pub fn new(color: Square, row: usize, col: usize, pass: bool) -> Action {
        Action {
            color: color,
            row: row,
            col: col,
            pass: pass,
        }
    }
}
