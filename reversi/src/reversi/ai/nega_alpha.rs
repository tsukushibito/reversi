use crate::{
    board::{BitBoard, Board},
    Action, ActionType, PlayerColor,
};

pub struct NegaAlphaNode {
    pub board: BitBoard,
    pub color: PlayerColor,
    pub move_count: u8,
    pub last_action: Action,
    pub value: Option<i32>,
    pub children: Vec<NegaAlphaNode>,
}

impl NegaAlphaNode {
    pub fn expand(&mut self) {
        let positions = self.board.get_movable_positions(&self.color);
        self.children = positions
            .iter()
            .map(|position| {
                let action = Action::new(self.color, ActionType::Move(*position));
                let next_board = self.board.apply_action(&action).unwrap();
                NegaAlphaNode {
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

    pub fn node_count(&self) -> usize {
        self.children
            .iter()
            .fold(1, |acc, child| acc + child.node_count())
    }

    pub fn searched_nodes(&self) -> usize {
        self.children
            .iter()
            .filter(|child| child.value.is_some())
            .fold(1, |acc, child| acc + child.searched_nodes())
    }

    pub fn candidate(&self) -> Option<Vec<Action>> {
        if self.children.is_empty() {
            None
        } else {
            let mut children = self
                .children
                .iter()
                .map(|child| (child.value, child.last_action))
                .collect::<Vec<_>>();
            children.sort_by(|a, b| b.0.cmp(&a.0));
            Some(
                children
                    .iter()
                    .map(|(_, action)| *action)
                    .collect::<Vec<_>>(),
            )
        }
    }
}

pub trait NegaAlphaEvaluationFunction {
    fn evaluate(&mut self, node: &NegaAlphaNode) -> i32;
}

fn nega_max_search<E>(node: &mut NegaAlphaNode, depth: usize, eval: &mut E) -> i32
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
            .map(|child| -nega_max_search(child, depth - 1, eval))
            .collect();

        let v = vs.iter().max_by(|a, b| a.cmp(b)).expect("no children");

        node.value = Some(*v);
        *v
    }
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
    fn search(&mut self, node: &mut NegaAlphaNode, depth: usize) -> i32 {
        nega_max_search(node, depth, &mut self.eval)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::Board;
    use crate::{ActionType, Square};

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
