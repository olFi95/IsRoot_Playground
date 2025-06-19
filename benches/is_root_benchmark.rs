use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use monte_carlo_root::cpu_is_root;
use monte_carlo_root::is_root::Root;
use monte_carlo_root::simd_is_root::SimdIsRoot;

fn bench_is_root(c: &mut Criterion) {
    let size = 1024 * 1024;
    let delta = 0.000001;
    let input: Vec<f64> = (0..size).map(|x| x as f64).collect();
    let sqrt: Vec<f64> = input.iter().map(|x| x.sqrt()).collect();

    c.bench_function("CpuIsRoot", |b| {
        b.iter(|| {
            let _ = cpu_is_root::CpuIsRoot::is_root(black_box(&sqrt), black_box(&input), black_box(delta));
        })
    });


    c.bench_function("SimdIsRoot::<4>", |b| {
        b.iter(|| {
            let _ = SimdIsRoot::<4>::is_root(black_box(&sqrt), black_box(&input), black_box(delta));
        })
    });
    c.bench_function("SimdIsRoot::<8>", |b| {
        b.iter(|| {
            let _ = SimdIsRoot::<8>::is_root(black_box(&sqrt), black_box(&input), black_box(delta));
        })
    });
    c.bench_function("SimdIsRoot::<16>", |b| {
        b.iter(|| {
            let _ = SimdIsRoot::<16>::is_root(black_box(&sqrt), black_box(&input), black_box(delta));
        })
    });
    c.bench_function("SimdIsRoot::<32>", |b| {
        b.iter(|| {
            let _ = SimdIsRoot::<32>::is_root(black_box(&sqrt), black_box(&input), black_box(delta));
        })
    });
    c.bench_function("SimdIsRoot::<64>", |b| {
        b.iter(|| {
            let _ = SimdIsRoot::<64>::is_root(black_box(&sqrt), black_box(&input), black_box(delta));
        })
    });
}

criterion_group!(benches, bench_is_root);
criterion_main!(benches);