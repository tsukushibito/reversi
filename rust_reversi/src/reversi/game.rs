use super::board::Board;
use super::player::Player;

pub struct Game<T, U>
where
    T: Player,
    U: Player,
{
    board: Board,
    black_player: T,
    white_player: U,
    depth: u32,
    has_passed: bool,
}

impl<T, U> Game<T, U>
where
    T: Player,
    U: Player,
{
    pub fn new(black_player: T, white_player: U) -> Game<T, U> {
        let board = Board::new_initial();
        Game {
            board: board,
            black_player: black_player,
            white_player: white_player,
            depth: 0,
            has_passed: false,
        }
    }

    pub fn run(&mut self) {
        loop {
            let action = if self.depth % 2 == 0 {
                self.black_player
                    .take_action(self.depth, &self.board.squares)
            } else {
                self.white_player
                    .take_action(self.depth, &self.board.squares)
            };

            match self.board.apply_action(&action) {
                Some(next_board) => {
                    if action.pass {
                        if self.has_passed {
                            // 2連続パスなのでゲーム終了
                            break;
                        }
                        self.has_passed = true;
                    } else {
                        self.has_passed = false;
                    }

                    self.depth += 1;
                    self.board = next_board;
                }
                None => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::action::Action;
    use super::super::board::Square;
    use super::super::board::Squares;
    use super::*;

    struct Test1Player {
        board: Board,
    }

    impl Test1Player {
        fn new() -> Test1Player {
            Test1Player {
                board: Board::new_initial(),
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
                return Action::new_pass(color);
            }

            let (r, c) = positions[0];
            Action::new_move(color, r, c)
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
                0 => Action::new_move(color, 4, 5),
                1 => Action::new_move(color, 5, 5),
                2 => Action::new_move(color, 5, 4),
                3 => Action::new_move(color, 3, 5),
                4 => Action::new_move(color, 2, 4),
                5 => Action::new_move(color, 1, 3),
                6 => Action::new_move(color, 2, 3),
                7 => Action::new_move(color, 5, 3),
                8 => Action::new_move(color, 3, 2),
                9 => Action::new_move(color, 3, 1),
                _ => Action::new_pass(color),
            }
        }
    }

    #[test]
    fn test_run() {
        let black = Test1Player::new();
        let white = Test1Player::new();
        let mut reversi = Game::new(black, white);
        reversi.run();

        let black = Test2Player::new();
        let white = Test2Player::new();
        let mut reversi = Game::new(black, white);
        reversi.run();

        assert_eq!(11, reversi.depth)
    }
}
