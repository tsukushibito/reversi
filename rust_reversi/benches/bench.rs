use criterion::{criterion_group, criterion_main, Criterion};
use reversi::board::array_board::ArrayBoard;
use reversi::board::index_board::IndexBoard;
use reversi::board::indexer::Indexer;
use reversi::board::Board;
use reversi::Action;
use reversi::ActionType;
use reversi::PlayerColor;
use reversi::Position;
use std::rc::Rc;

fn action_table() -> [Action; 10] {
    [
        Action::new(PlayerColor::Black, ActionType::Move(Position(4, 5))),
        Action::new(PlayerColor::White, ActionType::Move(Position(5, 5))),
        Action::new(PlayerColor::Black, ActionType::Move(Position(5, 4))),
        Action::new(PlayerColor::White, ActionType::Move(Position(3, 5))),
        Action::new(PlayerColor::Black, ActionType::Move(Position(2, 4))),
        Action::new(PlayerColor::White, ActionType::Move(Position(1, 3))),
        Action::new(PlayerColor::Black, ActionType::Move(Position(2, 3))),
        Action::new(PlayerColor::White, ActionType::Move(Position(5, 3))),
        Action::new(PlayerColor::Black, ActionType::Move(Position(3, 2))),
        Action::new(PlayerColor::White, ActionType::Move(Position(3, 1))),
    ]
}

fn index_board(c: &mut Criterion) {
    c.bench_function("Index Board", |b| {
        b.iter(|| {
            let actions = action_table();
            let indexer = Rc::new(Indexer::new());
            for _ in 0..1000 {
                let mut board = IndexBoard::new_initial(indexer.clone());
                for action in &actions {
                    board = board.apply_action(&action).unwrap();
                }
            }
        })
    });
}

fn array_board(c: &mut Criterion) {
    c.bench_function("Array Board", |b| {
        b.iter(|| {
            let actions = action_table();
            for _ in 0..1000 {
                let mut board = ArrayBoard::new_initial();
                for action in &actions {
                    board = board.apply_action(&action).unwrap();
                }
            }
        })
    });
}

criterion_group!(benches, index_board, array_board);
criterion_main!(benches);
