use reversi::board::IndexBoard;
use reversi::board::Indexer;
use reversi::game::Game;
use reversi::player::AiPlayer;
use reversi::player::ConsoleIoPlayer;
use std::rc::Rc;

fn main() {
    let indexer = Rc::new(Indexer::new());
    let board = Rc::new(IndexBoard::new_initial(indexer));
    let black_player = Box::new(ConsoleIoPlayer::new());
    let white_player = Box::new(AiPlayer::new(7));
    let mut reversi = Game::new(board, black_player, white_player, None);
    reversi.run();
}
