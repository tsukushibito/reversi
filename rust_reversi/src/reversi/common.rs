use std::rc::Rc;

pub const BOARD_SIZE: usize = 8;

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
    Empty = 0,
    Black = 1,
    White = 2,
}

// プレイヤー先手or後手
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlayerColor {
    Black,
    White,
}

/// 位置
/// (行, 列)のタプル
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position(pub usize, pub usize);

#[derive(PartialEq, Eq, Hash)]
pub enum ActionType {
    Move(Position),
    Pass,
}

#[derive(PartialEq, Eq, Hash)]
pub struct Action {
    pub color: PlayerColor,
    pub action: ActionType,
}

impl Action {
    pub fn new(color: PlayerColor, action: ActionType) -> Action {
        Action {
            color: color,
            action: action,
        }
    }
}

pub trait Board<T> {
    fn apply_action(&self, action: &Action) -> Option<Rc<T>>;
    fn get_movable_positions(&self, color: &PlayerColor) -> Vec<Position>;
    fn is_game_over(&self) -> bool;
    fn square_count(&self, color: Square) -> u32;
    fn black_count(&self) -> u32;
    fn white_count(&self) -> u32;
    fn squares(&self) -> &Squares;
}
