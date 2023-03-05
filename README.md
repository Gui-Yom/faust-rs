# Experiments with Faust and Rust

Various improvements to the compiler and associated architectures.

*In progress*

Targeting Rust latest stable version (no nightly only optimizations).

## Improved developer experience

Auto generate Rust code via macro, Rust code is always up-to-date with the dsp source.

## Improved DSP interface

Interfaces in a common separated crate. Usage of const generics.

## Improved generated methods

...

## Much better code auto-vectorization

### Benchmarks

In `./benches`, run with `cargo criterion --bench <name>`.

Compilation options : `--release` and `RUSTFLAGS="-C target-cpu=native"`

#### The effect of `-C target-cpu=native`

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

#### Comparison with C code

The benchmarks also measure the equivalent generated C code.

***TODO** define a set of optimal compilation parameters for different C compilers*
