//! https://godbolt.org/z/Pn7daf8sf

use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion, Throughput};

type F32 = f32;

#[inline(never)]
fn copy1(input: &[F32], output: &mut [F32]) {
    output.copy_from_slice(input);
}

#[allow(non_snake_case)]
#[allow(unused_mut)]
#[inline(never)]
fn copy1_faust(count: i32, inputs: &[&[F32]], outputs: &mut [&mut [F32]]) {
    let (inputs0) = if let [inputs0, ..] = inputs {
        let inputs0 = inputs0[..count as usize].iter();
        (inputs0)
    } else {
        panic!("wrong number of inputs");
    };
    let (outputs0) = if let [outputs0, ..] = outputs {
        let outputs0 = outputs0[..count as usize].iter_mut();
        (outputs0)
    } else {
        panic!("wrong number of outputs");
    };
    let zipped_iterators = inputs0.zip(outputs0);
    for (input0, output0) in zipped_iterators {
        *output0 = *input0;
    }
}

#[allow(non_snake_case)]
#[allow(unused_mut)]
#[inline(never)]
fn copy1_faust_vec<const CHUNK_SIZE: i32>(
    count: i32,
    inputs: &[&[F32]],
    outputs: &mut [&mut [F32]],
) {
    let mut input0_ptr: &[F32] = inputs[0];
    let mut output0_ptr: &mut [F32] = outputs[0];
    let mut vindex: i32 = count / CHUNK_SIZE * CHUNK_SIZE;
    /* Main loop */
    for vindex in (0..=i32::wrapping_sub(count, CHUNK_SIZE)).step_by(CHUNK_SIZE as usize) {
        let mut vsize: i32 = CHUNK_SIZE;
        /* Vectorizable loop 0 */
        /* Compute code */
        for i in 0..vsize {
            output0_ptr[(i32::wrapping_add(vindex, i)) as usize] =
                input0_ptr[(i32::wrapping_add(vindex, i)) as usize];
        }
    }
    /* Remaining frames */
    if vindex < count {
        let mut vsize: i32 = i32::wrapping_sub(count, vindex);
        /* Vectorizable loop 0 */
        /* Compute code */
        for i in 0..vsize {
            output0_ptr[(i32::wrapping_add(vindex, i)) as usize] =
                input0_ptr[(i32::wrapping_add(vindex, i)) as usize];
        }
    }
}

#[inline(never)]
fn copy2(inputs: &[&[F32]], outputs: &mut [&mut [F32]]) {
    for (output, input) in outputs.iter_mut().zip(inputs) {
        output.copy_from_slice(input);
    }
}

#[allow(non_snake_case)]
#[allow(unused_mut)]
#[inline(never)]
fn copy2_faust(count: i32, inputs: &[&[F32]], outputs: &mut [&mut [F32]]) {
    let (inputs0, inputs1) = if let [inputs0, inputs1, ..] = inputs {
        let inputs0 = inputs0[..count as usize].iter();
        let inputs1 = inputs1[..count as usize].iter();
        (inputs0, inputs1)
    } else {
        panic!("wrong number of inputs");
    };
    let (outputs0, outputs1) = if let [outputs0, outputs1, ..] = outputs {
        let outputs0 = outputs0[..count as usize].iter_mut();
        let outputs1 = outputs1[..count as usize].iter_mut();
        (outputs0, outputs1)
    } else {
        panic!("wrong number of outputs");
    };
    let zipped_iterators = inputs0.zip(inputs1).zip(outputs0).zip(outputs1);
    for (((input0, input1), output0), output1) in zipped_iterators {
        *output0 = *input0;
        *output1 = *input1;
    }
}

