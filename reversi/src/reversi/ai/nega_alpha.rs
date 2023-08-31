use crate::{
    board::{BitBoard, Board},
    Move, PlayerColor,
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

struct NegaAlpha<E>
where
    E: NegaAlphaEvaluationFunction,
{
    eval: E,
}

impl<E> NegaAlpha<E>
where
    E: NegaAlphaEvaluationFunction,
{
    fn search(&mut self, node: &mut NegaAlphaNode, depth: usize) -> i32 {
        Self::nega_alpha(node, depth, &mut self.eval)
    }

    fn nega_alpha(node: &mut NegaAlphaNode, depth: usize, eval: &mut E) -> i32
    where
        E: NegaAlphaEvaluationFunction,
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
                .map(|child| -Self::nega_alpha(child, depth - 1, eval))
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

    struct TestEvaluationFunction {}

    impl NegaAlphaEvaluationFunction for TestEvaluationFunction {
        fn evaluate(&mut self, node: &NegaAlphaNode) -> i32 {
            let black = node.board.square_count(Square::Black) as i32;
            let white = node.board.square_count(Square::White) as i32;
            match node.color {
                PlayerColor::Black => black - white,
                PlayerColor::White => white - black,
            }
        }
    }

    #[test]
    fn test_nega_max() {
        let mut nega_max = NegaAlpha {
            eval: TestEvaluationFunction {},
        };

        let mut root = NegaAlphaNode {
            board: BitBoard::new_initial(),
            color: PlayerColor::Black,
            move_count: 0,
            last_move: Move::new_pass(PlayerColor::White),
            value: None,
            children: Vec::new(),
        };

        nega_max.search(&mut root, 1);
        for child in &root.children {
            println!(
                "{}: {}",
                child.board.to_console_text(),
                child.value.unwrap()
            );
        }
        assert_eq!(root.value, Some(3));
    }
}
