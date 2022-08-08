use crate::reversi::common::*;

pub fn simple_evaluate(board: &Squares, color: &PlayerColor) -> i32 {
    let weight_table: [i32; 64] = [
        30, -12, 0, -1, -1, 0, -12, 30, //
        -12, -15, -3, -3, -3, -3, -15, -12, //
        0, -3, 0, -1, -1, 0, -3, 0, //
        -1, -3, -1, -1, -1, -1, -3, -1, //
        -1, -3, -1, -1, -1, -1, -3, -1, //
        0, -3, 0, -1, -1, 0, -3, 0, //
        -12, -15, -3, -3, -3, -3, -15, -12, //
        30, -12, 0, -1, -1, 0, -12, 30, //
    ];
    let value = board
        .iter()
        .zip(weight_table.iter())
        .fold(0, |v, (s, w)| -> i32 {
            let color = match color {
                PlayerColor::Black => Square::Black,
                PlayerColor::White => Square::White,
            };
            if *s == Square::Empty {
                v
            } else if *s == color {
                v + *w
            } else {
                v - *w
            }
        });
    value
}
