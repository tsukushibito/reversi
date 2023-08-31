use crate::{
    board::{BitBoard, Board},
    Move, PlayerColor,
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

pub trait NegaMaxEvaluationFunction {
    fn evaluate(&mut self, node: &NegaMaxNode) -> i32;
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
    use crate::board::Board;
    use crate::Square;

    struct TestEvaluationFunction {
        is_first: bool,
    }

    impl NegaMaxEvaluationFunction for TestEvaluationFunction {
        fn evaluate(&mut self, node: &NegaMaxNode) -> i32 {
            if self.is_first {
                self.is_first = false;
                10
            } else {
                -10
            }
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
            eval: TestEvaluationFunction { is_first: true },
        };

        let mut root = NegaMaxNode {
            board: BitBoard::new_initial(),
            color: PlayerColor::Black,
            move_count: 0,
            last_move: Move::new_pass(PlayerColor::White),
            value: None,
            children: Vec::new(),
        };

        nega_max.search(&mut root, 7);
        println!("node_count: {}", root.node_count());
        println!("searched_nodes: {}", root.searched_nodes());
        println!("value: {}", root.value().unwrap());
        assert_eq!(root.value, Some(10));
    }
}
