//! Interleave two buffers into one.
//! https://godbolt.org/z/7h51edWbq

use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[inline(never)]
fn interleave(buf: &mut [f32], buf0: &[f32], buf1: &[f32]) {
    for i in 0..buf.len() {
        let half = i / 2;
        let modulo = i % 2;
        buf[i] = if modulo == 0 {
            buf0[i - half]
        } else {
            buf1[i - half - modulo]
        };
    }
}

#[inline(never)]
fn interleave_loop(buf: &mut [f32], buf0: &[f32], buf1: &[f32]) {
    let mut idx = 0;
    for i in 0..buf.len() {
        buf[i] = if i % 2 == 0 {
            buf0[idx]
        } else {
            let tmp = buf1[idx];
            idx += 1;
            tmp
        }
    }
}

#[inline(never)]
fn interleave_iter(buf: &mut [f32], buf0: &[f32], buf1: &[f32]) {
    let mut idx = 0;
    let mut right = true;
    for out in buf {
        *out = if right {
            right = false;
            buf0[idx]
        } else {
            right = true;
            let tmp = buf1[idx];
            idx += 1;
            tmp
        }
    }
}

#[inline(never)]
fn interleave_twice(buf: &mut [f32], mut buf0: &[f32], mut buf1: &[f32]) {
    for ((out, a), b) in buf.chunks_exact_mut(2).zip(buf0).zip(buf1) {
        out[0] = *a;
        out[1] = *b;
    }
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("interleave");
    let mut buf = [0.0; 8192];
    let buf0 = [0.0; 4096];
    let buf1 = [0.0; 4096];
    group.bench_function("naive", |b| {
        b.iter(|| interleave(black_box(&mut buf), black_box(&buf0), black_box(&buf1)))
    });
    group.bench_function("loop", |b| {
        b.iter(|| interleave_loop(black_box(&mut buf), black_box(&buf0), black_box(&buf1)))
    });
    group.bench_function("iter", |b| {
        b.iter(|| interleave_iter(black_box(&mut buf), black_box(&buf0), black_box(&buf1)))
    });
    group.bench_function("twice", |b| {
        b.iter(|| interleave_twice(black_box(&mut buf), black_box(&buf0), black_box(&buf1)))
    });
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
