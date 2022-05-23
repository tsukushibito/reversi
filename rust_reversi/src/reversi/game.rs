use crate::player::Player;
use crate::ActionType;
use crate::Board;
use std::rc::Rc;

pub struct Game<T, U, V>
where
    T: Player,
    U: Player,
    V: Board<V>,
{
    board: Rc<V>,
    black_player: T,
    white_player: U,
    depth: u32,
    has_passed: bool,
    board_history: Vec<Rc<V>>,
}

impl<T, U, V> Game<T, U, V>
where
    T: Player,
    U: Player,
    V: Board<V>,
{
    pub fn new(initial_board: Rc<V>, black_player: T, white_player: U) -> Game<T, U, V> {
        Game {
            board: initial_board,
            black_player: black_player,
            white_player: white_player,
            depth: 0,
            has_passed: false,
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
                if let ActionType::Pass = action.action {
                    if self.has_passed {
                        // 2連続パスなのでゲーム終了
                        break;
                    }
                    self.has_passed = true;
                } else {
                    self.has_passed = false;
                }

                self.depth += 1;
                self.board_history.push(self.board.clone());
                self.board = next_board;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::index_board::IndexBoard;
    use crate::indexer::Indexer;
    use crate::Action;
    use crate::Position;
    use crate::Square;
    use crate::Squares;
    use std::rc::Rc;

    struct Test1Player {
        board: IndexBoard,
    }

    impl Test1Player {
        fn new() -> Test1Player {
            let indexer = Rc::new(Indexer::new());
            Test1Player {
                board: IndexBoard::new_initial(indexer),
            }
        }
    }

    impl Player for Test1Player {
        fn take_action(&mut self, depth: u32, squares: &Squares) -> Action {
            let color = if depth % 2 == 0 {
                Square::Black
            } else {
                Square::White
            };

            self.board.squares = squares.clone();
            let positions = self.board.get_movable_positions(color);

            if positions.len() == 0 {
                return Action::new(color, ActionType::Pass);
            }

            Action::new(color, ActionType::Move(positions[0]))
        }
    }

    struct Test2Player {}

    impl Test2Player {
        pub fn new() -> Test2Player {
            Test2Player {}
        }
    }

    impl Player for Test2Player {
        fn take_action(&mut self, depth: u32, _: &Squares) -> Action {
            let color = if depth % 2 == 0 {
                Square::Black
            } else {
                Square::White
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
    fn test_run() {
        let indexer = Rc::new(Indexer::new());
        let board = Rc::new(IndexBoard::new_initial(indexer));
        let black = Test1Player::new();
        let white = Test1Player::new();
        let mut reversi = Game::new(board, black, white);
        reversi.run();

        let indexer = Rc::new(Indexer::new());
        let board = Rc::new(IndexBoard::new_initial(indexer));
        let black = Test2Player::new();
        let white = Test2Player::new();
        let mut reversi = Game::new(board, black, white);
        reversi.run();

        assert_eq!(11, reversi.depth)
    }
}