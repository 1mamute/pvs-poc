

use criterion::criterion_group;
use criterion::criterion_main;
use criterion::BenchmarkId;
use criterion::Criterion;
use criterion::Throughput;
use pvs_poc::pvs::baseline::BaselinePVS32;
use pvs_poc::pvs::baseline::BaselinePVS64;
use pvs_poc::pvs::bitstring::BitStringPVS32;
use pvs_poc::pvs::bitstring::BitStringPVS64;



fn benchmark_baseline_32(c: &mut Criterion) {
    let mut group = c.benchmark_group("u32");
    for size in [10, 100, 1000, 10000, 100000, 1000000].iter() {
        let mut current_entities = BaselinePVS32::generate_random_entities(*size);
        let new_entities = BaselinePVS32::generate_random_entities(*size);
        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &_size| {
            b.iter(|| BaselinePVS32::update_entities(&mut current_entities, &new_entities));
        });
    }
    group.finish();
}

fn benchmark_bitstring_32(c: &mut Criterion) {
  let mut group = c.benchmark_group("u32");
  for size in [10, 100, 1000, 10000, 100000, 1000000].iter() {
      let mut current_entities = BitStringPVS32::generate_random_entities(*size);
      let new_entities = BitStringPVS32::generate_random_entities(*size);
      group.throughput(Throughput::Bytes(*size as u64));
      group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &_size| {
          b.iter(|| BitStringPVS32::update_entities(&mut current_entities, &new_entities));
      });
  }
  group.finish();
}

fn benchmark_baseline_64(c: &mut Criterion) {
let mut group = c.benchmark_group("u64");
for size in [10, 100, 1000, 10000, 100000, 1000000].iter() {
    let mut current_entities = BaselinePVS64::generate_random_entities(*size);
    let new_entities = BaselinePVS64::generate_random_entities(*size);
    group.throughput(Throughput::Bytes(*size as u64));
    group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &_size| {
        b.iter(|| BaselinePVS64::update_entities(&mut current_entities, &new_entities));
    });
}
group.finish();
}

fn benchmark_bitstring_64(c: &mut Criterion) {
  let mut group = c.benchmark_group("u64");
  for size in [10, 100, 1000, 10000, 100000, 1000000].iter() {
      let mut current_entities = BitStringPVS64::generate_random_entities(*size);
      let new_entities = BitStringPVS64::generate_random_entities(*size);
      group.throughput(Throughput::Bytes(*size as u64));
      group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &_size| {
          b.iter(|| BitStringPVS64::update_entities(&mut current_entities, &new_entities));
      });
  }
  group.finish();
}

criterion_group!(u32, 
    // benchmark_baseline_32, 
    benchmark_bitstring_32);
criterion_group!(u64, 
    // benchmark_baseline_64, 
    benchmark_bitstring_64);
criterion_main!(u32, u64);
