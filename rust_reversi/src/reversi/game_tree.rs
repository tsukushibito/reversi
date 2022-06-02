use crate::board::Board;
use crate::evaluator::Eval;
use crate::Action;
use crate::ActionType;
use crate::PlayerColor;
use std::collections::HashMap;
use std::marker::PhantomData;

#[derive(Debug)]
pub enum SearchType {
    NegaMax,
    NegaAlpha,
}

#[derive(Debug)]
pub struct GameTreeNode<T> {
    board: T,
    player_color: PlayerColor,
    value: i32,
    action: Option<Action>,
    children: Vec<GameTreeNode<T>>,
}

impl<T> GameTreeNode<T>
where
    T: Board,
{
    pub fn new(board: T, color: PlayerColor, action: Option<Action>) -> GameTreeNode<T> {
        GameTreeNode {
            board: board,
            player_color: color,
            value: 0,
            action: action,
            children: Default::default(),
        }
    }

    pub fn search<E>(
        &mut self,
        search_type: &SearchType,
        depth: usize,
        visited_count: &mut usize,
    ) -> (i32, Option<Action>)
    where
        E: Eval<BoardType = T>,
    {
        match search_type {
            SearchType::NegaAlpha => {
                self.nega_alpha::<E>(depth, i32::MIN + 1, i32::MAX, visited_count)
            }
            SearchType::NegaMax => self.nega_max::<E>(depth, visited_count),
        }
    }

    /// NegaAlpha法で評価
    /// evaluator: 評価関数
    /// depth: 読みの深さ
    /// alpha: α値
    /// beta: ベータ値
    /// return: (評価値, 次の手)
    fn nega_alpha<E>(
        &mut self,
        depth: usize,
        alpha: i32,
        beta: i32,
        visited_count: &mut usize,
    ) -> (i32, Option<Action>)
    where
        E: Eval<BoardType = T>,
    {
        *visited_count += 1;
        if depth == 0 || self.board.is_game_over() {
            // リーフノードなので評価
            self.value = E::evaluate(&self.board, &self.player_color);
        } else {
            // ノードの処理
            let opponent = if self.player_color == PlayerColor::Black {
                PlayerColor::White
            } else {
                PlayerColor::Black
            };

            let positions = self.board.get_movable_positions(&self.player_color);
            if positions.len() > 0 {
                let actions = positions
                    .iter()
                    .map(|p| Action::new(self.player_color, ActionType::Move(*p)))
                    .collect::<Vec<_>>();

                // 展開
                for act in &actions {
                    let next = self.board.apply_action(&act);
                    self.children
                        .push(GameTreeNode::new(next.unwrap(), opponent, None));
                }

                // NegaAlphaで評価
                let mut alpha = alpha;
                let mut index = 0;
                for (i, child) in self.children.iter_mut().enumerate() {
                    let v = -child
                        .nega_alpha::<E>(depth - 1, -beta, -alpha, visited_count)
                        .0;
                    if v >= beta {
                        break;
                    }
                    if v > alpha {
                        alpha = v;
                        index = i;
                    }
                }
                self.value = alpha;
                self.action = Some(actions[index]);
            } else {
                // パス時
                // ボードをコピーして展開
                self.children
                    .push(GameTreeNode::new(self.board.duplicate(), opponent, None));

                // 評価
                self.value = -(self.children[0]
                    .nega_alpha::<E>(depth - 1, -beta, -alpha, visited_count)
                    .0);

                // 手はパス
                self.action = Some(Action::new(self.player_color, ActionType::Pass));
            }
        }
        (self.value, self.action)
    }

    fn nega_max<E>(&mut self, depth: usize, visited_count: &mut usize) -> (i32, Option<Action>)
    where
        E: Eval<BoardType = T>,
    {
        *visited_count += 1;
        if depth == 0 || self.board.is_game_over() {
            // リーフノードなので評価
            self.value = E::evaluate(&self.board, &self.player_color);
        } else {
            // ノードの処理
            let opponent = if self.player_color == PlayerColor::Black {
                PlayerColor::White
            } else {
                PlayerColor::Black
            };

            let positions = self.board.get_movable_positions(&self.player_color);
            if positions.len() > 0 {
                let actions = positions
                    .iter()
                    .map(|p| Action::new(self.player_color, ActionType::Move(*p)))
                    .collect::<Vec<_>>();

                // 展開
                for act in &actions {
                    let next = self.board.apply_action(&act);
                    self.children
                        .push(GameTreeNode::new(next.unwrap(), opponent, None));
                }

                let mut value = i32::MIN + 1;
                let mut index = 0;
                for (i, child) in self.children.iter_mut().enumerate() {
                    let v = -child.nega_max::<E>(depth - 1, visited_count).0;
                    if v > value {
                        value = v;
                        index = i;
                    }
                }
                self.value = value;
                self.action = Some(actions[index]);
            } else {
                // パス時
                // ボードをコピーして展開
                self.children
                    .push(GameTreeNode::new(self.board.duplicate(), opponent, None));

                // 評価
                self.value = -(self.children[0].nega_max::<E>(depth - 1, visited_count).0);

                // 手はパス
                self.action = Some(Action::new(self.player_color, ActionType::Pass));
            }
        }
        (self.value, self.action)
    }
}

#[derive(Debug)]
pub struct GameTree<T> {
    root: GameTreeNode<T>,
    visited_count: usize,
    hash_table: HashMap<u64, i32>,
}

impl<T> GameTree<T>
where
    T: Board,
{
    fn new(board: T, color: PlayerColor) -> GameTree<T> {
        GameTree {
            root: GameTreeNode::new(board, color, None),
            visited_count: 0,
            hash_table: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::IndexBoard;
    use crate::board::Indexer;
    use crate::evaluator::SimpleEvaluator;
    use crate::Action;
    use crate::ActionType;
    use crate::PlayerColor;
    use crate::Position;
    use std::rc::Rc;

    #[test]
    fn test_game_tree_negaalpha_search() {
        let indexer = Rc::new(Indexer::new());
        let board = IndexBoard::new_initial(indexer.clone());
        let mut node = GameTreeNode::new(board, PlayerColor::Black, None);

        let mut visited_count: usize = 0;
        let value_action = node.search::<SimpleEvaluator<IndexBoard>>(
            &SearchType::NegaAlpha,
            2,
            &mut visited_count,
        );

        assert_eq!(value_action.0, -1);

        let act = Action::new(PlayerColor::Black, ActionType::Move(Position(2, 3)));
        assert_eq!(value_action.1.unwrap(), act);
    }

    #[test]
    fn test_game_tree_negamax_search() {
        let indexer = Rc::new(Indexer::new());
        let board = IndexBoard::new_initial(indexer.clone());
        let mut node = GameTreeNode::new(board, PlayerColor::Black, None);

        let mut visited_count: usize = 0;
        let value_action =
            node.search::<SimpleEvaluator<IndexBoard>>(&SearchType::NegaMax, 2, &mut visited_count);

        assert_eq!(value_action.0, -1);

        let act = Action::new(PlayerColor::Black, ActionType::Move(Position(2, 3)));
        assert_eq!(value_action.1.unwrap(), act);
    }
}
