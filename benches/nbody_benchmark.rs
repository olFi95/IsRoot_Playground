use criterion::async_executor::FuturesExecutor;
use criterion::{Criterion, criterion_group, criterion_main};
use monte_carlo_root::is_root::cpu_is_root;
use monte_carlo_root::nbody::nbody::{NBody, generate_test_data};
use monte_carlo_root::nbody::nbody_cpu::NBodyCPU;
use monte_carlo_root::nbody::nbody_gpu::NBodyGPU;
use std::hint::black_box;

fn nbody_bench(c: &mut Criterion) {
    let mut initial_positions = generate_test_data(1000);
    let mut steps = 1;

    for steps in [1, 10, 100] {
        for initial_positions in [
            generate_test_data(100),
            generate_test_data(1000),
            generate_test_data(10000),
        ] {
            if steps > 10 && initial_positions.len() > 1000 {
                // 100 rounds of 10000 points takes a very long time on CPU and even on GPU...
                // ain't nobody got time for that, we get the point without this result.
                continue;
            }
            c.bench_function(
                format!("NBodyGPU {} points {steps} step", initial_positions.len()).as_str(),
                |b| {
                    b.to_async(FuturesExecutor).iter(|| async {
                        let mut nbody = NBodyGPU::new(&initial_positions).await;
                        nbody.step(steps).await;
                    })
                },
            );
            c.bench_function(
                format!("NBodyCPU {} points {steps} step", initial_positions.len()).as_str(),
                |b| {
                    b.to_async(FuturesExecutor).iter(|| async {
                        let mut nbody = NBodyCPU::new(&initial_positions);
                        nbody.step(steps).await;
                    })
                },
            );
        }
    }
}

criterion_group!(benches, nbody_bench);
criterion_main!(benches);
