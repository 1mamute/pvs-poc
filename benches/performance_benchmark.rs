use criterion::{criterion_group, criterion_main, Criterion};
use pvs_optimization::pvs::baseline::BaselinePVS;

fn benchmark_baseline(c: &mut Criterion) {
    c.bench_function("baseline_pvs", |b| b.iter(|| {
        let mut pvs = BaselinePVS::new();
        pvs.update_pvs();
    }));
}

criterion_group!(benches, benchmark_baseline);
criterion_main!(benches);
