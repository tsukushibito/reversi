pub const BOARD_SIZE: usize = 8;

pub type Squares = [Square; BOARD_SIZE * BOARD_SIZE];
pub const fn position_to_index(pos: &Position) -> usize {
    BOARD_SIZE * pos.0 + pos.1
}

pub const fn index_to_position(index: usize) -> Position {
    Position(index / BOARD_SIZE, index % BOARD_SIZE)
}

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

pub fn squares_to_string(squares: &Squares) -> String {
    let mut text = String::from("   a b c d e f g h\n");
    text += "   ----------------\n";
    for r in 0..BOARD_SIZE {
        let mut row_string = (r + 1).to_string() + "|";
        let index = r * BOARD_SIZE;
        for square in &squares[index..index + BOARD_SIZE] {
            row_string += match square {
                Square::Empty => " .",
                Square::Black => " b",
                Square::White => " w",
            }
        }
        row_string += "\n";
        text += &row_string;
    }
    text
}
