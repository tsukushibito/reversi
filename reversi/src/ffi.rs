use std::rc::Rc;

use crate::{
    board::BitBoard,
    game::{Game, GameEvent, GameEventHandler, GameEventParameter},
    player::{AiPlayer, Player},
    Action, ActionType, PlayerColor, Position, BOARD_SIZE,
};

struct ReversiGameEventHandler {
    pub game_event_handler: extern "C" fn(ReversiGameEvent, &ReversiGameEventParam),
}

impl GameEventHandler for ReversiGameEventHandler {
    fn handle(&self, event: GameEvent, param: &GameEventParameter) {
        let e = match event {
            GameEvent::Started => ReversiGameEvent::Started,
            GameEvent::TurnStarted => ReversiGameEvent::TurnStarted,
            GameEvent::TurnEnded => ReversiGameEvent::TurnEnded,
            GameEvent::GameOver => ReversiGameEvent::GameOver,
        };

        let p = game_event_param_to_reversi_game_event_param(param);
        (self.game_event_handler)(e, &p);
    }
}

struct FfiPlayer {
    pub action_func: extern "C" fn(&ReversiGameEventParam) -> ReversiAction,
}

impl Player for FfiPlayer {
    fn take_action(&mut self, param: &GameEventParameter) -> Action {
        let p = game_event_param_to_reversi_game_event_param(param);

        let action = (self.action_func)(&p);

        if action.is_none {
            panic!()
        } else if action.is_pass {
            Action::new(param.turn, ActionType::Pass)
        } else {
            Action::new(
                param.turn,
                ActionType::Move(Position(action.move_pos_row, action.move_pos_col)),
            )
        }
    }
}

#[repr(C)]
pub enum ReversiGameEvent {
    Started,
    TurnStarted,
    TurnEnded,
    GameOver,
}

#[repr(C)]
pub enum ReversiPlayerColor {
    Black,
    White,
}

#[repr(C)]
pub struct ReversiGameEventParam {
    pub board: [[u32; BOARD_SIZE]; BOARD_SIZE],
    pub depth: u32,
    pub black_count: u32,
    pub white_count: u32,
    pub is_end: bool,
    pub turn: ReversiPlayerColor,
    pub last_action: ReversiAction,
}

#[repr(C)]
pub struct ReversiAction {
    pub is_none: bool,
    pub turn: ReversiPlayerColor,
    pub move_pos_row: usize,
    pub move_pos_col: usize,
    pub is_pass: bool,
}

#[repr(C)]
pub struct ReversiInitParam {
    pub game_event_handler: Option<extern "C" fn(ReversiGameEvent, &ReversiGameEventParam)>,
    pub black_player_action: Option<extern "C" fn(&ReversiGameEventParam) -> ReversiAction>,
    pub white_player_action: Option<extern "C" fn(&ReversiGameEventParam) -> ReversiAction>,
}

#[no_mangle]
pub extern "C" fn reversi_create_game(param: &ReversiInitParam) -> *mut Game<BitBoard> {
    let board = Rc::new(BitBoard::new_initial());

    let black_player: Box<dyn Player> = if let Some(action_func) = param.black_player_action {
        Box::new(FfiPlayer { action_func })
    } else {
        Box::new(AiPlayer::new(4))
    };

    let white_player: Box<dyn Player> = if let Some(action_func) = param.white_player_action {
        Box::new(FfiPlayer { action_func })
    } else {
        Box::new(AiPlayer::new(4))
    };

    let event_handler: Option<Box<dyn GameEventHandler>> =
        if let Some(handler) = param.game_event_handler {
            Some(Box::new(ReversiGameEventHandler {
                game_event_handler: handler,
            }))
        } else {
            None
        };

    let game = Box::new(Game::new(board, black_player, white_player, event_handler));

    Box::into_raw(game)
}

#[no_mangle]
pub extern "C" fn reversi_destroy_game(game: *mut Game<BitBoard>) {
    unsafe { Box::from_raw(game) };
}

#[no_mangle]
pub extern "C" fn reversi_run(game: *mut Game<BitBoard>) {
    unsafe {
        (*game).run();
    }
}

fn player_color_to_reversi_player_color(c: PlayerColor) -> ReversiPlayerColor {
    match c {
        PlayerColor::Black => ReversiPlayerColor::Black,
        PlayerColor::White => ReversiPlayerColor::White,
    }
}

fn game_event_param_to_reversi_game_event_param(
    param: &GameEventParameter,
) -> ReversiGameEventParam {
    let mut board = [[0u32; BOARD_SIZE]; BOARD_SIZE];
    for (r, _) in param.board.iter().enumerate() {
        for (c, _) in param.board.iter().enumerate() {
            board[r][c] = match param.board[r][c] {
                crate::Square::Empty => 0,
                crate::Square::Black => 1,
                crate::Square::White => 2,
            };
        }
    }

    let turn = player_color_to_reversi_player_color(param.turn);

    let (last_action_turn, move_pos_row, move_pos_col, is_pass) =
        if let Some(act) = param.last_action {
            let t = player_color_to_reversi_player_color(act.color);
            match act.action {
                ActionType::Move(pos) => (t, pos.0, pos.1, false),
                ActionType::Pass => (t, 0, 0, true),
            }
        } else {
            (ReversiPlayerColor::Black, 0, 0, false)
        };

    let last_action = ReversiAction {
        is_none: param.last_action.is_none(),
        turn: last_action_turn,
        move_pos_row,
        move_pos_col,
        is_pass,
    };

    ReversiGameEventParam {
        board,
        depth: param.depth,
        black_count: param.black_count,
        white_count: param.white_count,
        is_end: param.is_end,
        turn,
        last_action,
    }
}
