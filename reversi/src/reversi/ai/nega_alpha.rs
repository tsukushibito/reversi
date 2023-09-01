use crate::{
    board::{BitBoard, Board},
    Move, PlayerColor, Square, Squares,
};

use super::node::Node;

pub struct NegaAlphaNode {
    pub board: BitBoard,
    pub color: PlayerColor,
    pub move_count: u8,
    pub last_move: Move,
    pub value: Option<i32>,
    pub children: Vec<NegaAlphaNode>,
}

impl Node for NegaAlphaNode {
    fn new(board: BitBoard, color: PlayerColor, move_count: u8, last_move: Move) -> Self {
        NegaAlphaNode {
            board,
            color,
            move_count,
            last_move,
            value: None,
            children: Vec::new(),
        }
    }

    fn board(&self) -> &BitBoard {
        &self.board
    }

    fn color(&self) -> &PlayerColor {
        &self.color
    }

    fn move_count(&self) -> &u8 {
        &self.move_count
    }

    fn children(&self) -> &[Self] {
        &self.children
    }

    fn children_mut(&mut self) -> &mut Vec<Self> {
        &mut self.children
    }

    fn set_children(&mut self, children: Vec<Self>) {
        self.children = children;
    }

    fn value(&self) -> &Option<i32> {
        &self.value
    }

    fn value_mut(&mut self) -> &mut Option<i32> {
        &mut self.value
    }

    fn last_move(&self) -> &Move {
        &self.last_move
    }
}

pub trait NegaAlphaEvaluationFunction {
    fn evaluate(&mut self, node: &NegaAlphaNode) -> i32;
}

pub struct SimpleNegaAlphaEvaluationFunction {}

impl SimpleNegaAlphaEvaluationFunction {
    pub fn new() -> Self {
        SimpleNegaAlphaEvaluationFunction {}
    }
}

impl NegaAlphaEvaluationFunction for SimpleNegaAlphaEvaluationFunction {
    fn evaluate(&mut self, node: &NegaAlphaNode) -> i32 {
        simple_evaluate(&node.board.squares(), &node.color)
    }
}

fn simple_evaluate(board: &Squares, color: &PlayerColor) -> i32 {
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

pub struct NegaAlpha<E>
where
    E: NegaAlphaEvaluationFunction,
{
    eval: E,
}

impl<E> NegaAlpha<E>
where
    E: NegaAlphaEvaluationFunction,
{
    pub fn new(eval: E) -> Self {
        NegaAlpha { eval }
    }

    pub fn search(&mut self, node: &mut NegaAlphaNode, depth: usize) -> i32 {
        Self::nega_alpha(node, depth, i32::MIN + 1, i32::MAX, &mut self.eval)
    }

    fn nega_alpha(
        node: &mut NegaAlphaNode,
        depth: usize,
        alpha: i32,
        beta: i32,
        eval: &mut E,
    ) -> i32
    where
        E: NegaAlphaEvaluationFunction,
    {
        if node.board.is_game_over() || depth == 0 {
            let value = eval.evaluate(node);
            node.value = Some(value);
            value
        } else {
            node.expand();

            let mut alpha = alpha;
            for child in node.children_mut().iter_mut() {
                let v = -Self::nega_alpha(child, depth - 1, -beta, -alpha, eval);
                if v > alpha {
                    alpha = v;
                }
                if alpha >= beta {
                    break;
                }
            }

            node.value = Some(alpha);
            alpha
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::nega_max::*;
    use super::*;

    struct TestEvaluationFunction {
        param: i32,
    }

    impl NegaAlphaEvaluationFunction for TestEvaluationFunction {
        fn evaluate(&mut self, node: &NegaAlphaNode) -> i32 {
            self.param += 1;
            if self.param > 10 {
                self.param = 0;
            }
            self.param
            // let black = node.board.square_count(Square::Black) as i32;
            // let white = node.board.square_count(Square::White) as i32;
            // match node.color {
            //     PlayerColor::Black => black - white,
            //     PlayerColor::White => white - black,
            // }
        }
    }

    struct TestEvaluationFunction2 {
        param: i32,
    }

    impl NegaMaxEvaluationFunction for TestEvaluationFunction2 {
        fn evaluate(&mut self, node: &NegaMaxNode) -> i32 {
            self.param += 1;
            if self.param > 10 {
                self.param = 0;
            }
            self.param
        }
    }

    #[test]
    fn test_nega_max() {
        let mut nega_alpha = NegaAlpha {
            eval: TestEvaluationFunction { param: 0 },
        };

        let mut root = NegaAlphaNode {
            board: BitBoard::new_initial(),
            color: PlayerColor::Black,
            move_count: 0,
            last_move: Move::new_pass(PlayerColor::White),
            value: None,
            children: Vec::new(),
        };

        nega_alpha.search(&mut root, 5);
        // println!("node_count: {}", root.node_count());
        // println!("searched_nodes: {}", root.searched_nodes());
        // println!("value: {}", root.value().unwrap());
    }
}
