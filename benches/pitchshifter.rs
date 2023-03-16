//! Pitchshifter
//! https://godbolt.org/z/bbM8jPhPf

use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion, Throughput};

pub struct mydsp {
    fHslider0: f32,
    fHslider1: f32,
    fRec0: [f32; 2],
    fHslider2: f32,
    IOTA0: i32,
    fVec0: [f32; 131072],
}

impl mydsp {
    fn new() -> mydsp {
        mydsp {
            fHslider0: 0.0,
            fHslider1: 0.0,
            fRec0: [0.0; 2],
            fHslider2: 0.0,
            IOTA0: 0,
            fVec0: [0.0; 131072],
        }
    }

    #[inline(never)]
    fn compute_original(&mut self, count: i32, inputs: &[&[f32]], outputs: &mut [&mut [f32]]) {
        let inputs0 = if let [inputs0, ..] = inputs {
            let inputs0 = inputs0[..count as usize].iter();
            inputs0
        } else {
            panic!("wrong number of inputs");
        };
        let outputs0 = if let [outputs0, ..] = outputs {
            let outputs0 = outputs0[..count as usize].iter_mut();
            outputs0
        } else {
            panic!("wrong number of outputs");
        };
        let fSlow0 = f32::powf(2.0, 0.083333336 * self.fHslider0);
        let fSlow1 = self.fHslider1;
        let fSlow2 = 1.0 / self.fHslider2;
        let zipped_iterators = inputs0.zip(outputs0);
        for (input0, output0) in zipped_iterators {
            self.fRec0[0] = (fSlow1 + self.fRec0[1] + 1.0 - fSlow0) % fSlow1;
            let fTemp0 = f32::min(fSlow2 * self.fRec0[0], 1.0);
            let fTemp1 = *input0;
            self.fVec0[(self.IOTA0 & 131071) as usize] = fTemp1;
            let fTemp2 = fSlow1 + self.fRec0[0];
            let iTemp3: i32 = (fTemp2) as i32;
            let fTemp4 = f32::floor(fTemp2);
            let fTemp5 = 1.0 - self.fRec0[0];
            let iTemp6: i32 = (self.fRec0[0]) as i32;
            let fTemp7 = f32::floor(self.fRec0[0]);
            *output0 = (self.fVec0[((i32::wrapping_sub(
                self.IOTA0,
                std::cmp::min(65537, std::cmp::max(0, iTemp6)),
            )) & 131071) as usize]
                * (fTemp7 + fTemp5)
                + (self.fRec0[0] - fTemp7)
                    * self.fVec0[((i32::wrapping_sub(
                        self.IOTA0,
                        std::cmp::min(65537, std::cmp::max(0, i32::wrapping_add(iTemp6, 1))),
                    )) & 131071) as usize])
                * fTemp0
                + (self.fVec0[((i32::wrapping_sub(
                    self.IOTA0,
                    std::cmp::min(65537, std::cmp::max(0, iTemp3)),
                )) & 131071) as usize]
                    * (fTemp4 + fTemp5 - fSlow1)
                    + (fSlow1 + (self.fRec0[0] - fTemp4))
                        * self.fVec0[((i32::wrapping_sub(
                            self.IOTA0,
                            std::cmp::min(65537, std::cmp::max(0, i32::wrapping_add(iTemp3, 1))),
                        )) & 131071) as usize])
                    * (1.0 - fTemp0);
            self.fRec0[1] = self.fRec0[0];
            self.IOTA0 = i32::wrapping_add(self.IOTA0, 1);
        }
    }

