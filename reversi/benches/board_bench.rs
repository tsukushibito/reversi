use criterion::{criterion_group, criterion_main, Criterion};
use reversi::board::ArrayBoard;
use reversi::board::BitBoard;
use reversi::board::Board;
use reversi::board::IndexBoard;
use reversi::board::Indexer;
use reversi::Move;
use reversi::PlayerColor;
use reversi::Position;
use std::rc::Rc;

fn action_table() -> [Move; 10] {
    [
        Move::new_position(PlayerColor::Black, Position(4, 5)),
        Move::new_position(PlayerColor::White, Position(5, 5)),
        Move::new_position(PlayerColor::Black, Position(5, 4)),
        Move::new_position(PlayerColor::White, Position(3, 5)),
        Move::new_position(PlayerColor::Black, Position(2, 4)),
        Move::new_position(PlayerColor::White, Position(1, 3)),
        Move::new_position(PlayerColor::Black, Position(2, 3)),
        Move::new_position(PlayerColor::White, Position(5, 3)),
        Move::new_position(PlayerColor::Black, Position(3, 2)),
        Move::new_position(PlayerColor::White, Position(3, 1)),
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
                    board = board.apply_move(action).unwrap();
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
                    board = board.apply_move(action).unwrap();
                }
            }
        })
    });
}

fn bit_board(c: &mut Criterion) {
    c.bench_function("Bit Board", |b| {
        b.iter(|| {
            let actions = action_table();
            for _ in 0..1000 {
                let mut board = BitBoard::new_initial();
                for action in &actions {
                    board = board.apply_move(action).unwrap();
                }
            }
        })
    });
}

criterion_group!(benches, index_board, array_board, bit_board);
criterion_main!(benches);
