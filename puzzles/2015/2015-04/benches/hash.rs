use criterion::{Criterion, criterion_group, criterion_main};
use puzzle_2015_04::{part_one, part_two};

fn one(c: &mut Criterion) {
    c.bench_function("part one", |b| {
        b.iter(|| part_one(std::hint::black_box("bgvyzdsv")))
    });
}

fn two(c: &mut Criterion) {
    c.bench_function("part two", |b| {
        b.iter(|| {
            part_two(std::hint::black_box(
                "bgvyzdsv
",
            ))
        })
    });
}

criterion_group!(benches, one, two);
criterion_main!(benches);
