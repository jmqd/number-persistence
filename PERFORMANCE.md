# Performance profiling, optimization history

## Completely unoptimized solution.

```
jordan@linux-desktop:~/src/number-persistence% sudo perf stat -d cargo run --release search 0 26889000
    Finished release [optimized] target(s) in 0.05s                                                                                                                                                                                                            
     Running `target/release/number-persistence search 0 26889000`
Found a new record: 11
Found a new record: 25
Found a new record: 39
Found a new record: 77
Found a new record: 679
Found a new record: 6788
Found a new record: 68889
Found a new record: 2677889
Found a new record: 26888999
Overall record: 26888999

 Performance counter stats for 'cargo run --release search 0 26889000':

      30148.084832      task-clock (msec)         #    0.999 CPUs utilized          
                56      context-switches          #    0.002 K/sec                  
                 1      cpu-migrations            #    0.000 K/sec                  
             2,085      page-faults               #    0.069 K/sec                  
   105,295,599,156      cycles                    #    3.493 GHz                      (50.00%)
   270,413,608,805      instructions              #    2.57  insn per cycle           (62.50%)
    58,619,531,315      branches                  # 1944.387 M/sec                    (62.51%)
        81,379,745      branch-misses             #    0.14% of all branches          (62.51%)
    62,240,836,979      L1-dcache-loads           # 2064.504 M/sec                    (62.47%)
         3,933,617      L1-dcache-load-misses     #    0.01% of all L1-dcache hits    (25.00%)
           838,041      LLC-loads                 #    0.028 M/sec                    (25.00%)
            87,056      LLC-load-misses           #   10.39% of all LL-cache hits     (37.50%)

      30.165656130 seconds time elapsed
```

## Trial with dynamic programming

Basically, showed slower runtimes. Last-level cache misses and register LOAD
times were high. Ultimately, for this problem, DP is a bad fit because the tree
of multiplication subproblems do not sufficiently overlap. Fundamentally, just
doing the math on the CPU is faster than the hash lookups.

Reducing the search space as cleverly as possible will be the best efficiency
improvement. As for performance, we just need to pipeline as fast as possible.
Probably do some loop unrolling and reduce unnecessary copies and intermediate
results is the best.

## Removed unnecessary intermediate BigUint in fold accumulator

Also used u8 instead of u32 where possible to pack more bits into the cache
lines.

Got ~2x speedup.

```
jordan@linux-desktop:~/src/number-persistence% sudo perf stat -d cargo run --release search 0 26889000
    Finished release [optimized] target(s) in 0.03s                                                                            
     Running `target/release/number-persistence search 0 26889000`
Found a new record: 11
Found a new record: 25
Found a new record: 39
Found a new record: 77
Found a new record: 679
Found a new record: 6788
Found a new record: 68889
Found a new record: 2677889
Found a new record: 26888999
Overall record: 26888999

 Performance counter stats for 'cargo run --release search 0 26889000':

      16891.854780      task-clock (msec)         #    1.000 CPUs utilized          
                34      context-switches          #    0.002 K/sec                  
                 1      cpu-migrations            #    0.000 K/sec                  
             2,151      page-faults               #    0.127 K/sec                  
    59,495,526,300      cycles                    #    3.522 GHz                      (50.00%)
   143,884,357,394      instructions              #    2.42  insn per cycle           (62.50%)
    31,742,030,562      branches                  # 1879.132 M/sec                    (62.50%)
        54,294,401      branch-misses             #    0.17% of all branches          (62.50%)
    30,727,593,782      L1-dcache-loads           # 1819.078 M/sec                    (62.44%)
         2,155,811      L1-dcache-load-misses     #    0.01% of all L1-dcache hits    (25.00%)
           336,642      LLC-loads                 #    0.020 M/sec                    (25.00%)
            46,022      LLC-load-misses           #   13.67% of all LL-cache hits     (37.50%)

      16.897185961 seconds time elapsed
```

## Unrolling the loop

Unrolling once gets us ~1.09 speedup.

```
jordan@linux-desktop:~/src/number-persistence% sudo perf stat -d cargo run --release search 0 26889000
    Finished release [optimized] target(s) in 0.02s                                                                            
     Running `target/release/number-persistence search 0 26889000`
Found a new record: 1
Found a new record: 25
Found a new record: 39
Found a new record: 77
Found a new record: 679
Found a new record: 6788
Found a new record: 68889
Found a new record: 2677889
Found a new record: 26888999
Overall record: 26888999

 Performance counter stats for 'cargo run --release search 0 26889000':

      15377.292911      task-clock (msec)         #    1.000 CPUs utilized          
                29      context-switches          #    0.002 K/sec                  
                 0      cpu-migrations            #    0.000 K/sec                  
             2,088      page-faults               #    0.136 K/sec                  
    54,156,566,532      cycles                    #    3.522 GHz                      (49.97%)
   132,718,626,792      instructions              #    2.45  insn per cycle           (62.48%)
    29,340,126,690      branches                  # 1908.016 M/sec                    (62.51%)
        34,374,290      branch-misses             #    0.12% of all branches          (62.53%)
    28,494,020,730      L1-dcache-loads           # 1852.993 M/sec                    (62.47%)
         1,481,694      L1-dcache-load-misses     #    0.01% of all L1-dcache hits    (24.98%)
           292,247      LLC-loads                 #    0.019 M/sec                    (24.97%)
            31,786      LLC-load-misses           #   10.88% of all LL-cache hits     (37.46%)

      15.376571734 seconds time elapsed
```

Let's unroll it again.

```
jordan@linux-desktop:~/src/number-persistence% sudo perf stat -d cargo run --release search 0 26889000
    Finished release [optimized] target(s) in 0.03s                                                                            
     Running `target/release/number-persistence search 0 26889000`
Found a new record: 1
Found a new record: 39
Found a new record: 77
Found a new record: 679
Found a new record: 6788
Found a new record: 68889
Found a new record: 2677889
Found a new record: 26888999
Overall record: 26888999

 Performance counter stats for 'cargo run --release search 0 26889000':

      13728.501202      task-clock (msec)         #    1.000 CPUs utilized          
                29      context-switches          #    0.002 K/sec                  
                 0      cpu-migrations            #    0.000 K/sec                  
             2,086      page-faults               #    0.152 K/sec                  
    48,445,367,611      cycles                    #    3.529 GHz                      (50.01%)
   121,723,798,032      instructions              #    2.51  insn per cycle           (62.51%)
    27,079,480,127      branches                  # 1972.501 M/sec                    (62.51%)
        32,619,766      branch-misses             #    0.12% of all branches          (62.51%)
    26,043,678,256      L1-dcache-loads           # 1897.052 M/sec                    (24.99%)
         1,566,512      L1-dcache-load-misses     #    0.01% of all L1-dcache hits    (24.99%)
           378,204      LLC-loads                 #    0.028 M/sec                    (24.99%)
            29,571      LLC-load-misses           #    7.82% of all LL-cache hits     (37.49%)

      13.732009739 seconds time elapsed
```

Not bad. Another ~1.1x speedup.
