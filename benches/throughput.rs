use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use pibary_of_babel::*;

pub fn generator_throughtput(c: &mut Criterion) {
    let mut group = c.benchmark_group("byte throughput");

    for size in [64, 256] {
        group.throughput(Throughput::Bytes(size));
        group.bench_with_input(
            BenchmarkId::new("pi throughtput (spiggot)", size),
            &size,
            |b, bytes_to_read| {
                let mut pi = Spigot::new();
                b.iter(|| {
                    for _ in 0..*bytes_to_read {
                        let _ = black_box(pi.next());
                    }
                })
            },
        );
        group.bench_with_input(
            BenchmarkId::new("pi throughtput (bellard)", size),
            &size,
            |b, bytes_to_read| {
                let mut pi = BbpBellard::<9>::new(0);
                b.iter(|| {
                    for _ in 0..*bytes_to_read {
                        let _ = black_box(pi.next());
                    }
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("byte-generator throughtput (spiggot)", size),
            &size,
            |b, bytes_to_read| {
                let mut bg = ByteGenerator::new(Spigot::new());
                b.iter(|| {
                    for _ in 0..*bytes_to_read {
                        let _ = black_box(bg.next());
                    }
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("byte-generator throughtput (bellard)", size),
            &size,
            |b, bytes_to_read| {
                let mut bg = ByteGenerator::new(BbpBellard::<9>::new(0));
                b.iter(|| {
                    for _ in 0..*bytes_to_read {
                        let _ = black_box(bg.next());
                    }
                })
            },
        );
    }
}

criterion_group!(benches, generator_throughtput);
criterion_main!(benches);
