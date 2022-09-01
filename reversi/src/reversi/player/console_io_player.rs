use crate::game::GameState;
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
    fn take_action(&self, state: &GameState) -> Action {
        println!("[{}]", state.depth);
        println!("{}", squares_to_string(&state.board));
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
