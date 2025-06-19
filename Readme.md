# Toy project for testing SIMD
My goal for this project was to test if i can write simple SIMD code using rust.  
Given two arrays, i test element wise if one value is the squareroot of the other value. Basic stuff.

I made two implementations. so far
- One where i loop over the arrays and check this sequentially.
- One where i use Simd instructions and test multiple fields at the same time.

Using benchmarks we can check which SIMD operands are most performant.  
I used Criterion to test these. Apparently SIMD operands with 16 fields were the fastest for me.  
You can run `cargo bench` and check for yourself what is fastest on your machine. 
Check your `target/criterion/report/index.html` to see pretty graphs.

## My test results so far
```
CpuIsRoot               time:   [478.41 µs 480.18 µs 482.09 µs]
Found 23 outliers among 100 measurements (23.00%)
  14 (14.00%) low mild
  2 (2.00%) high mild
  7 (7.00%) high severe

SimdIsRoot::<4>         time:   [269.10 µs 272.41 µs 276.31 µs]
Found 7 outliers among 100 measurements (7.00%)
  6 (6.00%) high mild
  1 (1.00%) high severe

SimdIsRoot::<8>         time:   [158.05 µs 162.89 µs 167.84 µs]
Found 15 outliers among 100 measurements (15.00%)
  2 (2.00%) low mild
  8 (8.00%) high mild
  5 (5.00%) high severe

SimdIsRoot::<16>        time:   [142.93 µs 144.69 µs 146.61 µs]
Found 6 outliers among 100 measurements (6.00%)
  6 (6.00%) high mild

SimdIsRoot::<32>        time:   [289.12 µs 291.96 µs 295.15 µs]
Found 7 outliers among 100 measurements (7.00%)
  5 (5.00%) high mild
  2 (2.00%) high severe

SimdIsRoot::<64>        time:   [347.62 µs 351.37 µs 355.63 µs]
Found 15 outliers among 100 measurements (15.00%)
  3 (3.00%) high mild
  12 (12.00%) high severe
```