use crate::board::Board;
use crate::player::Player;
use crate::{Action, PlayerColor, Squares};
use std::rc::Rc;

pub enum GameEvent {
    Started,
    TurnStarted,
    TurnEnded,
    GameOver,
}

pub struct GameEventParameter {
    pub board: Squares,
    pub depth: u32,
    pub black_count: u32,
    pub white_count: u32,
    pub is_end: bool,
    pub turn: PlayerColor,
    pub last_action: Option<Action>,
}

impl GameEventParameter {
    pub fn new<T>(board: &T, last_action: Option<Action>) -> Self
    where
        T: Board,
    {
        Self {
            board: *board.squares(),
            depth: board.depth(),
            black_count: board.black_count(),
            white_count: board.white_count(),
            is_end: board.is_game_over(),
            turn: board.turn(),
            last_action,
        }
    }
}

pub trait GameEventHandler {
    fn handle(&self, event: GameEvent, param: &GameEventParameter);
}

pub struct Game<T>
where
    T: Board,
{
    board: Rc<T>,
    last_action: Option<Action>,
    black_player: Box<dyn Player>,
    white_player: Box<dyn Player>,
    board_history: Vec<Rc<T>>,
    event_handler: Option<Box<dyn GameEventHandler>>,
}

impl<T> Game<T>
where
    T: Board,
{
    pub fn new(
        initial_board: Rc<T>,
        black_player: Box<dyn Player>,
        white_player: Box<dyn Player>,
        event_handler: Option<Box<dyn GameEventHandler>>,
    ) -> Game<T> {
        Game {
            board: initial_board,
            last_action: None,
            black_player,
            white_player,
            board_history: Default::default(),
            event_handler,
        }
    }

    pub fn run(&mut self) {
        if let Some(event_handler) = &self.event_handler {
            event_handler.handle(
                GameEvent::Started,
                &GameEventParameter::new(&(*self.board), self.last_action),
            );
        }

        loop {
            if let Some(event_handler) = &self.event_handler {
                event_handler.handle(
                    GameEvent::TurnStarted,
                    &GameEventParameter::new(&(*self.board), self.last_action),
                );
            }

            let action = if self.board.turn() == PlayerColor::Black {
                self.black_player
                    .take_action(&GameEventParameter::new(&(*self.board), self.last_action))
            } else {
                self.white_player
                    .take_action(&GameEventParameter::new(&(*self.board), self.last_action))
            };

            if let Some(event_handler) = &self.event_handler {
                event_handler.handle(
                    GameEvent::TurnEnded,
                    &GameEventParameter::new(&(*self.board), self.last_action),
                );
            }

            if let Some(next_board) = self.board.apply_action(&action) {
                self.last_action = Some(action);
                self.board_history.push(self.board.clone());
                self.board = Rc::new(next_board);

                if self.board.is_game_over() {
                    break;
                }
            }
        }

        if let Some(event_handler) = &self.event_handler {
            event_handler.handle(
                GameEvent::GameOver,
                &GameEventParameter::new(&(*self.board), self.last_action),
            );
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
    use crate::Position;
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
        fn take_action(&mut self, state: &GameEventParameter) -> Action {
            let color = state.turn;
            let board = IndexBoard::new(state.board, state.depth, self.indexer.clone());
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
        fn take_action(&mut self, state: &GameEventParameter) -> Action {
            let color = state.turn;
            match state.depth {
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
            let black = Box::new(Test1Player::new());
            let white = Box::new(Test1Player::new());
            let mut reversi = Game::new(board, black, white, None);
            reversi.run();

            assert_eq!(64, reversi.board.depth());
            assert_eq!(19, reversi.board.black_count());
            assert_eq!(45, reversi.board.white_count());
        }

        {
            let indexer = Rc::new(Indexer::new());
            let board = Rc::new(IndexBoard::new_initial(indexer));
            let black = Box::new(Test2Player::new());
            let white = Box::new(Test2Player::new());
            let mut reversi = Game::new(board, black, white, None);
            reversi.run();

            assert_eq!(10, reversi.board.depth());
            assert_eq!(0, reversi.board.black_count());
            assert_eq!(14, reversi.board.white_count());
        }
    }

    #[test]
    fn test_array_board_run() {
        {
            let board = Rc::new(ArrayBoard::new_initial());
            let black = Box::new(Test1Player::new());
            let white = Box::new(Test1Player::new());
            let mut reversi = Game::new(board, black, white, None);
            reversi.run();

            assert_eq!(64, reversi.board.depth());
            assert_eq!(19, reversi.board.black_count());
            assert_eq!(45, reversi.board.white_count());
        }

        {
            let board = Rc::new(ArrayBoard::new_initial());
            let black = Box::new(Test2Player::new());
            let white = Box::new(Test2Player::new());
            let mut reversi = Game::new(board, black, white, None);
            reversi.run();

            assert_eq!(10, reversi.board.depth());
            assert_eq!(0, reversi.board.black_count());
            assert_eq!(14, reversi.board.white_count());
        }
    }
}
