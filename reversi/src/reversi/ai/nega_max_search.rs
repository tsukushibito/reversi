use std::marker::PhantomData;

use crate::{board::Board, Action, ActionType, PlayerColor, Squares};

use super::{EvaluationFunction, GameTreeNode, SearchFunction, SearchResult};

pub struct NegaMaxSearch {}

impl SearchFunction for NegaMaxSearch {
    fn search<Node, Eval>(node: &mut Node, depth: usize) -> SearchResult
    where
        Node: GameTreeNode,
        Eval: EvaluationFunction,
    {
        if node.is_leaf() || depth == 0 {
            let result = Eval::evaluate(node.board().squares(), &node.color());
            SearchResult {
                value: result.0,
                action: result.1,
                searched_nodes: 0,
            }
        } else {
            let positions = node.board().get_movable_positions(&node.color());
            if !positions.is_empty() {
                let actions = positions
                    .iter()
                    .map(|p| Action::new(*node.color(), ActionType::Move(*p)))
                    .collect::<Vec<_>>();

                node.expand(&actions);

                let mut value = i32::MIN + 1;
                let mut index = 0;
                for (i, child) in node.children().iter_mut().enumerate() {
                    let v = -Self::search(child, depth - 1).value;
                    if v > value {
                        value = v;
                        index = i;
                    }
                }

                SearchResult {
                    value,
                    action: actions[index],
                    searched_nodes: 0,
                }
            } else {
                // パス時
                let actions = vec![Action::new(*node.color(), ActionType::Pass)];
                node.expand(&actions);

                // 評価
                let value = -Self::search(node, depth - 1).value;

                let action = actions[0];
                SearchResult {
                    value,
                    action,
                    searched_nodes: 0,
                }
            }
        }
    }
}
