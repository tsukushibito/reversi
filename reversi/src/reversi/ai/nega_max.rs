use crate::{
    board::{BitBoard, Board},
    Move, PlayerColor, Square, Squares,
};

use super::node::Node;

pub struct NegaMaxNode {
    pub board: BitBoard,
    pub color: PlayerColor,
    pub move_count: u8,
    pub last_move: Move,
    pub value: Option<i32>,
    pub children: Vec<NegaMaxNode>,
}

impl Node for NegaMaxNode {
    fn new(board: BitBoard, color: PlayerColor, move_count: u8, last_move: Move) -> Self {
        NegaMaxNode {
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

impl NegaMaxNode {
    fn to_string_impl(&self, str: &mut String, depth: usize) {
        for _ in 0..depth {
            str.push_str("  ");
        }
        str.push_str(&format!(
            "move_count: {}, value: {:?}\n",
            self.move_count, self.value
        ));
        for child in &self.children {
            child.to_string_impl(str, depth + 1);
        }
    }

    pub fn to_string(&self) -> String {
        let mut str = String::new();
        self.to_string_impl(&mut str, 0);
        str
    }
}

pub trait NegaMaxEvaluationFunction {
    fn evaluate(&mut self, node: &NegaMaxNode) -> i32;
}

pub struct SimpleNegaMaxEvaluationFunction {}

impl SimpleNegaMaxEvaluationFunction {
    pub fn new() -> Self {
        SimpleNegaMaxEvaluationFunction {}
    }
}

impl NegaMaxEvaluationFunction for SimpleNegaMaxEvaluationFunction {
    fn evaluate(&mut self, node: &NegaMaxNode) -> i32 {
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

pub struct NegaMax<E>
where
    E: NegaMaxEvaluationFunction,
{
    eval: E,
}

impl<E> NegaMax<E>
where
    E: NegaMaxEvaluationFunction,
{
    pub fn new(eval: E) -> Self {
        NegaMax { eval }
    }

    pub fn search(&mut self, node: &mut NegaMaxNode, depth: usize) -> i32 {
        Self::nega_max(node, depth, &mut self.eval)
    }

    fn nega_max(node: &mut NegaMaxNode, depth: usize, eval: &mut E) -> i32
    where
        E: NegaMaxEvaluationFunction,
    {
        if node.board.is_game_over() || depth == 0 {
            let value = eval.evaluate(&node);
            node.value = Some(value);
            value
        } else {
            node.expand();

            let vs: Vec<i32> = node
                .children
                .iter_mut()
                .map(|child| -Self::nega_max(child, depth - 1, eval))
                .collect();

            let v = vs.iter().max_by(|a, b| a.cmp(b)).expect("no children");

            node.value = Some(*v);
            *v
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestEvaluationFunction {
        param: i32,
    }

    impl NegaMaxEvaluationFunction for TestEvaluationFunction {
        fn evaluate(&mut self, node: &NegaMaxNode) -> i32 {
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

    #[test]
    fn test_nega_max() {
        let mut nega_max = NegaMax {
            eval: TestEvaluationFunction { param: 0 },
        };

        let mut root = NegaMaxNode {
            board: BitBoard::new_initial(),
            color: PlayerColor::Black,
            move_count: 0,
            last_move: Move::new_pass(PlayerColor::White),
            value: None,
            children: Vec::new(),
        };

        nega_max.search(&mut root, 2);
        print!("{}", root.to_string());
        // println!("node_count: {}", root.node_count());
        // println!("searched_nodes: {}", root.searched_nodes());
        // println!("value: {}", root.value().unwrap());
        // assert_eq!(root.value, Some(10));
    }
}
