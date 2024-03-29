//! https://godbolt.org/z/a3Yj6rbPq

use std::hint::black_box;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

type F32 = f32;

#[allow(non_snake_case)]
#[cfg_attr(feature = "reprc", repr(C))]
struct mydsp {
    IOTA0: i32,
    iVec0: [i32; 1024],
    iRec0: [i32; 2],
    fSampleRate: i32,
}

impl mydsp {
    fn new() -> Self {
        Self {
            IOTA0: 0,
            iVec0: [0; 1024],
            iRec0: [0; 2],
            fSampleRate: 0,
        }
    }

    #[allow(non_snake_case)]
    #[inline(never)]
    fn rms_faust(&mut self, count: i32, inputs: &[&[F32]], outputs: &mut [&mut [F32]]) {
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
            let mut iTemp0: i32 = (1048576.0 * mydsp_faustpower2_f(*input0)) as i32;
            self.iVec0[(self.IOTA0 & 1023) as usize] = iTemp0;
            self.iRec0[0] = i32::wrapping_sub(
                i32::wrapping_add(self.iRec0[1], iTemp0),
                self.iVec0[((i32::wrapping_sub(self.IOTA0, 1000)) & 1023) as usize],
            );
            *output0 = F32::sqrt(9.536744e-10 * (self.iRec0[0]) as F32);
            self.IOTA0 = i32::wrapping_add(self.IOTA0, 1);
            self.iRec0[1] = self.iRec0[0];
        }
    }
}

#[inline(always)]
fn mydsp_faustpower2_f(value: F32) -> F32 {
    return value * value;
}

#[cfg_attr(feature = "reprc", repr(C))]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
struct mydsp_vec {
    iYec0: [i32; 2048],
    iYec0_idx: i32,
    iYec0_idx_save: i32,
    iRec0_perm: [i32; 4],
    fSampleRate: i32,
}

