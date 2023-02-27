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

/// Isn't actually vectorized effectively
#[inline(never)]
fn noise_vectorizable<const CHUNK_SIZE: usize, const N: usize>(
    last: &mut i32,
    outputs: [&mut [f32; N]; 2],
) {
    #[inline(always)]
    fn iter(last: i32) -> i32 {
        i32::wrapping_add(i32::wrapping_mul(1103515245, last), 12345)
    }

    let [output0_, output1_] = outputs.map(|i| i.chunks_mut(CHUNK_SIZE));
    for (output0, output1) in output0_.zip(output1_) {
        if output0.len() == CHUNK_SIZE && output1.len() == CHUNK_SIZE {
            let mut tmp = [0; CHUNK_SIZE];
            // This part can only be done sequentially
            // I think this is preventing further compiler optimizations
            tmp[0] = iter(*last);
            for i in 1..CHUNK_SIZE {
                tmp[i] = iter(tmp[i - 1]);
            }
            let tmpf = tmp.map(|v| (v as f32) * 4.6566128e-12);
            output0.copy_from_slice(&tmpf);
            output1.copy_from_slice(&tmpf);
            *last = tmp[CHUNK_SIZE - 1];
        } else {
            noise_slice(last, [output0, output1]);
        }
    }
}

/// Instead of having sequential noise, we use parallel noise
#[inline(never)]
fn noise_vectorizable2<const CHUNK_SIZE: usize, const N: usize>(
    last: &mut [i32; CHUNK_SIZE],
    outputs: [&mut [f32; N]; 2],
) {
    let [output0_, output1_] = outputs.map(|i| i.chunks_mut(CHUNK_SIZE));
    for (output0, output1) in output0_.zip(output1_) {
        if output0.len() == CHUNK_SIZE && output1.len() == CHUNK_SIZE {
            let mut tmp = last.map(|v| i32::wrapping_add(i32::wrapping_mul(1103515245, v), 12345));
            let tmpf = tmp.map(|v| (v as f32) * 4.6566128e-12);
            output0.copy_from_slice(&tmpf);
            output1.copy_from_slice(&tmpf);
            *last = tmp;
        } else {
            let mut tmp = last.map(|v| i32::wrapping_add(i32::wrapping_mul(1103515245, v), 12345));
            let tmpf = tmp.map(|v| (v as f32) * 4.6566128e-12);
            output0.copy_from_slice(&tmpf[..output0.len()]);
            output1.copy_from_slice(&tmpf[..output1.len()]);
            *last = tmp;
        }
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
    group.bench_function("vectorizable", |b| {
        let mut last = 0;
        b.iter(|| {
            noise_vectorizable::<4, 4096>(&mut last, [black_box(&mut buf0), black_box(&mut buf1)])
        })
    });
    group.bench_function("vectorizable (chunk=8)", |b| {
        let mut last = 0;
        b.iter(|| {
            noise_vectorizable::<8, 4096>(&mut last, [black_box(&mut buf0), black_box(&mut buf1)])
        })
    });
    group.bench_function("vectorizable2", |b| {
        let mut last = [0, 1, 2, 3];
        b.iter(|| noise_vectorizable2(&mut last, [black_box(&mut buf0), black_box(&mut buf1)]))
    });
    group.bench_function("vectorizable2 (chunk=8)", |b| {
        let mut last = [0, 1, 2, 3, 4, 5, 6, 7];
        b.iter(|| noise_vectorizable2(&mut last, [black_box(&mut buf0), black_box(&mut buf1)]))
    });
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
