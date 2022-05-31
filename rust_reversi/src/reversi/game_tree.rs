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
}