impl mydsp_vec {
    fn new() -> Self {
        Self {
            iYec0: [0; 2048],
            iYec0_idx: 0,
            iYec0_idx_save: 0,
            iRec0_perm: [0; 4],
            fSampleRate: 0,
        }
    }

    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[inline(never)]
    fn compute_original_4(&mut self, count: i32, inputs: &[&[F32]], outputs: &mut [&mut [F32]]) {
        const vsize: i32 = 4;
        let (inputs0) = if let [inputs0, ..] = inputs {
            let inputs0 = inputs0[..count as usize].chunks(vsize as usize);
            (inputs0)
        } else {
            panic!("wrong number of inputs");
        };
        let (outputs0) = if let [outputs0, ..] = outputs {
            let outputs0 = outputs0[..count as usize].chunks_mut(vsize as usize);
            (outputs0)
        } else {
            panic!("wrong number of outputs");
        };
        let mut iRec0_tmp: [i32; 8] = [0; 8];
        /* Main loop */
        let zipped_iterators = inputs0.zip(outputs0);
        for (input0, output0) in zipped_iterators {
            /* Vectorizable loop 0 */
            /* Pre code */
            self.iYec0_idx = (i32::wrapping_add(self.iYec0_idx, self.iYec0_idx_save)) & 1023;
            /* Compute code */
            for i in 0..vsize {
                self.iYec0[((i32::wrapping_add(i, self.iYec0_idx)) & 1023) as usize] =
                    (1048576.0 * mydsp_faustpower2_f(input0[i as usize])) as i32;
            }
            /* Post code */
            self.iYec0_idx_save = vsize;
            /* Recursive loop 1 */
            /* Pre code */
            for j0 in 0..4 {
                iRec0_tmp[j0 as usize] = self.iRec0_perm[j0 as usize];
            }
            /* Compute code */
            for i in 0..vsize {
                iRec0_tmp[(i32::wrapping_add(4, i)) as usize] = i32::wrapping_sub(
                    i32::wrapping_add(
                        iRec0_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize],
                        self.iYec0[((i32::wrapping_add(i, self.iYec0_idx)) & 1023) as usize],
                    ),
                    self.iYec0[((i32::wrapping_sub(i32::wrapping_add(i, self.iYec0_idx), 1000))
                        & 1023) as usize],
                );
            }
            /* Post code */
            for j1 in 0..4 {
                self.iRec0_perm[j1 as usize] = iRec0_tmp[(i32::wrapping_add(vsize, j1)) as usize];
            }
            /* Vectorizable loop 2 */
            /* Compute code */
            for i in 0..vsize {
                output0[i as usize] = F32::sqrt(
                    9.536744e-10 * (iRec0_tmp[(i32::wrapping_add(4, i)) as usize]) as F32,
                );
            }
        }
    }

    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[inline(never)]
    fn compute_original_32(&mut self, count: i32, inputs: &[&[F32]], outputs: &mut [&mut [F32]]) {
        const vsize: i32 = 32;
        let (inputs0) = if let [inputs0, ..] = inputs {
            let inputs0 = inputs0[..count as usize].chunks(vsize as usize);
            (inputs0)
        } else {
            panic!("wrong number of inputs");
        };
        let (outputs0) = if let [outputs0, ..] = outputs {
            let outputs0 = outputs0[..count as usize].chunks_mut(vsize as usize);
            (outputs0)
        } else {
            panic!("wrong number of outputs");
        };
        let mut iRec0_tmp: [i32; 36] = [0; 36];
        /* Main loop */
        let zipped_iterators = inputs0.zip(outputs0);
        for (input0, output0) in zipped_iterators {
            /* Vectorizable loop 0 */
            /* Pre code */
            self.iYec0_idx = (i32::wrapping_add(self.iYec0_idx, self.iYec0_idx_save)) & 2047;
            /* Compute code */
            for i in 0..vsize {
                self.iYec0[((i32::wrapping_add(i, self.iYec0_idx)) & 2047) as usize] =
                    (1048576.0 * mydsp_faustpower2_f(input0[i as usize])) as i32;
            }
            /* Post code */
            self.iYec0_idx_save = vsize;
            /* Recursive loop 1 */
            /* Pre code */
            for j0 in 0..4 {
                iRec0_tmp[j0 as usize] = self.iRec0_perm[j0 as usize];
            }
            /* Compute code */
            for i in 0..vsize {
                iRec0_tmp[(i32::wrapping_add(4, i)) as usize] = i32::wrapping_sub(
                    i32::wrapping_add(
                        iRec0_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize],
                        self.iYec0[((i32::wrapping_add(i, self.iYec0_idx)) & 2047) as usize],
                    ),
                    self.iYec0[((i32::wrapping_sub(i32::wrapping_add(i, self.iYec0_idx), 1000))
                        & 2047) as usize],
                );
            }
            /* Post code */
            for j1 in 0..4 {
                self.iRec0_perm[j1 as usize] = iRec0_tmp[(i32::wrapping_add(vsize, j1)) as usize];
            }
            /* Vectorizable loop 2 */
            /* Compute code */
            for i in 0..vsize {
                output0[i as usize] = F32::sqrt(
                    9.536744e-10 * (iRec0_tmp[(i32::wrapping_add(4, i)) as usize]) as F32,
                );
            }
        }
    }
}

pub fn criterion_benchmark(c: &mut Criterion) {
    const SIZE: usize = 4096;
    let mut group = c.benchmark_group("rms");
    group.throughput(Throughput::Elements(SIZE as u64));
    let mut dsp = mydsp::new();
    let buf0 = [0.0; SIZE];
    let mut buf1 = [0.0; SIZE];

    group.bench_function("original linear", |b| {
        b.iter(|| {
            black_box(mydsp::rms_faust(
                black_box(&mut dsp),
                black_box(SIZE as i32),
                black_box(&[&buf0]),
                black_box(&mut [&mut buf1]),
            ))
        })
    });

    let mut dsp_vec = mydsp_vec::new();
    group.bench_with_input(BenchmarkId::new("original vec", 4), &4, |b, i| {
        b.iter(|| {
            black_box(mydsp_vec::compute_original_4(
                black_box(&mut dsp_vec),
                black_box(4096),
                black_box(&[&buf0]),
                black_box(&mut [&mut buf1]),
            ))
        })
    });
    group.bench_with_input(BenchmarkId::new("original vec", 32), &32, |b, i| {
        b.iter(|| {
            black_box(mydsp_vec::compute_original_32(
                black_box(&mut dsp_vec),
                black_box(4096),
                black_box(&[&buf0]),
                black_box(&mut [&mut buf1]),
            ))
        })
    });
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
