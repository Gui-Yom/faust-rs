//! Stereo LCG noise
//! https://godbolt.org/z/f97zsj1jn

use std::time::Instant;

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use criterion_polyglot::{BenchSpec, CriterionPolyglotExt};

/// Most basic implementation using iterators.
/// Does not get vectorized but gets unrolled 4 times.
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

/// Using a chunking iterator and vectorized noise.
/// Seems to contain much more vector instructions. Still contains a lot of bound checks.
#[inline(never)]
fn noise_slice_chunked<const CHUNK_SIZE: usize>(
    last: &mut [i32; CHUNK_SIZE],
    outputs: [&mut [f32]; 2],
) {
    let [output0_, output1_] = outputs.map(|i| i.chunks_mut(CHUNK_SIZE));
    for (output0, output1) in output0_.zip(output1_) {
        let mut tmp = last.map(|v| i32::wrapping_add(i32::wrapping_mul(1103515245, v), 12345));
        let tmpf = tmp.map(|v| (v as f32) * 4.6566128e-12);
        output0.copy_from_slice(&tmpf);
        output1.copy_from_slice(&tmpf);
        *last = tmp;
    }
}

/// Using an array as output, can probably elide bound checks much better.
/// No vector instructions.
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

/// Chunking iterator and array output.
/// The sequential noise is preventing effective vectorization.
#[inline(never)]
fn noise_array_chunked<const CHUNK_SIZE: usize, const N: usize>(
    last: &mut i32,
    outputs: [&mut [f32; N]; 2],
) {
    #[inline(always)]
    fn iter(last: i32) -> i32 {
        i32::wrapping_add(i32::wrapping_mul(1103515245, last), 12345)
    }

    let [output0_, output1_] = outputs.map(|i| i.chunks_mut(CHUNK_SIZE));
    for (output0, output1) in output0_.zip(output1_) {
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
    }
}

