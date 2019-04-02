# Number persistence

## Description

A rust binary to calculate the multiplicative [persistence of a number][].

See [OEIS Sequence A003001][].

## Testing

```
jordan@linux-desktop:~/src/number-persistence% cargo run --release search 0 26889000
Finished release [optimized] target(s) in 0.07s
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
```

[OEIS Sequence A003001]: https://oeis.org/A003001
[persistence of a number]: https://en.wikipedia.org/wiki/Persistence_of_a_number
