use crate::{board::BitBoard, game::play_game, player::AiPlayer};

pub struct SelfPlayParameter {
    pub search_depth: usize,
}

pub fn run_self_play(param: &SelfPlayParameter) {
    let black_player = Box::new(AiPlayer::new(param.search_depth));
    let white_player = Box::new(AiPlayer::new(param.search_depth));
    let board = BitBoard::new_initial();
    let result = play_game(&board, black_player, white_player);
}
