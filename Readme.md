# Toy project for testing CPU, SIMD and GPU
The goal of this project is to showcase how much of a speedup i can achieve for an parallelizable problem.
I chose an NBody simulation as test project because it is simple to implement (yes, i have big mistakes in the simulation, vibe-coding is still not there yet i guess) and easy to parallelize.


## My test results for NBody simulation (CPU and GPU, SIMD is still missing.)
Run the NBody benchmark with `cargo bench -- NBody`.
Check your `target/criterion/report/index.html` to see pretty graphs.

```
NBodyGPU 100 points 1 step
                        time:   [35.014 ms 35.138 ms 35.278 ms]
Found 8 outliers among 100 measurements (8.00%)
  6 (6.00%) high mild
  2 (2.00%) high severe

NBodyCPU 100 points 1 step
                        time:   [25.178 µs 25.226 µs 25.287 µs]
Found 11 outliers among 100 measurements (11.00%)
  2 (2.00%) high mild
  9 (9.00%) high severe

NBodyGPU 1000 points 1 step
                        time:   [37.641 ms 38.065 ms 38.516 ms]
Found 8 outliers among 100 measurements (8.00%)
  8 (8.00%) high mild

NBodyCPU 1000 points 1 step
                        time:   [2.6109 ms 2.6152 ms 2.6192 ms]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) low mild

NBodyGPU 10000 points 1 step
                        time:   [45.385 ms 45.690 ms 46.013 ms]
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild

Benchmarking NBodyCPU 10000 points 1 step: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 26.0s, or reduce sample count to 10.
NBodyCPU 10000 points 1 step
                        time:   [257.13 ms 257.55 ms 257.95 ms]
Found 7 outliers among 100 measurements (7.00%)
  7 (7.00%) low mild

NBodyGPU 100 points 10 step
                        time:   [39.854 ms 40.157 ms 40.484 ms]
Found 5 outliers among 100 measurements (5.00%)
  5 (5.00%) high mild

NBodyCPU 100 points 10 step
                        time:   [259.75 µs 260.07 µs 260.38 µs]

NBodyGPU 1000 points 10 step
                        time:   [42.375 ms 42.744 ms 43.133 ms]
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild

NBodyCPU 1000 points 10 step
                        time:   [25.771 ms 25.802 ms 25.832 ms]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) low mild

Benchmarking NBodyGPU 10000 points 10 step: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 9.1s, or reduce sample count to 50.
NBodyGPU 10000 points 10 step
                        time:   [90.120 ms 90.645 ms 91.171 ms]
Found 24 outliers among 100 measurements (24.00%)
  14 (14.00%) low mild
  9 (9.00%) high mild
  1 (1.00%) high severe

Benchmarking NBodyCPU 10000 points 10 step: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 259.2s, or reduce sample count to 10.
NBodyCPU 10000 points 10 step
                        time:   [2.6015 s 2.6057 s 2.6099 s]

Benchmarking NBodyGPU 100 points 100 step: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 6.2s, or reduce sample count to 80.
NBodyGPU 100 points 100 step
                        time:   [61.454 ms 62.836 ms 64.203 ms]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

NBodyCPU 100 points 100 step
                        time:   [2.6040 ms 2.6078 ms 2.6116 ms]
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) low mild
  1 (1.00%) high mild

Benchmarking NBodyGPU 1000 points 100 step: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 7.9s, or reduce sample count to 60.
NBodyGPU 1000 points 100 step
                        time:   [77.685 ms 78.576 ms 79.378 ms]
Found 12 outliers among 100 measurements (12.00%)
  6 (6.00%) low severe
  2 (2.00%) low mild
  4 (4.00%) high mild

Benchmarking NBodyCPU 1000 points 100 step: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 26.2s, or reduce sample count to 10.
NBodyCPU 1000 points 100 step
                        time:   [261.30 ms 261.74 ms 262.16 ms]
Found 5 outliers among 100 measurements (5.00%)
  5 (5.00%) low mild

```