/// Chunking iterator, array output and parallel noise.
/// Best version yet.
#[inline(never)]
fn noise_array_chunked_vec<const CHUNK_SIZE: usize, const N: usize>(
    last: &mut [i32; CHUNK_SIZE],
    outputs: [&mut [f32; N]; 2],
) {
    let [output0_, output1_] = outputs.map(|i| i.chunks_mut(CHUNK_SIZE));
    for (output0, output1) in output0_.zip(output1_) {
        let mut tmp = last.map(|v| i32::wrapping_add(i32::wrapping_mul(1103515245, v), 12345));
        let tmpf = tmp.map(|v| (v as f32) * 4.6566128e-12);
        output0.copy_from_slice(&tmpf);
        output1.copy_from_slice(&tmpf);
        *last = tmp;
    }
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("noise");
    const SIZE: usize = 4096;
    let mut buf0 = [0.0; SIZE];
    let mut buf1 = [0.0; SIZE];
    group.throughput(Throughput::Elements(SIZE as u64));

    group.bench_function("slice", |b| {
        let mut last = 0;
        b.iter(|| noise_slice(&mut last, [black_box(&mut buf0), black_box(&mut buf1)]))
    });

    for chunk_size in [4, 8] {
        group.bench_with_input(
            BenchmarkId::new("slice chunked", chunk_size),
            &chunk_size,
            |b, &i| match i {
                4 => {
                    // Use different seeds or else we are just going to generate the same noise over 4 samples
                    let mut last = [0, 1, 2, 3];
                    b.iter(|| {
                        noise_slice_chunked(&mut last, [black_box(&mut buf0), black_box(&mut buf1)])
                    })
                }
                8 => {
                    let mut last = [0, 1, 2, 3, 4, 5, 6, 7];
                    b.iter(|| {
                        noise_slice_chunked(&mut last, [black_box(&mut buf0), black_box(&mut buf1)])
                    })
                }
                _ => {
                    unreachable!()
                }
            },
        );
    }

    group.bench_function("array", |b| {
        let mut last = 0;
        b.iter(|| noise_array(&mut last, [black_box(&mut buf0), black_box(&mut buf1)]))
    });

    for chunk_size in [4, 8] {
        group.bench_with_input(
            BenchmarkId::new("array chunked", chunk_size),
            &chunk_size,
            |b, i| match i {
                4 => b.iter(|| {
                    noise_array_chunked::<4, SIZE>(
                        &mut 0,
                        [black_box(&mut buf0), black_box(&mut buf1)],
                    )
                }),
                8 => b.iter(|| {
                    noise_array_chunked::<8, SIZE>(
                        &mut 0,
                        [black_box(&mut buf0), black_box(&mut buf1)],
                    )
                }),
                _ => {
                    unreachable!()
                }
            },
        );
    }

    for chunk_size in [4, 8] {
        group.bench_with_input(
            BenchmarkId::new("array chunked vec", chunk_size),
            &chunk_size,
            |b, &i| match i {
                4 => {
                    // Use different seeds or else we are just going to generate the same noise over 4 samples
                    let mut last = [0, 1, 2, 3];
                    b.iter(|| {
                        noise_array_chunked_vec(
                            &mut last,
                            [black_box(&mut buf0), black_box(&mut buf1)],
                        )
                    })
                }
                8 => {
                    let mut last = [0, 1, 2, 3, 4, 5, 6, 7];
                    b.iter(|| {
                        noise_array_chunked_vec(
                            &mut last,
                            [black_box(&mut buf0), black_box(&mut buf1)],
                        )
                    })
                }
                _ => {
                    unreachable!()
                }
            },
        );
    }

    group.c_benchmark_with_builder(
        "c scalar",
        BenchSpec::new(
            r#"
        FAUSTFLOAT* output0 = outputs[0];
        FAUSTFLOAT* output1 = outputs[1];
        int count = 4096;
        /* C99 loop */
        {
                int i0;
                for (i0 = 0; i0 < count; i0 = i0 + 1) {
                        dsp->iRec0[0] = 1103515245 * dsp->iRec0[1] + 12345;
                        float fTemp0 = 4.656613e-10f * (float)(dsp->iRec0[0]);
                        output0[i0] = (FAUSTFLOAT)(fTemp0);
                        output1[i0] = (FAUSTFLOAT)(fTemp0);
                        dsp->iRec0[1] = dsp->iRec0[0];
                }
        }
        "#,
        )
        .with_declarations(
            r#"
        typedef float FAUSTFLOAT;
        
        typedef struct {
            int iRec0[2];
            int fSampleRate;
        } mydsp;
        
        mydsp* newmydsp() {
            mydsp* dsp = (mydsp*)calloc(1, sizeof(mydsp));
            return dsp;
        }
        "#,
        )
        .with_global_init(
            r#"
        mydsp* dsp = newmydsp();
        FAUSTFLOAT output0_[4096];
        FAUSTFLOAT output1_[4096];
        FAUSTFLOAT* outputs[2] = { output0_, output1_ };
        "#,
        ),
        criterion_polyglot::cc::Build::new(),
    );

    group.c_benchmark_with_builder(
        "c vec",
        BenchSpec::new(
            r#"
        FAUSTFLOAT* output0_ptr = outputs[0];
        FAUSTFLOAT* output1_ptr = outputs[1];
        int count = 4096;
        int iRec0_tmp[36];
        int* iRec0 = &iRec0_tmp[4];
        float fZec0[32];
        int vindex = 0;
        /* Main loop */
        for (vindex = 0; vindex <= (count - 32); vindex = vindex + 32) {
                FAUSTFLOAT* output0 = &output0_ptr[vindex];
                FAUSTFLOAT* output1 = &output1_ptr[vindex];
                int vsize = 32;
    /* Recursive loop 0 */
    /* Pre code */
    /* C99 loop */
                {
                        int j0;
                        for (j0 = 0; j0 < 4; j0 = j0 + 1) {
                                iRec0_tmp[j0] = dsp->iRec0_perm[j0];
                        }
                }
    /* Compute code */
    /* C99 loop */
                {
                        int i;
                        for (i = 0; i < vsize; i = i + 1) {
                                iRec0[i] = 1103515245 * iRec0[i - 1] + 12345;
                        }
                }
    /* Post code */
    /* C99 loop */
                {
                        int j1;
                        for (j1 = 0; j1 < 4; j1 = j1 + 1) {
                                dsp->iRec0_perm[j1] = iRec0_tmp[vsize + j1];
                        }
                }
    /* Vectorizable loop 1 */
    /* Compute code */
    /* C99 loop */
                {
                        int i;
                        for (i = 0; i < vsize; i = i + 1) {
                                fZec0[i] = 4.656613e-10f * (float)(iRec0[i]);
                        }
                }
    /* Vectorizable loop 2 */
    /* Compute code */
    /* C99 loop */
                {
                        int i;
                        for (i = 0; i < vsize; i = i + 1) {
                                output0[i] = (FAUSTFLOAT)(fZec0[i]);
                        }
                }
    /* Vectorizable loop 3 */
    /* Compute code */
    /* C99 loop */
                {
                        int i;
                        for (i = 0; i < vsize; i = i + 1) {
                                output1[i] = (FAUSTFLOAT)(fZec0[i]);
                        }
                }
        }
    /* Remaining frames */
        if ((vindex < count)) {
                FAUSTFLOAT* output0 = &output0_ptr[vindex];
                FAUSTFLOAT* output1 = &output1_ptr[vindex];
                int vsize = count - vindex;
    /* Recursive loop 0 */
    /* Pre code */
    /* C99 loop */
                {
                        int j0;
                        for (j0 = 0; j0 < 4; j0 = j0 + 1) {
                                iRec0_tmp[j0] = dsp->iRec0_perm[j0];
                        }
                }
    /* Compute code */
    /* C99 loop */
                {
                        int i;
                        for (i = 0; i < vsize; i = i + 1) {
                                iRec0[i] = 1103515245 * iRec0[i - 1] + 12345;
                        }
                }
    /* Post code */
    /* C99 loop */
                {
                        int j1;
                        for (j1 = 0; j1 < 4; j1 = j1 + 1) {
                                dsp->iRec0_perm[j1] = iRec0_tmp[vsize + j1];
                        }
                }
    /* Vectorizable loop 1 */
    /* Compute code */
    /* C99 loop */
                {
                        int i;
                        for (i = 0; i < vsize; i = i + 1) {
                                fZec0[i] = 4.656613e-10f * (float)(iRec0[i]);
                        }
                }
    /* Vectorizable loop 2 */
    /* Compute code */
    /* C99 loop */
                {
                        int i;
                        for (i = 0; i < vsize; i = i + 1) {
                                output0[i] = (FAUSTFLOAT)(fZec0[i]);
                        }
                }
    /* Vectorizable loop 3 */
    /* Compute code */
    /* C99 loop */
                {
                        int i;
                        for (i = 0; i < vsize; i = i + 1) {
                                output1[i] = (FAUSTFLOAT)(fZec0[i]);
                        }
                }
        }
    "#,
        )
        .with_declarations(
            r#"
        typedef float FAUSTFLOAT;
        
        typedef struct {
            int iRec0_perm[4];
            int fSampleRate;
        } mydsp;
        
        mydsp* newmydsp() {
            mydsp* dsp = (mydsp*)calloc(1, sizeof(mydsp));
            return dsp;
        }
        "#,
        )
        .with_global_init(
            r#"
        mydsp* dsp = newmydsp();
        FAUSTFLOAT output0_[4096];
        FAUSTFLOAT output1_[4096];
        FAUSTFLOAT* outputs[2] = { output0_, output1_ };
        "#,
        ),
        criterion_polyglot::cc::Build::default(),
    );

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
