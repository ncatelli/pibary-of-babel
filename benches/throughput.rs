use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use pibary_of_babel::*;

pub fn generator_throughtput(c: &mut Criterion) {
    let mut group = c.benchmark_group("1MB of throughput");
    group.throughput(Throughput::Bytes(1024 * 1024));

    group.bench_function("pi throughtput", |b| {
        let mut pi = DigitsOfPi::new();
        b.iter(|| {
            let _ = black_box(pi.next());
        })
    });

    group.bench_function("byte-generator throughtput", |b| {
        let mut bg = ByteGenerator::new();
        b.iter(|| {
            let _ = black_box(bg.next());
        })
    });
}

criterion_group!(benches, generator_throughtput);
criterion_main!(benches);
