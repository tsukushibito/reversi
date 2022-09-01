use reversi::board::BitBoard;
use reversi::game::play_game;
use reversi::player::AiPlayer;
use reversi::player::ConsoleIoPlayer;

fn main() {
    let board = BitBoard::new_initial();
    let black_player = Box::new(ConsoleIoPlayer::new());
    let white_player = Box::new(AiPlayer::new(7));
    play_game(&board, black_player, white_player);
}
