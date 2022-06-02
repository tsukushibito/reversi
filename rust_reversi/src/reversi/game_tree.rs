use crate::Action;
use crate::ActionType;
use crate::Board;
use crate::PlayerColor;

pub struct GameTreeNode<T>
where
    T: Board<T>,
{
    board: T,
    player_color: PlayerColor,
    value: i32,
    action: Option<Action>,
    children: Vec<GameTreeNode<T>>,
}

impl<T> GameTreeNode<T>
where
    T: Board<T>,
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

    pub fn search<F>(
        &mut self,
        evaluator: &F,
        depth: usize,
        visited_count: &mut usize,
    ) -> (i32, Option<Action>)
    where
        F: Fn(&T, &PlayerColor) -> i32,
    {
        self.nega_alpha(evaluator, depth, i32::MIN + 1, i32::MAX, visited_count)
    }

    /// NegaAlpha法で評価
    /// evaluator: 評価関数
    /// depth: 読みの深さ
    /// alpha: α値
    /// beta: ベータ値
    /// return: (評価値, 次の手)
    fn nega_alpha<F>(
        &mut self,
        evaluator: &F,
        depth: usize,
        alpha: i32,
        beta: i32,
        visited_count: &mut usize,
    ) -> (i32, Option<Action>)
    where
        F: Fn(&T, &PlayerColor) -> i32,
    {
        *visited_count += 1;
        if depth == 0 || self.board.is_game_over() {
            // リーフノードなので評価
            self.value = evaluator(&self.board, &self.player_color);
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
                        .nega_alpha(evaluator, depth - 1, -beta, -alpha, visited_count)
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
                    .nega_alpha(evaluator, depth - 1, -beta, -alpha, visited_count)
                    .0);

                // 手はパス
                self.action = Some(Action::new(self.player_color, ActionType::Pass));
            }
        }
        (self.value, self.action)
    }
}

#[cfg(test)]
mod tests {
    use crate::evaluator::simple_evaluator;
    use crate::game_tree::GameTreeNode;
    use crate::index_board::IndexBoard;
    use crate::indexer::Indexer;
    use crate::Action;
    use crate::ActionType;
    use crate::PlayerColor;
    use crate::Position;
    use std::rc::Rc;

    #[test]
    fn test_game_tree_evaluate() {
        let indexer = Rc::new(Indexer::new());
        let board = IndexBoard::new_initial(indexer.clone());
        let mut node = GameTreeNode::new(board, PlayerColor::Black, None);

        let mut visited_count: usize = 0;
        let value_action = node.search(&simple_evaluator, 2, &mut visited_count);

        assert_eq!(value_action.0, -1);

        let act = Action::new(PlayerColor::Black, ActionType::Move(Position(2, 3)));
        assert_eq!(value_action.1.unwrap(), act);
    }
}
