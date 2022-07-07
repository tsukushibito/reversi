use std::rc::Rc;

use crate::{
    board::BitBoard,
    game::{Game, GameEvent, GameEventHandler, GameEventParameter},
    player::{AiPlayer, Player},
    Action,
};

pub struct ReversiGameEventHandler {
    pub game_event_handler: fn(GameEvent, &GameEventParameter),
}

impl GameEventHandler for ReversiGameEventHandler {
    fn handle(&self, event: GameEvent, param: &GameEventParameter) {
        (self.game_event_handler)(event, &param);
    }
}

struct FfiPlayer {
    pub action_func: fn(&GameEventParameter) -> Action,
}

impl Player for FfiPlayer {
    fn take_action(&mut self, param: &GameEventParameter) -> Action {
        (self.action_func)(param)
    }
}

pub struct CreateParam {
    pub game_event_handler: Option<fn(GameEvent, &GameEventParameter)>,
    pub black_player_action: Option<fn(&GameEventParameter) -> Action>,
    pub white_player_action: Option<fn(&GameEventParameter) -> Action>,
}

pub fn create_game(param: &CreateParam) -> *mut Game<BitBoard> {
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

pub fn destroy_game(game: *mut Game<BitBoard>) {
    unsafe { Box::from_raw(game) };
}

pub fn game_run(game: *mut Game<BitBoard>) {
    unsafe {
        (*game).run();
    }
}
