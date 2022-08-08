use crate::game::GameEventParameter;
use crate::player::Player;
use crate::reversi::common::*;

pub struct ConsoleIoPlayer {}

impl ConsoleIoPlayer {
    pub fn new() -> ConsoleIoPlayer {
        ConsoleIoPlayer {}
    }
}

impl Default for ConsoleIoPlayer {
    fn default() -> Self {
        Self::new()
    }
}

impl Player for ConsoleIoPlayer {
    fn take_action(&mut self, state: &GameEventParameter) -> Action {
        println!("[{}]", state.depth);
        println!("   a b c d e f g h");
        println!("   ----------------");
        for r in 0..BOARD_SIZE {
            let mut row_string = (r + 1).to_string() + "|";
            let index = r * BOARD_SIZE;
            for square in &state.board[index..index + BOARD_SIZE] {
                row_string += match square {
                    Square::Empty => " .",
                    Square::Black => " b",
                    Square::White => " w",
                }
            }
            println!("{}", r);
        }
        println!("Please input move position.");

        let color = state.turn;

        loop {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).ok();
            let pos: Vec<char> = input.trim().chars().collect();
            if pos.len() < 2 || pos[0] < 'a' || pos[0] > 'h' || pos[1] < '1' || pos[1] > '8' {
                if input == "pass" {
                    break Action::new(color, ActionType::Pass);
                }
                println!("Invalid input!");
            } else {
                let pos = Position(
                    pos[1] as usize - '1' as usize,
                    pos[0] as usize - 'a' as usize,
                );
                break Action::new(color, ActionType::Move(pos));
            }
        }
    }
}
