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
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Square {
    Empty = 0,
    Black = 1,
    White = 2,
}

// プレイヤー先手or後手
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PlayerColor {
    Black,
    White,
}

impl PlayerColor {
    pub fn opponent(&self) -> PlayerColor {
        if *self == PlayerColor::Black {
            PlayerColor::White
        } else {
            PlayerColor::Black
        }
    }
}

/// 位置
/// (行, 列)のタプル
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Position(pub usize, pub usize);

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ActionType {
    Move(Position),
    Pass,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Action {
    pub color: PlayerColor,
    pub action: ActionType,
}

impl Action {
    pub fn new(color: PlayerColor, action: ActionType) -> Action {
        Action { color, action }
    }
}

/// ゲーム状態通知用オブジェクト
pub struct GameStateDto {
    pub board: Squares,
    pub depth: u32,
    pub black_count: u32,
    pub white_count: u32,
    pub is_end: bool,
    pub turn: PlayerColor,
    pub last_action: Option<Action>,
}
