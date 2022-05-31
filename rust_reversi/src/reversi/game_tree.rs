use crate::Action;
use crate::ActionType;
use crate::Board;
use crate::PlayerColor;

struct GameTreeNode<T>
where
    T: Board<T>,
{
    board: T,
    player_color: PlayerColor,
    value: i32,
    children: Vec<GameTreeNode<T>>,
}

impl<T> GameTreeNode<T>
where
    T: Board<T>,
{
    pub fn new(board: T, color: PlayerColor) -> GameTreeNode<T> {
        GameTreeNode {
            board: board,
            player_color: color,
            value: 0,
            children: Default::default(),
        }
    }

    /// NegaMax法で評価
    /// evaluator: 評価関数
    /// depth: 読みの深さ
    pub fn evaluate<F>(&mut self, evaluator: &F, depth: usize) -> i32
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

            let movables = self.board.get_movable_positions(&self.player_color);
            if movables.len() > 0 {
                // 展開
                for movable in movables {
                    let next = self
                        .board
                        .apply_action(&Action::new(self.player_color, ActionType::Move(movable)));
                    self.children
                        .push(GameTreeNode::new(next.unwrap(), opponent));
                }

                let value = self
                    .children
                    .iter_mut()
                    .map(|child| -child.evaluate(evaluator, depth - 1))
                    .max();

                self.value = value.unwrap();
            } else {
                let opponent_movables = self.board.get_movable_positions(&opponent);
                if opponent_movables.len() == 0 {
                    // ゲーム終了なので評価処理実行
                    self.value = evaluator(&self.board, &self.player_color);
                } else {
                    // ボードをコピーして展開
                    self.children
                        .push(GameTreeNode::new(self.board.duplicate(), opponent));

                    // 評価
                    self.value = -self.children[0].evaluate(evaluator, depth - 1);
                }
            }
        }
        self.value
    }
    pub fn get_action(&self) -> Option<Action> {
        if self.children.len() == 0 {
            return None;
        }

        let (max_index, _) = self
            .children
            .iter()
            .map(|child| child.value)
            .enumerate()
            .fold((usize::MIN, i32::MIN), |(i_a, a), (i_b, b)| {
                if b > a {
                    (i_b, b)
                } else {
                    (i_a, a)
                }
            });

        let positions = self.board.get_movable_positions(&self.player_color);

        Some(Action::new(
            self.player_color,
            ActionType::Move(positions[max_index]),
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::game_tree::GameTreeNode;
    use crate::index_board::IndexBoard;
    use crate::indexer::Indexer;
    use crate::PlayerColor;
    use crate::Square;
    use std::rc::Rc;

    #[test]
    fn test_game_tree_evaluate() {
        let indexer = Rc::new(Indexer::new());
        let board = IndexBoard::new_initial(indexer);
        let mut node = GameTreeNode::new(board, PlayerColor::Black);
        let evaluator = |board: &IndexBoard, color: &PlayerColor| {
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
                .squares
                .iter()
                .flatten()
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
        };

        let v = node.evaluate(&evaluator, 1);
        assert_eq!(v, -3);

        let act = node.get_action();
        assert_ne!(act, None);
    }
}
