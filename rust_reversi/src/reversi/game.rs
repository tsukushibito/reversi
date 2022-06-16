use crate::board::Board;
use crate::player::Player;
use std::rc::Rc;

pub struct Game<T, U, V>
where
    T: Player,
    U: Player,
    V: Board,
{
    board: Rc<V>,
    black_player: T,
    white_player: U,
    depth: u32,
    board_history: Vec<Rc<V>>,
}

impl<T, U, V> Game<T, U, V>
where
    T: Player,
    U: Player,
    V: Board,
{
    pub fn new(initial_board: Rc<V>, black_player: T, white_player: U) -> Game<T, U, V> {
        Game {
            board: initial_board,
            black_player,
            white_player,
            depth: 0,
            board_history: Default::default(),
        }
    }

    pub fn run(&mut self) {
        loop {
            let action = if self.depth % 2 == 0 {
                self.black_player
                    .take_action(self.depth, self.board.squares())
            } else {
                self.white_player
                    .take_action(self.depth, self.board.squares())
            };

            if let Some(next_board) = self.board.apply_action(&action) {
                self.depth += 1;
                self.board_history.push(self.board.clone());
                self.board = Rc::new(next_board);

                if self.board.is_game_over() {
                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::ArrayBoard;
    use crate::board::IndexBoard;
    use crate::board::Indexer;
    use crate::Action;
    use crate::ActionType;
    use crate::PlayerColor;
    use crate::Position;
    use crate::Squares;
    use std::rc::Rc;

    struct Test1Player {
        indexer: Rc<Indexer>,
    }

    /// 左上優先で置けるところに置いていくプレイヤー
    impl Test1Player {
        fn new() -> Test1Player {
            Test1Player {
                indexer: Rc::new(Indexer::new()),
            }
        }
    }

    impl Player for Test1Player {
        fn take_action(&mut self, depth: u32, squares: &Squares) -> Action {
            let color = if depth % 2 == 0 {
                PlayerColor::Black
            } else {
                PlayerColor::White
            };

            let board = IndexBoard::new(*squares, self.indexer.clone());
            let positions = board.get_movable_positions(&color);

            if positions.is_empty() {
                return Action::new(color, ActionType::Pass);
            }

            Action::new(color, ActionType::Move(positions[0]))
        }
    }

    /// 最短でゲーム終了する手順を踏むプレイヤー
    struct Test2Player {}

    impl Test2Player {
        pub fn new() -> Test2Player {
            Test2Player {}
        }
    }

    impl Player for Test2Player {
        fn take_action(&mut self, depth: u32, _: &Squares) -> Action {
            let color = if depth % 2 == 0 {
                PlayerColor::Black
            } else {
                PlayerColor::White
            };
            match depth {
                0 => Action::new(color, ActionType::Move(Position(4, 5))),
                1 => Action::new(color, ActionType::Move(Position(5, 5))),
                2 => Action::new(color, ActionType::Move(Position(5, 4))),
                3 => Action::new(color, ActionType::Move(Position(3, 5))),
                4 => Action::new(color, ActionType::Move(Position(2, 4))),
                5 => Action::new(color, ActionType::Move(Position(1, 3))),
                6 => Action::new(color, ActionType::Move(Position(2, 3))),
                7 => Action::new(color, ActionType::Move(Position(5, 3))),
                8 => Action::new(color, ActionType::Move(Position(3, 2))),
                9 => Action::new(color, ActionType::Move(Position(3, 1))),
                _ => Action::new(color, ActionType::Pass),
            }
        }
    }

    #[test]
    fn test_index_board_run() {
        {
            let indexer = Rc::new(Indexer::new());
            let board = Rc::new(IndexBoard::new_initial(indexer));
            let black = Test1Player::new();
            let white = Test1Player::new();
            let mut reversi = Game::new(board, black, white);
            reversi.run();

            assert_eq!(64, reversi.depth);
            assert_eq!(19, reversi.board.black_count());
            assert_eq!(45, reversi.board.white_count());
        }

        {
            let indexer = Rc::new(Indexer::new());
            let board = Rc::new(IndexBoard::new_initial(indexer));
            let black = Test2Player::new();
            let white = Test2Player::new();
            let mut reversi = Game::new(board, black, white);
            reversi.run();

            assert_eq!(10, reversi.depth);
            assert_eq!(0, reversi.board.black_count());
            assert_eq!(14, reversi.board.white_count());
        }
    }

    #[test]
    fn test_array_board_run() {
        {
            let board = Rc::new(ArrayBoard::new_initial());
            let black = Test1Player::new();
            let white = Test1Player::new();
            let mut reversi = Game::new(board, black, white);
            reversi.run();

            assert_eq!(64, reversi.depth);
            assert_eq!(19, reversi.board.black_count());
            assert_eq!(45, reversi.board.white_count());
        }

        {
            let board = Rc::new(ArrayBoard::new_initial());
            let black = Test2Player::new();
            let white = Test2Player::new();
            let mut reversi = Game::new(board, black, white);
            reversi.run();

            assert_eq!(10, reversi.depth);
            assert_eq!(0, reversi.board.black_count());
            assert_eq!(14, reversi.board.white_count());
        }
    }
}
