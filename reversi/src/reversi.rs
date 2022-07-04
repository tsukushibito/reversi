use self::{board::BitBoard, game::Game};

pub mod ai;
pub mod board;
pub mod common;
pub mod game;
pub mod player;

#[repr(C)]
pub struct GameEventParamDto {}

#[repr(C)]
pub struct ActionDto {
    pub move_pos_row: u32,
    pub move_pos_col: u32,
    pub is_pass: bool,
}

#[repr(C)]
pub struct InitParam {
    pub game_event_handler: fn(&GameEventParamDto),
    pub black_player_action: fn(&GameEventParamDto) -> ActionDto,
    pub white_player_action: fn(&GameEventParamDto) -> ActionDto,
}

// #[no_mangle]
// pub extern "C" fn reversi_create_game() -> *mut Game {
//     let board = Rc<BitBoard>();
//     Game::new(board, );
// }
