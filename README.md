# Experiments with Faust and Rust

Targeting Rust latest stable version (no nightly only optimizations).

## Benchmarks

In `./benches`, run with `cargo criterion --bench <name>`.

Compilation options : `--release` and `RUSTFLAGS="-C target-cpu=native"`

### The effect of `repr(C)` on the DSP struct

Rust binary layout is unstable and recently changed with Rust 1.67, causing sometimes dramatic performance variations
compared to code compiled with previous Rust version. Using `repr(C)` guarantees layout stability, is it necessarily
better though ?

Here is a benchmark run comparing using `repr(C)` to without using it.

```text
karplus32/original scalar time:   [224.16 µs 224.29 µs 224.45 µs]
                          thrpt:  [18.249 Melem/s 18.262 Melem/s 18.273 Melem/s]
                   change:
                          time:   [-60.016% -59.948% -59.864%] (p = 0.00 < 0.05)
                          thrpt:  [+149.15% +149.67% +150.10%]
                          Performance has improved.
karplus32/original vec/4  time:   [192.18 µs 192.45 µs 192.78 µs]
                          thrpt:  [21.248 Melem/s 21.283 Melem/s 21.313 Melem/s]
                   change:
                          time:   [+5.8379% +5.9454% +6.0674%] (p = 0.00 < 0.05)
                          thrpt:  [-5.7203% -5.6118% -5.5159%]
                          Performance has regressed.
```

Judging by this example, using `repr(C)` or not isn't evident. The impact on performance is similar (relatively) when
compiling for a native cpu.

### The effect of `-C target-cpu=native`

```shell
cargo bench --bench noise -- --save-baseline cpu
RUSTFLAGS="-C target-cpu=native" cargo bench --bench noise -- --baseline cpu
```

| Benchmark                 | Avg time (µs) | Change (%)     |
|---------------------------|---------------|----------------|
| noise/slice               | 3.653         | *None*         |
| noise/slice chunked/4     | 3.487         | 🟩 -11.5 %     |
| noise/slice chunked/8     | 2.000         | *within noise* |
| noise/array               | 3.656         | *None*         |
| noise/array chunked/4     | 3.646         | 🟩 -2.8 %      |
| noise/array chunked/8     | 3.649         | *within noise* |
| noise/array chunked vec/4 | 2.743         | 🟥 +26.1 %     |
| noise/array chunked vec/8 | 1.327         | 🟥 +4.8 %      |

### Comparison with C code

The benchmarks also measure the equivalent generated C code.

***TODO** define a set of optimal compilation parameters for different C compilers*
