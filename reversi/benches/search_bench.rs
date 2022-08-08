use criterion::{criterion_group, criterion_main, Criterion};
use reversi::board::*;
use reversi::*;

fn nega_max(c: &mut Criterion) {
    c.bench_function("NegaMax", |b| {
        b.iter(|| {
            let board = BitBoard::new_initial();
            let _result = ai::search_game_tree(
                board.squares(),
                &PlayerColor::Black,
                &ai::simple_evaluate,
                &ai::SearchType::NegaMax,
                7,
            );
            // println!("searched_nodes: {}", result.searched_nodes);
        })
    });
}

fn nega_alpha(c: &mut Criterion) {
    c.bench_function("NegaAlpha", |b| {
        b.iter(|| {
            let board = BitBoard::new_initial();
            let _result = ai::search_game_tree(
                board.squares(),
                &PlayerColor::Black,
                &ai::simple_evaluate,
                &ai::SearchType::NegaAlpha,
                7,
            );
            // println!("searched_nodes: {}", result.searched_nodes);
        })
    });
}

criterion_group!(benches, nega_max, nega_alpha);
criterion_main!(benches);
