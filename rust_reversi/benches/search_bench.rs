use criterion::{criterion_group, criterion_main, Criterion};
use reversi::board::*;
use reversi::*;
use std::rc::Rc;

fn nega_max(c: &mut Criterion) {
    c.bench_function("NegaMax", |b| {
        b.iter(|| {
            let indexer = Rc::new(Indexer::new());
            let board = IndexBoard::new_initial(indexer);
            let _result = ai::search_game_tree(
                &board,
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
            let indexer = Rc::new(Indexer::new());
            let board = IndexBoard::new_initial(indexer);
            let _result = ai::search_game_tree(
                &board,
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
