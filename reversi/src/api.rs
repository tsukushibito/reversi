use std::convert::TryInto;

use crate::{
    ai::{self, SearchType, SimpleEvaluator},
    board::{BitBoard, Board},
    ActionType, PlayerColor, Square, Squares, BOARD_SIZE,
};

#[allow(dead_code)]
pub struct Action {
    pub color: i32,
    pub row: i32,
    pub col: i32,
    pub is_pass: bool,
}

#[allow(dead_code)]
pub struct SearchResult {
    pub value: i32,
    pub action: Box<Action>,
}

#[allow(dead_code)]
pub fn search_game_tree(
    squares: Vec<i32>,
    turn_depth: i32,
    color: i32,
    search_depth: i32,
) -> SearchResult {
    let squares = squares
        .iter()
        .map(|i| match i {
            1 => Square::Black,
            2 => Square::White,
            _ => Square::Empty,
        })
        .collect::<Vec<Square>>();
    let squares: Squares = squares
        .try_into()
        .unwrap_or([Square::Empty; BOARD_SIZE * BOARD_SIZE]);
    let board = BitBoard::new(&squares, turn_depth as u32);
    let player_color = match color {
        1 => PlayerColor::Black,
        _ => PlayerColor::White,
    };
    let result = ai::search_game_tree::<SimpleEvaluator>(
        board.squares(),
        &player_color,
        &SearchType::NegaAlpha,
        search_depth as usize,
    );

    let value = result.value;
    let act = result.action.unwrap();
    let mut row = 0;
    let mut col = 0;
    let mut is_pass = false;
    match act.action {
        ActionType::Move(pos) => {
            row = pos.0 as i32;
            col = pos.1 as i32;
        }
        ActionType::Pass => is_pass = true,
    }
    let action = Box::new(Action {
        color,
        row,
        col,
        is_pass,
    });

    SearchResult { action, value }
}
