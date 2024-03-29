use crate::board::Board;
use crate::player::Player;
use crate::{Move, PlayerColor, Squares};

pub struct GameState {
    pub board: Squares,
    pub depth: u32,
    pub black_count: u32,
    pub white_count: u32,
    pub is_end: bool,
    pub turn: PlayerColor,
}

impl GameState {
    pub fn new<T>(board: &T) -> Self
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
        }
    }
}

pub struct GameResult<T>
where
    T: Board,
{
    pub state: GameState,
    pub history: Vec<T>,
    pub game_record: Vec<Move>,
}

pub fn play_game<T>(
    initial_board: &T,
    black_player: Box<dyn Player>,
    white_player: Box<dyn Player>,
) -> GameResult<T>
where
    T: Board,
{
    let mut board = initial_board.duplicate();
    let mut board_history: Vec<T> = Default::default();
    let mut game_record: Vec<Move> = Default::default();

    loop {
        let action = if board.turn() == PlayerColor::Black {
            black_player.take_action(&GameState::new(&board))
        } else {
            white_player.take_action(&GameState::new(&board))
        };

        if let Some(next_board) = board.apply_move(&action) {
            board_history.push(board.duplicate());
            game_record.push(action);
            board = next_board;

            if board.is_game_over() {
                break;
            }
        }
    }

    GameResult {
        state: GameState::new(&board),
        history: board_history,
        game_record,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::BitBoard;
    use crate::Move;
    use crate::Position;

    /// 左上優先で置けるところに置いていくプレイヤー
    struct Test1Player();

    impl Player for Test1Player {
        fn take_action(&self, state: &GameState) -> Move {
            let color = state.turn;
            let board = BitBoard::new(&state.board, state.depth);
            let positions = board.get_movable_positions(&color);

            if positions.is_empty() {
                return Move::new_pass(color);
            }

            Move::new_position(color, positions[0])
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
        fn take_action(&self, state: &GameState) -> Move {
            let color = state.turn;
            match state.depth {
                0 => Move::new_position(color, Position(4, 5)),
                1 => Move::new_position(color, Position(5, 5)),
                2 => Move::new_position(color, Position(5, 4)),
                3 => Move::new_position(color, Position(3, 5)),
                4 => Move::new_position(color, Position(2, 4)),
                5 => Move::new_position(color, Position(1, 3)),
                6 => Move::new_position(color, Position(2, 3)),
                7 => Move::new_position(color, Position(5, 3)),
                8 => Move::new_position(color, Position(3, 2)),
                9 => Move::new_position(color, Position(3, 1)),
                _ => Move::new_pass(color),
            }
        }
    }

    #[test]
    fn test_play_game() {
        {
            let board = BitBoard::new_initial();
            let black = Box::new(Test1Player {});
            let white = Box::new(Test1Player {});
            let result = play_game(&board, black, white);

            assert_eq!(64, result.state.depth);
            assert_eq!(19, result.state.black_count);
            assert_eq!(45, result.state.white_count);
        }

        {
            let board = BitBoard::new_initial();
            let black = Box::new(Test2Player::new());
            let white = Box::new(Test2Player::new());
            let result = play_game(&board, black, white);

            assert_eq!(10, result.state.depth);
            assert_eq!(0, result.state.black_count);
            assert_eq!(14, result.state.white_count);
        }
    }
}
