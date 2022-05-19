use std::rc::Rc;

pub const BOARD_SIZE: usize = 8;

#[derive(PartialEq, Eq, Hash)]
pub struct Action {
    pub color: Square,
    pub position: BoardPosition,
    pub pass: bool,
}

impl Action {
    pub fn new_move(color: Square, position: BoardPosition) -> Action {
        Action {
            color: color,
            position: position,
            pass: false,
        }
    }

    pub fn new_pass(color: Square) -> Action {
        Action {
            color: color,
            position: BoardPosition(0, 0),
            pass: true,
        }
    }
}

pub type Squares = [[Square; BOARD_SIZE]; BOARD_SIZE];

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

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct BoardPosition(pub usize, pub usize);

pub trait Board {
    fn apply_action(&self, action: &Action) -> Option<Rc<dyn Board>>;
    fn get_movable_positions(&self, color: Square) -> Vec<BoardPosition>;
    fn is_game_over(&self) -> bool;
    fn square_count(&self, color: Square) -> u32;
    fn black_count(&self) -> u32;
    fn white_count(&self) -> u32;
    fn squares(&self) -> &Squares;
}
