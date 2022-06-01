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

    /// NegaMax法で評価
    /// evaluator: 評価関数
    /// depth: 読みの深さ
    /// return: (評価値, 次の手)
    pub fn evaluate<F>(&mut self, evaluator: &F, depth: usize) -> (i32, Option<Action>)
    where
        F: Fn(&T, &PlayerColor) -> i32,
    {
        if depth == 0 {
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

                // 子ノードを評価(NegaMaxなので符号反転)
                let values = self
                    .children
                    .iter_mut()
                    .map(|child| -child.evaluate(evaluator, depth - 1).0)
                    .collect::<Vec<_>>();

                // 子ノードの最大評価を自分の評価値とする
                self.value = *values.iter().max().unwrap();

                // 手の保存
                let mut value_actions = values.iter().zip(actions);
                self.action = Some(value_actions.find(|va| *va.0 == self.value).unwrap().1);
            } else {
                // パス時
                // ゲーム終了判定
                let opponent_movables = self.board.get_movable_positions(&opponent);
                if opponent_movables.len() == 0 {
                    // ゲーム終了なので評価処理実行
                    self.value = evaluator(&self.board, &self.player_color);
                } else {
                    // ボードをコピーして展開
                    self.children
                        .push(GameTreeNode::new(self.board.duplicate(), opponent, None));

                    // 評価
                    self.value = -(self.children[0].evaluate(evaluator, depth - 1).0);

                    // 手はパス
                    self.action = Some(Action::new(self.player_color, ActionType::Pass));
                }
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
    use crate::Square;
    use std::rc::Rc;

    #[test]
    fn test_game_tree_evaluate() {
        let indexer = Rc::new(Indexer::new());
        let board = IndexBoard::new_initial(indexer);
        let mut node = GameTreeNode::new(board, PlayerColor::Black, None);

        let value_action = node.evaluate(&simple_evaluator, 1);
        assert_eq!(value_action.0, -3);

        let act = Action::new(PlayerColor::Black, ActionType::Move(Position(2, 3)));
        assert_eq!(value_action.1.unwrap(), act);
    }
}
