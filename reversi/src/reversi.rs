pub mod ai;
pub mod board;
pub mod common;
pub mod game;
pub mod player;

#[repr(C)]
pub struct GameEventParamDto {}

#[repr(C)]
pub struct InitParam {
    pub on_game_event_received: fn(&GameEventParamDto),
}

#[no_mangle]
pub extern "C" fn reversi_init() {}
