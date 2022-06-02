use reversi::ai_player::AiPlayer;
use reversi::game::Game;
use reversi::index_board::IndexBoard;
use reversi::indexer::Indexer;
use reversi::player::ConsoleIoPlayer;
use std::rc::Rc;

fn main() {
    let indexer = Rc::new(Indexer::new());
    let board = Rc::new(IndexBoard::new_initial(indexer));
    let black_player = ConsoleIoPlayer::new();
    let white_player = AiPlayer::new(7);
    let mut reversi = Game::new(board, black_player, white_player);
    reversi.run();
}