#[allow(non_snake_case)]
#[allow(unused_mut)]
#[inline(never)]
fn copy2_faust_vec<const CHUNK_SIZE: i32>(
    count: i32,
    inputs: &[&[F32]],
    outputs: &mut [&mut [F32]],
) {
    let mut input0_ptr: &[F32] = inputs[0];
    let mut input1_ptr: &[F32] = inputs[1];
    let (output0_ptr, output1_ptr) = if let [outputs0, outputs1, ..] = outputs {
        let outputs0 = &mut outputs0[..count as usize];
        let outputs1 = &mut outputs1[..count as usize];
        (outputs0, outputs1)
    } else {
        panic!("wrong number of outputs");
    };
    // let mut output0_ptr: &mut [F32] = outputs[0];
    // let mut output1_ptr: &mut [F32] = outputs[1];
    let mut vindex: i32 = count / CHUNK_SIZE * CHUNK_SIZE;
    /* Main loop */
    for vindex in (0..=i32::wrapping_sub(count, CHUNK_SIZE)).step_by(CHUNK_SIZE as usize) {
        let mut vsize: i32 = CHUNK_SIZE;
        /* Vectorizable loop 0 */
        /* Compute code */
        for i in 0..vsize {
            output1_ptr[(i32::wrapping_add(vindex, i)) as usize] =
                input1_ptr[(i32::wrapping_add(vindex, i)) as usize];
        }
        /* Vectorizable loop 1 */
        /* Compute code */
        for i in 0..vsize {
            output0_ptr[(i32::wrapping_add(vindex, i)) as usize] =
                input0_ptr[(i32::wrapping_add(vindex, i)) as usize];
        }
    }
    /* Remaining frames */
    if vindex < count {
        let mut vsize: i32 = i32::wrapping_sub(count, vindex);
        /* Vectorizable loop 0 */
        /* Compute code */
        for i in 0..vsize {
            output1_ptr[(i32::wrapping_add(vindex, i)) as usize] =
                input1_ptr[(i32::wrapping_add(vindex, i)) as usize];
        }
        /* Vectorizable loop 1 */
        /* Compute code */
        for i in 0..vsize {
            output0_ptr[(i32::wrapping_add(vindex, i)) as usize] =
                input0_ptr[(i32::wrapping_add(vindex, i)) as usize];
        }
    }
}

pub fn criterion_benchmark(c: &mut Criterion) {
    const SIZE: usize = 4096;
    let mut group = c.benchmark_group("copy1");
    group.throughput(Throughput::Elements(SIZE as u64));
    let buf0 = [0.0; SIZE];
    let mut buf1 = [0.0; SIZE];

    group.bench_function("best", |b| {
        b.iter(|| black_box(copy1(black_box(&buf0), black_box(&mut buf1))))
    });
    group.bench_function("faust linear", |b| {
        b.iter(|| {
            black_box(copy1_faust(
                black_box(SIZE as i32),
                black_box(&[&buf0]),
                black_box(&mut [&mut buf1]),
            ))
        })
    });
    group.bench_function("faust vec/4", |b| {
        b.iter(|| {
            black_box(copy1_faust_vec::<4>(
                black_box(SIZE as i32),
                black_box(&[&buf0]),
                black_box(&mut [&mut buf1]),
            ))
        })
    });
    group.bench_function("faust vec/32", |b| {
        b.iter(|| {
            black_box(copy1_faust_vec::<32>(
                black_box(SIZE as i32),
                black_box(&[&buf0]),
                black_box(&mut [&mut buf1]),
            ))
        })
    });
    group.finish();

    let mut group = c.benchmark_group("copy2");
    group.throughput(Throughput::Elements(SIZE as u64));
    let buf2 = [0.0; SIZE];
    let mut buf3 = [0.0; SIZE];

    group.bench_function("best", |b| {
        b.iter(|| {
            black_box(copy2(
                black_box(&[&buf0, &buf2]),
                black_box(&mut [&mut buf1, &mut buf3]),
            ))
        })
    });
    group.bench_function("faust linear", |b| {
        b.iter(|| {
            black_box(copy2_faust(
                black_box(SIZE as i32),
                black_box(&[&buf0, &buf2]),
                black_box(&mut [&mut buf1, &mut buf3]),
            ))
        })
    });
    group.bench_function("faust vec/4", |b| {
        b.iter(|| {
            black_box(copy2_faust_vec::<4>(
                black_box(SIZE as i32),
                black_box(&[&buf0, &buf2]),
                black_box(&mut [&mut buf1, &mut buf3]),
            ))
        })
    });
    group.bench_function("faust vec/32", |b| {
        b.iter(|| {
            black_box(copy2_faust_vec::<32>(
                black_box(SIZE as i32),
                black_box(&[&buf0, &buf2]),
                black_box(&mut [&mut buf1, &mut buf3]),
            ))
        })
    });
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
