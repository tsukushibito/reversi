use reversi::ConsoleIoPlayer;
use reversi::Reversi;

fn main() {
    let black_player = ConsoleIoPlayer::new();
    let white_player = ConsoleIoPlayer::new();
    let mut reversi = Reversi::new(black_player, white_player);
    reversi.run();
}
