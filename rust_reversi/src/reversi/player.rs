use crate::*;

pub trait Player {
    fn take_action(&mut self, depth: u32, squares: &Squares) -> Action;
}

pub struct ConsoleIoPlayer {}

impl ConsoleIoPlayer {
    pub fn new() -> ConsoleIoPlayer {
        ConsoleIoPlayer {}
    }
}

impl Player for ConsoleIoPlayer {
    fn take_action(&mut self, depth: u32, squares: &Squares) -> Action {
        println!("[{}]", depth);
        println!("   a b c d e f g h");
        println!("   ----------------");
        for (i, row) in squares.iter().enumerate() {
            let mut r = (i + 1).to_string() + "|";
            for square in row {
                r += match square {
                    Square::Empty => " .",
                    Square::Black => " b",
                    Square::White => " w",
                }
            }
            println!("{}", r);
        }
        println!("Please input move position.");

        let color = if depth % 2 == 0 {
            Square::Black
        } else {
            Square::White
        };

        loop {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).ok();
            let pos: Vec<char> = input.trim().chars().collect();
            if pos.len() < 2 || pos[0] < 'a' || pos[0] > 'h' || pos[1] < '1' || pos[1] > '8' {
                if input == "pass" {
                    break Action::new_pass(color);
                }
                println!("Invalid input!");
            } else {
                let pos = BoardPosition(
                    pos[1] as usize - '1' as usize,
                    pos[0] as usize - 'a' as usize,
                );
                break Action::new_move(color, pos);
            }
        }
    }
}
