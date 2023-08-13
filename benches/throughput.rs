use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use pibary_of_babel::*;

pub fn generator_throughtput(c: &mut Criterion) {
    let mut group = c.benchmark_group("1MB of throughput");
    group.throughput(Throughput::Bytes(1024 * 1024));

    group.bench_function("pi throughtput (spiggot)", |b| {
        let mut pi = Spigot::new();
        b.iter(|| {
            let _ = black_box(pi.next());
        })
    });

    group.bench_function("pi throughtput (bellard)", |b| {
        let mut pi = BbpBellard::<9>::new(0);
        b.iter(|| {
            let _ = black_box(pi.next());
        })
    });

    group.bench_function("byte-generator throughtput (spiggot)", |b| {
        let mut bg = ByteGenerator::new(Spigot::new());
        b.iter(|| {
            let _ = black_box(bg.next());
        })
    });

    group.bench_function("byte-generator throughtput (bellard)", |b| {
        let mut bg = ByteGenerator::new(BbpBellard::<9>::new(0));
        b.iter(|| {
            let _ = black_box(bg.next());
        })
    });
}

criterion_group!(benches, generator_throughtput);
criterion_main!(benches);
