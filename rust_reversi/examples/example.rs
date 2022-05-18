use reversi::game::Game;
use reversi::player::ConsoleIoPlayer;

fn main() {
    let black_player = ConsoleIoPlayer::new();
    let white_player = ConsoleIoPlayer::new();
    let mut reversi = Game::new(black_player, white_player);
    reversi.run();
}
