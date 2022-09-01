use crate::reversi::common::*;

pub struct EvalResult {
    pub value: i32,
    pub policy: Option<[i32; BOARD_SIZE * BOARD_SIZE]>,
}

pub trait Evaluator {
    fn evaluate(board: &Squares, color: &PlayerColor) -> EvalResult;
}

pub struct SimpleEvaluator {}

impl Evaluator for SimpleEvaluator {
    fn evaluate(board: &Squares, color: &PlayerColor) -> EvalResult {
        simple_evaluate(board, color)
    }
}

pub fn simple_evaluate(board: &Squares, color: &PlayerColor) -> EvalResult {
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
    EvalResult {
        value,
        policy: None,
    }
}

#[cfg(test)]
mod tests {
    use crate::board::{BitBoard, Board};

    use super::*;

    #[test]
    fn test_simple_evaluate() {
        let board = BitBoard::new_initial();
        let result0 = simple_evaluate(board.squares(), &PlayerColor::Black);
        assert_eq!(result0.value, 0);

        let result1 = simple_evaluate(board.squares(), &PlayerColor::White);
        assert_eq!(result1.value, 0);

        let mut squares: Squares = [Square::Empty; BOARD_SIZE * BOARD_SIZE];
        squares[0] = Square::Black;
        squares[1] = Square::White;

        let result2 = simple_evaluate(&squares, &PlayerColor::Black);
        assert_eq!(result2.value, 42);

        let result3 = simple_evaluate(&squares, &PlayerColor::White);
        assert_eq!(result3.value, -42);
    }
}
