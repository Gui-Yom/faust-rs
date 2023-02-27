//! Stereo LCG noise
//! https://godbolt.org/z/PcnoroEME

use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[inline(never)]
fn noise_slice(last: &mut i32, outputs: [&mut [f32]; 2]) {
    let [output0_, output1_] = outputs;
    for (output0, output1) in output0_.iter_mut().zip(output1_) {
        let tmp = i32::wrapping_add(i32::wrapping_mul(1103515245, *last), 12345);
        let tmpf: f32 = 4.6566128e-12 * (tmp as f32);
        *output0 = tmpf;
        *output1 = tmpf;
        *last = tmp;
    }
}

#[inline(never)]
fn noise_array<const N: usize>(last: &mut i32, outputs: [&mut [f32; N]; 2]) {
    let [output0_, output1_] = outputs;
    for (output0, output1) in output0_.iter_mut().zip(output1_) {
        let tmp = i32::wrapping_add(i32::wrapping_mul(1103515245, *last), 12345);
        let tmpf: f32 = 4.6566128e-12 * (tmp as f32);
        *output0 = tmpf;
        *output1 = tmpf;
        *last = tmp;
    }
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("noise");
    let mut buf0 = [0.0; 4096];
    let mut buf1 = [0.0; 4096];
    group.bench_function("slice", |b| {
        let mut last = 0;
        b.iter(|| noise_slice(&mut last, [black_box(&mut buf0), black_box(&mut buf1)]))
    });
    group.bench_function("array", |b| {
        let mut last = 0;
        b.iter(|| noise_array(&mut last, [black_box(&mut buf0), black_box(&mut buf1)]))
    });
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
