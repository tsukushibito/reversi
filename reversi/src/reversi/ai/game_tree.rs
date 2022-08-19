use crate::board::BitBoard;
use crate::board::Board;
use crate::Action;
use crate::ActionType;
use crate::PlayerColor;
use crate::Squares;

use super::Evaluator;

#[derive(Debug)]
pub enum SearchType {
    NegaMax,
    NegaAlpha,
}

#[derive(Debug)]
struct GameTreeNode<T> {
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
    pub fn new(board: &T, color: &PlayerColor, action: Option<Action>) -> GameTreeNode<T> {
        GameTreeNode {
            board: board.duplicate(),
            player_color: *color,
            value: 0,
            action,
            children: Default::default(),
        }
    }

    pub fn search<E>(
        &mut self,
        search_type: &SearchType,
        depth: usize,
        searched_nodes: &mut usize,
    ) -> (i32, Option<Action>)
    where
        E: Evaluator,
    {
        match search_type {
            SearchType::NegaAlpha => {
                self.nega_alpha::<E>(depth, i32::MIN + 1, i32::MAX, searched_nodes)
            }
            SearchType::NegaMax => self.nega_max::<E>(depth, searched_nodes),
        }
    }

    fn is_leaf(&self, depth: usize) -> bool {
        depth == 0 || self.board.is_game_over()
    }

    fn expand(&mut self, actions: &[Action]) {
        // 展開
        for act in actions {
            let next = self.board.apply_action(act);
            self.children.push(GameTreeNode::new(
                &next.unwrap(),
                &self.player_color.opponent(),
                None,
            ));
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
        searched_nodes: &mut usize,
    ) -> (i32, Option<Action>)
    where
        // E: Fn(&Squares, &PlayerColor) -> i32,
        E: Evaluator,
    {
        self.children.clear();
        *searched_nodes += 1;
        if self.is_leaf(depth) {
            self.value = E::evaluate(self.board.squares(), &self.player_color)
        } else {
            let positions = self.board.get_movable_positions(&self.player_color);
            if !positions.is_empty() {
                let actions = positions
                    .iter()
                    .map(|p| Action::new(self.player_color, ActionType::Move(*p)))
                    .collect::<Vec<_>>();

                self.expand(&actions);

                // NegaAlphaで評価
                let mut alpha = alpha;
                let mut index = 0;
                for (i, child) in self.children.iter_mut().enumerate() {
                    let v = -child
                        .nega_alpha::<E>(depth - 1, -beta, -alpha, searched_nodes)
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
                let actions = vec![Action::new(self.player_color, ActionType::Pass)];
                self.expand(&actions);

                // 評価
                self.value = -(self.children[0]
                    .nega_alpha::<E>(depth - 1, -beta, -alpha, searched_nodes)
                    .0);

                self.action = Some(actions[0]);
            }
        }
        (self.value, self.action)
    }

    /// NegaMax方による探索
    /// evaluator: 評価関数
    /// depth: 読みの深さ
    /// alpha: α値
    /// beta: ベータ値
    /// return: (評価値, 次の手)
    fn nega_max<E>(&mut self, depth: usize, searched_nodes: &mut usize) -> (i32, Option<Action>)
    where
        E: Evaluator,
    {
        *searched_nodes += 1;
        if self.is_leaf(depth) {
            self.value = E::evaluate(self.board.squares(), &self.player_color);
        } else {
            let positions = self.board.get_movable_positions(&self.player_color);
            if !positions.is_empty() {
                let actions = positions
                    .iter()
                    .map(|p| Action::new(self.player_color, ActionType::Move(*p)))
                    .collect::<Vec<_>>();

                self.expand(&actions);

                let mut value = i32::MIN + 1;
                let mut index = 0;
                for (i, child) in self.children.iter_mut().enumerate() {
                    let v = -child.nega_max::<E>(depth - 1, searched_nodes).0;
                    if v > value {
                        value = v;
                        index = i;
                    }
                }

                self.value = value;
                self.action = Some(actions[index]);
            } else {
                // パス時
                let actions = vec![Action::new(self.player_color, ActionType::Pass)];
                self.expand(&actions);

                // 評価
                self.value = -(self.children[0].nega_max::<E>(depth - 1, searched_nodes).0);

                self.action = Some(actions[0]);
            }
        }
        (self.value, self.action)
    }
}

pub struct SearchResult {
    pub value: i32,
    pub action: Option<Action>,
    pub searched_nodes: usize,
}

pub fn search_game_tree<E>(
    board: &Squares,
    color: &PlayerColor,
    search_type: &SearchType,
    depth: usize,
) -> SearchResult
where
    E: Evaluator,
{
    let board = BitBoard::new(board, 0);
    let mut root = GameTreeNode::new(&board, color, None);
    let mut searched_nodes = 0;
    let (value, action) = root.search::<E>(search_type, depth, &mut searched_nodes);
    SearchResult {
        value,
        action,
        searched_nodes,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai::SimpleEvaluator;
    use crate::board::ArrayBoard;
    use crate::board::IndexBoard;
    use crate::board::Indexer;
    use crate::Action;
    use crate::ActionType;
    use crate::PlayerColor;
    use crate::Position;
    use std::rc::Rc;

    #[test]
    fn test_game_tree_negaalpha_search() {
        let indexer = Rc::new(Indexer::new());
        let board = IndexBoard::new_initial(indexer);
        let mut node = GameTreeNode::new(&board, &PlayerColor::Black, None);

        let mut searched_nodes: usize = 0;
        let value_action =
            node.search::<SimpleEvaluator>(&SearchType::NegaAlpha, 2, &mut searched_nodes);

        assert_eq!(value_action.0, -1);

        let act = Action::new(PlayerColor::Black, ActionType::Move(Position(2, 3)));
        assert_eq!(value_action.1.unwrap(), act);
    }

    #[test]
    fn test_game_tree_negamax_search() {
        let indexer = Rc::new(Indexer::new());
        let board = IndexBoard::new_initial(indexer);
        let mut node = GameTreeNode::new(&board, &PlayerColor::Black, None);

        let mut searched_nodes: usize = 0;
        let value_action =
            node.search::<SimpleEvaluator>(&SearchType::NegaMax, 2, &mut searched_nodes);

        assert_eq!(value_action.0, -1);

        let act = Action::new(PlayerColor::Black, ActionType::Move(Position(2, 3)));
        assert_eq!(value_action.1.unwrap(), act);
    }
    #[test]
    fn test_search_game_tree() {
        let board = ArrayBoard::new_initial();
        let squares = board.squares();
        let depth = 7;

        let nega_max_result = search_game_tree::<SimpleEvaluator>(
            squares,
            &PlayerColor::Black,
            &SearchType::NegaMax,
            depth,
        );
        assert!(nega_max_result.action != None);
        println!(
            "[NegaMax] depth: {},  searched_nodes: {}",
            depth, nega_max_result.searched_nodes
        );

        let nega_alpha_result = search_game_tree::<SimpleEvaluator>(
            squares,
            &PlayerColor::Black,
            &SearchType::NegaAlpha,
            depth,
        );
        assert!(nega_alpha_result.action != None);
        println!(
            "[NegaAlpha] depth: {},  searched_nodes: {}",
            depth, nega_alpha_result.searched_nodes
        );
        assert!(nega_alpha_result.searched_nodes < nega_max_result.searched_nodes);
    }
}
