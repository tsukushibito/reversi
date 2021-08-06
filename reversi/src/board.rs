use crate::indexer::Indexer;

pub const BOARD_SIZE: usize = 8;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Square {
    Empty = 1,
    Black = 2,
    White = 0,
}

struct Board {
    squares: [[Square; BOARD_SIZE]; BOARD_SIZE],
    indexer: Indexer,
}

struct Action {
    pub color: Square,
    pub row: usize,
    pub col: usize,
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

    pub fn apply_action(&mut self, action: &Action) {}
}
