use crate::{
    board::{BitBoard, Board},
    Action, ActionType, PlayerColor,
};

use super::node::Node;

pub struct NegaMaxNode {
    pub board: BitBoard,
    pub color: PlayerColor,
    pub move_count: u8,
    pub last_action: Action,
    pub value: Option<i32>,
    pub children: Vec<NegaMaxNode>,
}

impl NegaMaxNode {
    pub fn expand(&mut self) {
        let positions = self.board.get_movable_positions(&self.color);
        self.children = positions
            .iter()
            .map(|position| {
                let action = Action::new(self.color, ActionType::Move(*position));
                let next_board = self.board.apply_action(&action).unwrap();
                NegaMaxNode {
                    board: next_board,
                    color: self.color.opponent(),
                    move_count: self.move_count + 1,
                    last_action: action,
                    value: None,
                    children: Vec::new(),
                }
            })
            .collect::<Vec<_>>();
    }
}

impl Node for NegaMaxNode {
    fn children(&self) -> &[Self] {
        &self.children
    }

    fn children_mut(&mut self) -> &mut Vec<Self> {
        &mut self.children
    }

    fn value(&self) -> &Option<i32> {
        &self.value
    }

    fn value_mut(&mut self) -> &mut Option<i32> {
        &mut self.value
    }

    fn last_action(&self) -> &Action {
        &self.last_action
    }

    fn expand(&mut self) {
        let positions = self.board.get_movable_positions(&self.color);
        self.children = positions
            .iter()
            .map(|position| {
                let action = Action::new(self.color, ActionType::Move(*position));
                let next_board = self.board.apply_action(&action).unwrap();
                NegaMaxNode {
                    board: next_board,
                    color: self.color.opponent(),
                    move_count: self.move_count + 1,
                    last_action: action,
                    value: None,
                    children: Vec::new(),
                }
            })
            .collect::<Vec<_>>();
    }
}

pub trait NegaMaxEvaluationFunction {
    fn evaluate(&mut self, node: &NegaMaxNode) -> i32;
}

fn nega_max_search<E>(node: &mut NegaMaxNode, depth: usize, eval: &mut E) -> i32
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
            .map(|child| -nega_max_search(child, depth - 1, eval))
            .collect();

        let v = vs.iter().max_by(|a, b| a.cmp(b)).expect("no children");

        node.value = Some(*v);
        *v
    }
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
    fn search(&mut self, node: &mut NegaMaxNode, depth: usize) -> i32 {
        nega_max_search(node, depth, &mut self.eval)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::Board;
    use crate::{ActionType, Square};

    struct TestEvaluationFunction {}

    impl NegaMaxEvaluationFunction for TestEvaluationFunction {
        fn evaluate(&mut self, node: &NegaMaxNode) -> i32 {
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
        let mut nega_max = NegaMax {
            eval: TestEvaluationFunction {},
        };

        let mut root = NegaMaxNode {
            board: BitBoard::new_initial(),
            color: PlayerColor::Black,
            move_count: 0,
            last_action: Action::new(PlayerColor::White, ActionType::Pass),
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
