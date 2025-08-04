use criterion::{Criterion, criterion_group, criterion_main};
use monte_carlo_root::is_root::Root;
use monte_carlo_root::{cpu_is_root, wgpu_is_root};
use monte_carlo_root::simd_is_root::SimdIsRoot;
use std::hint::black_box;
macro_rules! bench_lanes {
    ($group:ident, $sqrt:expr, $input:expr, $delta:expr, [ $( $lanes:literal ),* ]) => {
        $(
            $group.bench_function(&format!("SimdIsRoot::<{}>", $lanes), |b| {
                b.iter(|| {
                    let _ = SimdIsRoot::<$lanes>::is_root(
                        black_box($sqrt),
                        black_box($input),
                        black_box($delta),
                    );
                });
            });
        )*
    };
}

fn bench_is_root(c: &mut Criterion) {
    let size = 1024 * 1024;
    let delta = 0.000001;
    let input: Vec<f32> = (0..size).map(|x| x as f32).collect();
    let sqrt: Vec<f32> = input.iter().map(|x| x.sqrt()).collect();

    c.bench_function("CpuIsRoot", |b| {
        b.iter(|| {
            let _ = cpu_is_root::CpuIsRoot::is_root(
                black_box(&sqrt),
                black_box(&input),
                black_box(delta),
            );
        })
    });

    c.bench_function("WgpuIsRoot", |b| {
        b.iter(|| {
            let _ = wgpu_is_root::WgpuIsRoot::is_root(
                black_box(&sqrt),
                black_box(&input),
                black_box(delta),
            );
        })
    });

    bench_lanes!(
        c,
        &sqrt,
        &input,
        delta,
        [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46,
            47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64
        ]
    );
}

criterion_group!(benches, bench_is_root);
criterion_main!(benches);
