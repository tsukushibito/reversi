use crate::board::Board;
use crate::player::Player;

pub struct Reversi<T, U>
where
    T: Player,
    U: Player,
{
    board: Board,
    black_player: T,
    white_player: U,
    depth: u32,
    has_passed: bool,
}

impl<T, U> Reversi<T, U>
where
    T: Player,
    U: Player,
{
    pub fn new(black_player: T, white_player: U) -> Reversi<T, U> {
        let board = Board::new();
        Reversi {
            board: board,
            black_player: black_player,
            white_player: white_player,
            depth: 0,
            has_passed: false,
        }
    }

    pub fn run(&mut self) {
        loop {
            let action = if self.depth % 2 == 0 {
                self.black_player
                    .take_action(self.depth, self.board.squares)
            } else {
                self.white_player
                    .take_action(self.depth, self.board.squares)
            };

            if self.board.apply_action(&action) {
                self.depth += 1;
                if action.pass {
                    if self.has_passed {
                        // 2連続パスなのでゲーム終了
                        break;
                    }
                } else {
                    self.has_passed = false;
                }
            }
        }
    }
}
