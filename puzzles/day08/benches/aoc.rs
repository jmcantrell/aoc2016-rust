use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};

use aoc::{Input, Parse, Solve};

use day08::aoc::{Parser, Solver};

const INPUT: Input = include_str!("../input.txt");

fn benchmark<P: Parse, S: Solve<P> + Clone>(c: &mut Criterion, name: &str) {
    let mut group = c.benchmark_group(format!("{}/{}", stringify!(day08), name));

    let solver = S::new(P::new(INPUT).parse().unwrap());

    group.bench_function("parse", |b| {
        b.iter(|| P::new(black_box(INPUT)).parse().unwrap())
    });

    group.bench_function("solve", |b| {
        b.iter_batched(|| solver.clone(), |s| s.solve(), BatchSize::SmallInput)
    });

    group.finish();
}

fn answer(c: &mut Criterion) {
    benchmark::<Parser, Solver>(c, "answer");
}

criterion_group!(benches, answer);

criterion_main!(benches);
