use criterion::{criterion_group, criterion_main, Criterion};
use reversi::ai::{
    NegaAlpha, NegaAlphaNode, NegaMax, NegaMaxNode, Node, SimpleNegaAlphaEvaluationFunction,
    SimpleNegaMaxEvaluationFunction,
};
use reversi::board::*;
use reversi::*;

fn nega_max(c: &mut Criterion) {
    c.bench_function("NegaMax", |b| {
        b.iter(|| {
            let mut nega_max = NegaMax::new(SimpleNegaMaxEvaluationFunction::new());
            let mut root = NegaMaxNode::new(
                BitBoard::new_initial(),
                PlayerColor::Black,
                0,
                Move::new_pass(PlayerColor::White),
            );
            let _result = nega_max.search(&mut root, 7);
            // println!("searched_nodes: {}", root.searched_nodes());
        })
    });
}

fn nega_alpha(c: &mut Criterion) {
    c.bench_function("NegaAlpha", |b| {
        b.iter(|| {
            let mut nega_alpha = NegaAlpha::new(SimpleNegaAlphaEvaluationFunction::new());
            let mut root = NegaAlphaNode::new(
                BitBoard::new_initial(),
                PlayerColor::Black,
                0,
                Move::new_pass(PlayerColor::White),
            );
            let _result = nega_alpha.search(&mut root, 7);
            // println!("searched_nodes: {}", root.searched_nodes());
        })
    });
}

criterion_group!(benches, nega_max, nega_alpha);
criterion_main!(benches);
