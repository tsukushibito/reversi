pub mod ai;
pub mod board;
pub mod common;
pub mod game;
pub mod player;

#[no_mangle]
pub extern "C" fn reversi_init() {}
