use std::marker::PhantomData;

use crate::{board::Board, Action, ActionType, PlayerColor, Squares};

use super::{EvaluationFunction, GameTreeNode, SearchFunction, SearchResult};

pub struct NegaMaxSearch {}

impl SearchFunction for NegaMaxSearch {
    fn search<N, E>(node: &mut N, depth: usize, eval: &E) -> (i32, Action)
    where
        N: GameTreeNode,
        E: EvaluationFunction,
    {
        if node.is_leaf() || depth == 0 {
            let result = E::evaluate(node.board().squares(), &node.color());
            result
        } else {
            let positions = node.board().get_movable_positions(&node.color());
            let mut actions: Vec<Action>;
            if !positions.is_empty() {
                actions = positions
                    .iter()
                    .map(|p| Action::new(*node.color(), ActionType::Move(*p)))
                    .collect::<Vec<_>>();

                node.expand(&actions);
            } else {
                // パス時
                actions = vec![Action::new(*node.color(), ActionType::Pass)];
                node.expand(&actions);
            }

            let mut value = i32::MIN + 1;
            let mut index = 0;
            for (i, child) in node.children_mut().iter_mut().enumerate() {
                let v = -Self::search(child, depth - 1, eval).0;
                if v > value {
                    value = v;
                    index = i;
                }
            }

            (value, actions[index])
        }
    }
}