    #[inline(never)]
    fn compute_slice(&mut self, input: &[f32], output: &mut [f32]) {
        let fSlow0 = f32::powf(2.0, 0.083333336 * self.fHslider0);
        let fSlow1 = self.fHslider1;
        let fSlow2 = 1.0 / self.fHslider2;
        for (&input, output) in input.iter().zip(output) {
            self.fRec0[0] = (fSlow1 + self.fRec0[1] + 1.0 - fSlow0) % fSlow1;
            let fTemp0 = f32::min(fSlow2 * self.fRec0[0], 1.0);
            let fTemp1 = input;
            self.fVec0[(self.IOTA0 & 131071) as usize] = fTemp1;
            let fTemp2 = fSlow1 + self.fRec0[0];
            let iTemp3: i32 = (fTemp2) as i32;
            let fTemp4 = f32::floor(fTemp2);
            let fTemp5 = 1.0 - self.fRec0[0];
            let iTemp6: i32 = (self.fRec0[0]) as i32;
            let fTemp7 = f32::floor(self.fRec0[0]);
            *output = (self.fVec0[((i32::wrapping_sub(
                self.IOTA0,
                std::cmp::min(65537, std::cmp::max(0, iTemp6)),
            )) & 131071) as usize]
                * (fTemp7 + fTemp5)
                + (self.fRec0[0] - fTemp7)
                    * self.fVec0[((i32::wrapping_sub(
                        self.IOTA0,
                        std::cmp::min(65537, std::cmp::max(0, i32::wrapping_add(iTemp6, 1))),
                    )) & 131071) as usize])
                * fTemp0
                + (self.fVec0[((i32::wrapping_sub(
                    self.IOTA0,
                    std::cmp::min(65537, std::cmp::max(0, iTemp3)),
                )) & 131071) as usize]
                    * (fTemp4 + fTemp5 - fSlow1)
                    + (fSlow1 + (self.fRec0[0] - fTemp4))
                        * self.fVec0[((i32::wrapping_sub(
                            self.IOTA0,
                            std::cmp::min(65537, std::cmp::max(0, i32::wrapping_add(iTemp3, 1))),
                        )) & 131071) as usize])
                    * (1.0 - fTemp0);
            self.fRec0[1] = self.fRec0[0];
            self.IOTA0 = i32::wrapping_add(self.IOTA0, 1);
        }
    }

    #[inline(never)]
    fn compute_arr<const N: usize>(&mut self, input: &[f32; N], output: &mut [f32; N]) {
        let fSlow0 = f32::powf(2.0, 0.083333336 * self.fHslider0);
        let fSlow1 = self.fHslider1;
        let fSlow2 = 1.0 / self.fHslider2;
        for (&input, output) in input.iter().zip(output) {
            self.fRec0[0] = (fSlow1 + self.fRec0[1] + 1.0 - fSlow0) % fSlow1;
            let fTemp0 = f32::min(fSlow2 * self.fRec0[0], 1.0);
            let fTemp1 = input;
            self.fVec0[(self.IOTA0 & 131071) as usize] = fTemp1;
            let fTemp2 = fSlow1 + self.fRec0[0];
            let iTemp3: i32 = (fTemp2) as i32;
            let fTemp4 = f32::floor(fTemp2);
            let fTemp5 = 1.0 - self.fRec0[0];
            let iTemp6: i32 = (self.fRec0[0]) as i32;
            let fTemp7 = f32::floor(self.fRec0[0]);
            *output = (self.fVec0[((i32::wrapping_sub(
                self.IOTA0,
                std::cmp::min(65537, std::cmp::max(0, iTemp6)),
            )) & 131071) as usize]
                * (fTemp7 + fTemp5)
                + (self.fRec0[0] - fTemp7)
                    * self.fVec0[((i32::wrapping_sub(
                        self.IOTA0,
                        std::cmp::min(65537, std::cmp::max(0, i32::wrapping_add(iTemp6, 1))),
                    )) & 131071) as usize])
                * fTemp0
                + (self.fVec0[((i32::wrapping_sub(
                    self.IOTA0,
                    std::cmp::min(65537, std::cmp::max(0, iTemp3)),
                )) & 131071) as usize]
                    * (fTemp4 + fTemp5 - fSlow1)
                    + (fSlow1 + (self.fRec0[0] - fTemp4))
                        * self.fVec0[((i32::wrapping_sub(
                            self.IOTA0,
                            std::cmp::min(65537, std::cmp::max(0, i32::wrapping_add(iTemp3, 1))),
                        )) & 131071) as usize])
                    * (1.0 - fTemp0);
            self.fRec0[1] = self.fRec0[0];
            self.IOTA0 = i32::wrapping_add(self.IOTA0, 1);
        }
    }
}

