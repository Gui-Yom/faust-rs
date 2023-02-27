# Experiments with Faust and Rust

Various improvements to the compiler and associated architectures.

*In progress*

## Improved developer experience

Auto generate Rust code via macro, Rust code is always up-to-date with the dsp source.

## Improved DSP interface

Interfaces in a common separated crate. Usage of const generics.

## Improved generated methods

...

## Much better code auto-vectorization

### Benchmarks

In `./benches`, run with `cargo criterion --bench <name>`.
