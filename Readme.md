# Toy project for testing SIMD
My goal for this project was to test if i can write simple SIMD code using rust.  
Given two arrays, i test element wise if one value is the squareroot of the other value. Basic stuff.

I made two implementations. so far
- One where i loop over the arrays and check this sequentially.
- One where i use Simd instructions and test multiple fields at the same time.

Using benchmarks we can check which SIMD operands are most performant.  
I used Criterion to test these. Apparently SIMD operands with 16 fields were the fastest for me.  
You can run 'cargo bench' and check for yourself what is fastest on your machine. 
Check your 'target/criterion/report/index.html' to see pretty graphs.