pub struct mydsp_vec {
    fHslider0: f32,
    fHslider1: f32,
    fRec0_perm: [f32; 4],
    fHslider2: f32,
    fYec0: [f32; 131072],
    fYec0_idx: i32,
    fYec0_idx_save: i32,
    fSampleRate: i32,
}

impl mydsp_vec {
    fn new() -> Self {
        Self {
            fHslider0: 0.0,
            fHslider1: 0.0,
            fRec0_perm: [0.0; 4],
            fHslider2: 0.0,
            fYec0: [0.0; 131072],
            fYec0_idx: 0,
            fYec0_idx_save: 0,
            fSampleRate: 0,
        }
    }

    #[inline(never)]
    fn compute_original(&mut self, count: i32, inputs: &[&[f32]], outputs: &mut [&mut [f32]]) {
        let mut input0_ptr = inputs[0];
        let mut output0_ptr = outputs[0];
        let mut fSlow0: f32 = f32::powf(2.0, 0.083333336 * self.fHslider0);
        let mut fSlow1: f32 = self.fHslider1;
        let mut fRec0_tmp: [f32; 36] = [0.0; 36];
        let mut fRec0 = &mut fRec0_tmp[..4];
        let mut fSlow2: f32 = 1.0 / self.fHslider2;
        let mut fZec0: [f32; 32] = [0.0; 32];
        let mut fZec1: [f32; 32] = [0.0; 32];
        let mut iZec2: [i32; 32] = [0; 32];
        let mut fZec3: [f32; 32] = [0.0; 32];
        let mut fZec4: [f32; 32] = [0.0; 32];
        let mut iZec5: [i32; 32] = [0; 32];
        let mut fZec6: [f32; 32] = [0.0; 32];
        let mut vindex: i32 = 0;
        /* Main loop */
        vindex = 0;
        loop {
            let mut input0 = &input0_ptr[(vindex) as usize];
            let mut output0 = &mut output0_ptr[(vindex) as usize];
            let mut vsize: i32 = 32;
            /* Recursive loop 0 */
            /* Pre code */
            let mut j0: i32 = 0;
            loop {
                fRec0_tmp[(j0) as usize] = self.fRec0_perm[(j0) as usize];
                j0 = i32::wrapping_add(j0, 1);
                if j0 < 4 {
                    continue;
                } else {
                    break;
                }
            }
            /* Compute code */
            let mut i: i32 = 0;
            loop {
                fRec0[(i) as usize] =
                    (fSlow1 + (fRec0[(i32::wrapping_sub(i, 1)) as usize] + 1.0 - fSlow0)) % fSlow1;
                i = i32::wrapping_add(i, 1);
                if i < vsize {
                    continue;
                } else {
                    break;
                }
            }
            /* Post code */
            let mut j1: i32 = 0;
            loop {
                self.fRec0_perm[(j1) as usize] = fRec0_tmp[(i32::wrapping_add(vsize, j1)) as usize];
                j1 = i32::wrapping_add(j1, 1);
                if j1 < 4 {
                    continue;
                } else {
                    break;
                }
            }
            /* Vectorizable loop 1 */
            /* Compute code */
            let mut i: i32 = 0;
            loop {
                fZec1[(i) as usize] = fSlow1 + fRec0[(i) as usize];
                i = i32::wrapping_add(i, 1);
                if i < vsize {
                    continue;
                } else {
                    break;
                }
            }
            /* Vectorizable loop 2 */
            /* Pre code */
            self.fYec0_idx = (i32::wrapping_add(self.fYec0_idx, self.fYec0_idx_save)) & 131071;
            /* Compute code */
            let mut i: i32 = 0;
            loop {
                self.fYec0[((i32::wrapping_add(i, self.fYec0_idx)) & 131071) as usize] = *input0;
                i = i32::wrapping_add(i, 1);
                if i < vsize {
                    continue;
                } else {
                    break;
                }
            }
            /* Post code */
            self.fYec0_idx_save = vsize;
            /* Vectorizable loop 3 */
            /* Compute code */
            let mut i: i32 = 0;
            loop {
                iZec2[(i) as usize] = ((fZec1[(i) as usize]) as i32);
                i = i32::wrapping_add(i, 1);
                if i < vsize {
                    continue;
                } else {
                    break;
                }
            }
            /* Vectorizable loop 4 */
            /* Compute code */
            let mut i: i32 = 0;
            loop {
                fZec0[(i) as usize] = f32::min(fSlow2 * fRec0[(i) as usize], 1.0);
                i = i32::wrapping_add(i, 1);
                if i < vsize {
                    continue;
                } else {
                    break;
                }
            }
            /* Vectorizable loop 5 */
            /* Compute code */
            let mut i: i32 = 0;
            loop {
                fZec4[(i) as usize] = 1.0 - fRec0[(i) as usize];
                i = i32::wrapping_add(i, 1);
                if i < vsize {
                    continue;
                } else {
                    break;
                }
            }
            /* Vectorizable loop 6 */
            /* Compute code */
            let mut i: i32 = 0;
            loop {
                fZec6[(i) as usize] = f32::floor(fRec0[(i) as usize]);
                i = i32::wrapping_add(i, 1);
                if i < vsize {
                    continue;
                } else {
                    break;
                }
            }
            /* Vectorizable loop 7 */
            /* Compute code */
            let mut i: i32 = 0;
            loop {
                fZec3[(i) as usize] = f32::floor(fZec1[(i) as usize]);
                i = i32::wrapping_add(i, 1);
                if i < vsize {
                    continue;
                } else {
                    break;
                }
            }
            /* Vectorizable loop 8 */
            /* Compute code */
            let mut i: i32 = 0;
            loop {
                iZec5[(i) as usize] = ((fRec0[(i) as usize]) as i32);
                i = i32::wrapping_add(i, 1);
                if i < vsize {
                    continue;
                } else {
                    break;
                }
            }
            /* Vectorizable loop 9 */
            /* Compute code */
            let mut i: i32 = 0;
            loop {
                *output0 = (self.fYec0[((i32::wrapping_sub(
                    i32::wrapping_add(i, self.fYec0_idx),
                    std::cmp::min(65537, std::cmp::max(0, iZec5[(i) as usize])),
                )) & 131071) as usize]
                    * (fZec6[(i) as usize] + fZec4[(i) as usize])
                    + (fRec0[(i) as usize] - fZec6[(i) as usize])
                        * self.fYec0[((i32::wrapping_sub(
                            i32::wrapping_add(i, self.fYec0_idx),
                            std::cmp::min(
                                65537,
                                std::cmp::max(0, i32::wrapping_add(iZec5[(i) as usize], 1)),
                            ),
                        )) & 131071) as usize])
                    * fZec0[(i) as usize]
                    + (self.fYec0[((i32::wrapping_sub(
                        i32::wrapping_add(i, self.fYec0_idx),
                        std::cmp::min(65537, std::cmp::max(0, iZec2[(i) as usize])),
                    )) & 131071) as usize]
                        * (fZec3[(i) as usize] + fZec4[(i) as usize] - fSlow1)
                        + (fSlow1 + (fRec0[(i) as usize] - fZec3[(i) as usize]))
                            * self.fYec0[((i32::wrapping_sub(
                                i32::wrapping_add(i, self.fYec0_idx),
                                std::cmp::min(
                                    65537,
                                    std::cmp::max(0, i32::wrapping_add(iZec2[(i) as usize], 1)),
                                ),
                            )) & 131071) as usize])
                        * (1.0 - fZec0[(i) as usize]);
                i = i32::wrapping_add(i, 1);
                if i < vsize {
                    continue;
                } else {
                    break;
                }
            }
            vindex = i32::wrapping_add(vindex, 32);
            if vindex <= i32::wrapping_sub(count, 32) {
                continue;
            } else {
                break;
            }
        }
        /* Remaining frames */
        if (((vindex < count) as i32) as i32 == 1) {
            let mut input0: &f32 = &input0_ptr[(vindex) as usize];
            let mut output0: &mut f32 = &mut output0_ptr[(vindex) as usize];
            let mut vsize: i32 = i32::wrapping_sub(count, vindex);
            /* Recursive loop 0 */
            /* Pre code */
            let mut j0: i32 = 0;
            loop {
                fRec0_tmp[(j0) as usize] = self.fRec0_perm[(j0) as usize];
                j0 = i32::wrapping_add(j0, 1);
                if j0 < 4 {
                    continue;
                } else {
                    break;
                }
            }
            /* Compute code */
            let mut i: i32 = 0;
            loop {
                fRec0[(i) as usize] =
                    (fSlow1 + (fRec0[(i32::wrapping_sub(i, 1)) as usize] + 1.0 - fSlow0)) % fSlow1;
                i = i32::wrapping_add(i, 1);
                if i < vsize {
                    continue;
                } else {
                    break;
                }
            }
            /* Post code */
            let mut j1: i32 = 0;
            loop {
                self.fRec0_perm[(j1) as usize] = fRec0_tmp[(i32::wrapping_add(vsize, j1)) as usize];
                j1 = i32::wrapping_add(j1, 1);
                if j1 < 4 {
                    continue;
                } else {
                    break;
                }
            }
            /* Vectorizable loop 1 */
            /* Compute code */
            let mut i: i32 = 0;
            loop {
                fZec1[(i) as usize] = fSlow1 + fRec0[(i) as usize];
                i = i32::wrapping_add(i, 1);
                if i < vsize {
                    continue;
                } else {
                    break;
                }
            }
            /* Vectorizable loop 2 */
            /* Pre code */
            self.fYec0_idx = (i32::wrapping_add(self.fYec0_idx, self.fYec0_idx_save)) & 131071;
            /* Compute code */
            let mut i: i32 = 0;
            loop {
                self.fYec0[((i32::wrapping_add(i, self.fYec0_idx)) & 131071) as usize] = *input0;
                i = i32::wrapping_add(i, 1);
                if i < vsize {
                    continue;
                } else {
                    break;
                }
            }
            /* Post code */
            self.fYec0_idx_save = vsize;
            /* Vectorizable loop 3 */
            /* Compute code */
            let mut i: i32 = 0;
            loop {
                iZec2[(i) as usize] = ((fZec1[(i) as usize]) as i32);
                i = i32::wrapping_add(i, 1);
                if i < vsize {
                    continue;
                } else {
                    break;
                }
            }
            /* Vectorizable loop 4 */
            /* Compute code */
            let mut i: i32 = 0;
            loop {
                fZec0[(i) as usize] = f32::min(fSlow2 * fRec0[(i) as usize], 1.0);
                i = i32::wrapping_add(i, 1);
                if i < vsize {
                    continue;
                } else {
                    break;
                }
            }
            /* Vectorizable loop 5 */
            /* Compute code */
            let mut i: i32 = 0;
            loop {
                fZec4[(i) as usize] = 1.0 - fRec0[(i) as usize];
                i = i32::wrapping_add(i, 1);
                if i < vsize {
                    continue;
                } else {
                    break;
                }
            }
            /* Vectorizable loop 6 */
            /* Compute code */
            let mut i: i32 = 0;
            loop {
                fZec6[(i) as usize] = f32::floor(fRec0[(i) as usize]);
                i = i32::wrapping_add(i, 1);
                if i < vsize {
                    continue;
                } else {
                    break;
                }
            }
            /* Vectorizable loop 7 */
            /* Compute code */
            let mut i: i32 = 0;
            loop {
                fZec3[(i) as usize] = f32::floor(fZec1[(i) as usize]);
                i = i32::wrapping_add(i, 1);
                if i < vsize {
                    continue;
                } else {
                    break;
                }
            }
            /* Vectorizable loop 8 */
            /* Compute code */
            let mut i: i32 = 0;
            loop {
                iZec5[(i) as usize] = ((fRec0[(i) as usize]) as i32);
                i = i32::wrapping_add(i, 1);
                if i < vsize {
                    continue;
                } else {
                    break;
                }
            }
            /* Vectorizable loop 9 */
            /* Compute code */
            let mut i: i32 = 0;
            loop {
                *output0 = (self.fYec0[((i32::wrapping_sub(
                    i32::wrapping_add(i, self.fYec0_idx),
                    std::cmp::min(65537, std::cmp::max(0, iZec5[(i) as usize])),
                )) & 131071) as usize]
                    * (fZec6[(i) as usize] + fZec4[(i) as usize])
                    + (fRec0[(i) as usize] - fZec6[(i) as usize])
                        * self.fYec0[((i32::wrapping_sub(
                            i32::wrapping_add(i, self.fYec0_idx),
                            std::cmp::min(
                                65537,
                                std::cmp::max(0, i32::wrapping_add(iZec5[(i) as usize], 1)),
                            ),
                        )) & 131071) as usize])
                    * fZec0[(i) as usize]
                    + (self.fYec0[((i32::wrapping_sub(
                        i32::wrapping_add(i, self.fYec0_idx),
                        std::cmp::min(65537, std::cmp::max(0, iZec2[(i) as usize])),
                    )) & 131071) as usize]
                        * (fZec3[(i) as usize] + fZec4[(i) as usize] - fSlow1)
                        + (fSlow1 + (fRec0[i as usize] - fZec3[(i) as usize]))
                            * self.fYec0[((i32::wrapping_sub(
                                i32::wrapping_add(i, self.fYec0_idx),
                                std::cmp::min(
                                    65537,
                                    std::cmp::max(0, i32::wrapping_add(iZec2[(i) as usize], 1)),
                                ),
                            )) & 131071) as usize])
                        * (1.0 - fZec0[(i) as usize]);
                i = i32::wrapping_add(i, 1);
                if i < vsize {
                    continue;
                } else {
                    break;
                }
            }
        }
    }
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("pitchshifter");
    const SIZE: usize = 1024;
    let input = [0.0; SIZE];
    let mut output = [0.0; SIZE];
    group.throughput(Throughput::Elements(SIZE as u64));

    group.bench_function("original", |b| {
        let mut dsp = mydsp::new();
        b.iter(|| {
            mydsp::compute_original(
                black_box(&mut dsp),
                black_box(SIZE as i32),
                black_box(&[&input]),
                black_box(&mut [&mut output]),
            )
        })
    });
    group.bench_function("slice", |b| {
        let mut dsp = mydsp::new();
        b.iter(|| {
            mydsp::compute_slice(
                black_box(&mut dsp),
                black_box(&input),
                black_box(&mut output),
            )
        })
    });
    group.bench_function("array", |b| {
        let mut dsp = mydsp::new();
        b.iter(|| {
            mydsp::compute_arr(
                black_box(&mut dsp),
                black_box(&input),
                black_box(&mut output),
            )
        })
    });
    group.bench_function("vec original", |b| {
        let mut dsp = mydsp_vec::new();
        b.iter(|| {
            mydsp_vec::compute_original(
                black_box(&mut dsp),
                black_box(SIZE as i32),
                black_box(&[&input]),
                black_box(&mut [&mut output]),
            )
        })
    });
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
