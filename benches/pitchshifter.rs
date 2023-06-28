//! Pitchshifter
//! https://godbolt.org/z/bbM8jPhPf

use std::hint::black_box;
use std::ops::Rem;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

type F32 = f32;

#[allow(non_snake_case)]
#[cfg_attr(feature = "default-boxed", derive(default_boxed::DefaultBoxed))]
#[cfg_attr(feature = "reprc", repr(C))]
pub struct mydsp {
    fHslider0: F32,
    fHslider1: F32,
    fRec0: [F32; 2],
    fHslider2: F32,
    IOTA0: i32,
    fVec0: [F32; 131072],
    fSampleRate: i32,
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
            fSampleRate: 0,
        }
    }

    #[allow(non_snake_case)]
    #[inline(never)]
    fn compute(&mut self, count: i32, inputs: &[&[f32]], outputs: &mut [&mut [f32]]) {
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
        let mut fSlow0: F32 = F32::powf(2.0, 0.083333336 * self.fHslider0);
        let mut fSlow1: F32 = self.fHslider1;
        let mut fSlow2: F32 = 1.0 / self.fHslider2;
        let zipped_iterators = inputs0.zip(outputs0);
        for (input0, output0) in zipped_iterators {
            self.fRec0[0] = (fSlow1 + (self.fRec0[1] + 1.0 - fSlow0)) % fSlow1;
            let mut fTemp0: F32 = F32::min(fSlow2 * self.fRec0[0], 1.0);
            let mut fTemp1: F32 = *input0;
            self.fVec0[(self.IOTA0 & 131071) as usize] = fTemp1;
            let mut fTemp2: F32 = fSlow1 + self.fRec0[0];
            let mut iTemp3: i32 = (fTemp2) as i32;
            let mut fTemp4: F32 = F32::floor(fTemp2);
            let mut fTemp5: F32 = 1.0 - self.fRec0[0];
            let mut iTemp6: i32 = (self.fRec0[0]) as i32;
            let mut fTemp7: F32 = F32::floor(self.fRec0[0]);
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

    #[allow(non_snake_case)]
    #[inline(never)]
    fn compute_slice(&mut self, input: &[f32], output: &mut [f32]) {
        let mut fSlow0: F32 = F32::powf(2.0, 0.083333336 * self.fHslider0);
        let mut fSlow1: F32 = self.fHslider1;
        let mut fSlow2: F32 = 1.0 / self.fHslider2;
        for (input0, output0) in input.iter().zip(output) {
            self.fRec0[0] = (fSlow1 + (self.fRec0[1] + 1.0 - fSlow0)) % fSlow1;
            let mut fTemp0: F32 = F32::min(fSlow2 * self.fRec0[0], 1.0);
            let mut fTemp1: F32 = *input0;
            self.fVec0[(self.IOTA0 & 131071) as usize] = fTemp1;
            let mut fTemp2: F32 = fSlow1 + self.fRec0[0];
            let mut iTemp3: i32 = (fTemp2) as i32;
            let mut fTemp4: F32 = F32::floor(fTemp2);
            let mut fTemp5: F32 = 1.0 - self.fRec0[0];
            let mut iTemp6: i32 = (self.fRec0[0]) as i32;
            let mut fTemp7: F32 = F32::floor(self.fRec0[0]);
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

    #[allow(non_snake_case)]
    #[inline(never)]
    fn compute_arr<const N: usize>(&mut self, input: &[f32; N], output: &mut [f32; N]) {
        let mut fSlow0: F32 = F32::powf(2.0, 0.083333336 * self.fHslider0);
        let mut fSlow1: F32 = self.fHslider1;
        let mut fSlow2: F32 = 1.0 / self.fHslider2;
        for (input0, output0) in input.iter().zip(output) {
            self.fRec0[0] = (fSlow1 + (self.fRec0[1] + 1.0 - fSlow0)) % fSlow1;
            let mut fTemp0: F32 = F32::min(fSlow2 * self.fRec0[0], 1.0);
            let mut fTemp1: F32 = *input0;
            self.fVec0[(self.IOTA0 & 131071) as usize] = fTemp1;
            let mut fTemp2: F32 = fSlow1 + self.fRec0[0];
            let mut iTemp3: i32 = (fTemp2) as i32;
            let mut fTemp4: F32 = F32::floor(fTemp2);
            let mut fTemp5: F32 = 1.0 - self.fRec0[0];
            let mut iTemp6: i32 = (self.fRec0[0]) as i32;
            let mut fTemp7: F32 = F32::floor(self.fRec0[0]);
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
}

#[allow(non_snake_case)]
#[cfg_attr(feature = "reprc", repr(C))]
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

    #[allow(non_snake_case)]
    #[inline(never)]
    fn compute_old(&mut self, count: i32, inputs: &[&[f32]], outputs: &mut [&mut [f32]]) {
        let mut input0_ptr: &[F32] = inputs[0];
        let mut output0_ptr: &mut [F32] = outputs[0];
        let mut fSlow0: F32 = F32::powf(2.0, 0.083333336 * self.fHslider0);
        let mut fSlow1: F32 = self.fHslider1;
        let mut fRec0_tmp: [F32; 8] = [0.0; 8];
        let mut fSlow2: F32 = 1.0 / self.fHslider2;
        let mut fZec0: [F32; 4] = [0.0; 4];
        let mut fZec1: [F32; 4] = [0.0; 4];
        let mut iZec2: [i32; 4] = [0; 4];
        let mut fZec3: [F32; 4] = [0.0; 4];
        let mut fZec4: [F32; 4] = [0.0; 4];
        let mut iZec5: [i32; 4] = [0; 4];
        let mut fZec6: [F32; 4] = [0.0; 4];
        let mut vindex = count / 4 * 4;
        /* Main loop */
        for vindex in (0..=i32::wrapping_sub(count, 4)).step_by(4) {
            let mut vsize: i32 = 4;
            /* Recursive loop 0 */
            /* Pre code */
            for j0 in 0..4 {
                fRec0_tmp[j0 as usize] = self.fRec0_perm[j0 as usize];
            }
            /* Compute code */
            for i in 0..vsize {
                fRec0_tmp[(i32::wrapping_add(4, i)) as usize] = (fSlow1
                    + (fRec0_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + 1.0
                        - fSlow0))
                    % fSlow1;
            }
            /* Post code */
            for j1 in 0..4 {
                self.fRec0_perm[j1 as usize] = fRec0_tmp[(i32::wrapping_add(vsize, j1)) as usize];
            }
            /* Vectorizable loop 1 */
            /* Compute code */
            for i in 0..vsize {
                fZec1[i as usize] = fSlow1 + fRec0_tmp[(i32::wrapping_add(4, i)) as usize];
            }
            /* Vectorizable loop 2 */
            /* Compute code */
            for i in 0..vsize {
                fZec0[i as usize] =
                    F32::min(fSlow2 * fRec0_tmp[(i32::wrapping_add(4, i)) as usize], 1.0);
            }
            /* Vectorizable loop 3 */
            /* Compute code */
            for i in 0..vsize {
                iZec2[i as usize] = ((fZec1[i as usize]) as i32);
            }
            /* Vectorizable loop 4 */
            /* Pre code */
            self.fYec0_idx = (i32::wrapping_add(self.fYec0_idx, self.fYec0_idx_save)) & 131071;
            /* Compute code */
            for i in 0..vsize {
                self.fYec0[((i32::wrapping_add(i, self.fYec0_idx)) & 131071) as usize] =
                    input0_ptr[(i32::wrapping_add(vindex, i)) as usize];
            }
            /* Post code */
            self.fYec0_idx_save = vsize;
            /* Vectorizable loop 5 */
            /* Compute code */
            for i in 0..vsize {
                fZec3[i as usize] = F32::floor(fZec1[i as usize]);
            }
            /* Vectorizable loop 6 */
            /* Compute code */
            for i in 0..vsize {
                iZec5[i as usize] = ((fRec0_tmp[(i32::wrapping_add(4, i)) as usize]) as i32);
            }
            /* Vectorizable loop 7 */
            /* Compute code */
            for i in 0..vsize {
                fZec6[i as usize] = F32::floor(fRec0_tmp[(i32::wrapping_add(4, i)) as usize]);
            }
            /* Vectorizable loop 8 */
            /* Compute code */
            for i in 0..vsize {
                fZec4[i as usize] = 1.0 - fRec0_tmp[(i32::wrapping_add(4, i)) as usize];
            }
            /* Vectorizable loop 9 */
            /* Compute code */
            for i in 0..vsize {
                output0_ptr[(i32::wrapping_add(vindex, i)) as usize] = (self.fYec0
                    [((i32::wrapping_sub(
                        i32::wrapping_add(i, self.fYec0_idx),
                        std::cmp::min(65537, std::cmp::max(0, iZec5[i as usize])),
                    )) & 131071) as usize]
                    * (fZec6[i as usize] + fZec4[i as usize])
                    + (fRec0_tmp[(i32::wrapping_add(4, i)) as usize] - fZec6[i as usize])
                        * self.fYec0[((i32::wrapping_sub(
                            i32::wrapping_add(i, self.fYec0_idx),
                            std::cmp::min(
                                65537,
                                std::cmp::max(0, i32::wrapping_add(iZec5[i as usize], 1)),
                            ),
                        )) & 131071) as usize])
                    * fZec0[i as usize]
                    + (self.fYec0[((i32::wrapping_sub(
                        i32::wrapping_add(i, self.fYec0_idx),
                        std::cmp::min(65537, std::cmp::max(0, iZec2[i as usize])),
                    )) & 131071) as usize]
                        * (fZec3[i as usize] + fZec4[i as usize] - fSlow1)
                        + (fSlow1
                            + (fRec0_tmp[(i32::wrapping_add(4, i)) as usize] - fZec3[i as usize]))
                            * self.fYec0[((i32::wrapping_sub(
                                i32::wrapping_add(i, self.fYec0_idx),
                                std::cmp::min(
                                    65537,
                                    std::cmp::max(0, i32::wrapping_add(iZec2[i as usize], 1)),
                                ),
                            )) & 131071) as usize])
                        * (1.0 - fZec0[i as usize]);
            }
        }
        /* Remaining frames */
        if vindex < count {
            let mut vsize: i32 = i32::wrapping_sub(count, vindex);
            /* Recursive loop 0 */
            /* Pre code */
            for j0 in 0..4 {
                fRec0_tmp[j0 as usize] = self.fRec0_perm[j0 as usize];
            }
            /* Compute code */
            for i in 0..vsize {
                fRec0_tmp[(i32::wrapping_add(4, i)) as usize] = (fSlow1
                    + (fRec0_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + 1.0
                        - fSlow0))
                    % fSlow1;
            }
            /* Post code */
            for j1 in 0..4 {
                self.fRec0_perm[j1 as usize] = fRec0_tmp[(i32::wrapping_add(vsize, j1)) as usize];
            }
            /* Vectorizable loop 1 */
            /* Compute code */
            for i in 0..vsize {
                fZec1[i as usize] = fSlow1 + fRec0_tmp[(i32::wrapping_add(4, i)) as usize];
            }
            /* Vectorizable loop 2 */
            /* Compute code */
            for i in 0..vsize {
                fZec0[i as usize] =
                    F32::min(fSlow2 * fRec0_tmp[(i32::wrapping_add(4, i)) as usize], 1.0);
            }
            /* Vectorizable loop 3 */
            /* Compute code */
            for i in 0..vsize {
                iZec2[i as usize] = ((fZec1[i as usize]) as i32);
            }
            /* Vectorizable loop 4 */
            /* Pre code */
            self.fYec0_idx = (i32::wrapping_add(self.fYec0_idx, self.fYec0_idx_save)) & 131071;
            /* Compute code */
            for i in 0..vsize {
                self.fYec0[((i32::wrapping_add(i, self.fYec0_idx)) & 131071) as usize] =
                    input0_ptr[(i32::wrapping_add(vindex, i)) as usize];
            }
            /* Post code */
            self.fYec0_idx_save = vsize;
            /* Vectorizable loop 5 */
            /* Compute code */
            for i in 0..vsize {
                fZec3[i as usize] = F32::floor(fZec1[i as usize]);
            }
            /* Vectorizable loop 6 */
            /* Compute code */
            for i in 0..vsize {
                iZec5[i as usize] = ((fRec0_tmp[(i32::wrapping_add(4, i)) as usize]) as i32);
            }
            /* Vectorizable loop 7 */
            /* Compute code */
            for i in 0..vsize {
                fZec6[i as usize] = F32::floor(fRec0_tmp[(i32::wrapping_add(4, i)) as usize]);
            }
            /* Vectorizable loop 8 */
            /* Compute code */
            for i in 0..vsize {
                fZec4[i as usize] = 1.0 - fRec0_tmp[(i32::wrapping_add(4, i)) as usize];
            }
            /* Vectorizable loop 9 */
            /* Compute code */
            for i in 0..vsize {
                output0_ptr[(i32::wrapping_add(vindex, i)) as usize] = (self.fYec0
                    [((i32::wrapping_sub(
                        i32::wrapping_add(i, self.fYec0_idx),
                        std::cmp::min(65537, std::cmp::max(0, iZec5[i as usize])),
                    )) & 131071) as usize]
                    * (fZec6[i as usize] + fZec4[i as usize])
                    + (fRec0_tmp[(i32::wrapping_add(4, i)) as usize] - fZec6[i as usize])
                        * self.fYec0[((i32::wrapping_sub(
                            i32::wrapping_add(i, self.fYec0_idx),
                            std::cmp::min(
                                65537,
                                std::cmp::max(0, i32::wrapping_add(iZec5[i as usize], 1)),
                            ),
                        )) & 131071) as usize])
                    * fZec0[i as usize]
                    + (self.fYec0[((i32::wrapping_sub(
                        i32::wrapping_add(i, self.fYec0_idx),
                        std::cmp::min(65537, std::cmp::max(0, iZec2[i as usize])),
                    )) & 131071) as usize]
                        * (fZec3[i as usize] + fZec4[i as usize] - fSlow1)
                        + (fSlow1
                            + (fRec0_tmp[(i32::wrapping_add(4, i)) as usize] - fZec3[i as usize]))
                            * self.fYec0[((i32::wrapping_sub(
                                i32::wrapping_add(i, self.fYec0_idx),
                                std::cmp::min(
                                    65537,
                                    std::cmp::max(0, i32::wrapping_add(iZec2[i as usize], 1)),
                                ),
                            )) & 131071) as usize])
                        * (1.0 - fZec0[i as usize]);
            }
        }
    }

    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    fn compute_4(&mut self, count: i32, inputs: &[&[F32]], outputs: &mut [&mut [F32]]) {
        let mut fSlow0: F32 = F32::powf(2.0, 0.083333336 * self.fHslider0);
        let mut fSlow1: F32 = self.fHslider1;
        let mut fRec0_tmp: [F32; 8] = [0.0; 8];
        let mut fSlow2: F32 = 1.0 / self.fHslider2;
        let mut fZec0: [F32; 4] = [0.0; 4];
        let mut fZec1: [F32; 4] = [0.0; 4];
        let mut iZec2: [i32; 4] = [0; 4];
        let mut fZec3: [F32; 4] = [0.0; 4];
        let mut fZec4: [F32; 4] = [0.0; 4];
        let mut iZec5: [i32; 4] = [0; 4];
        let mut fZec6: [F32; 4] = [0.0; 4];
        const vsize: i32 = 4;
        /* Main loop */
        for (input0, output0) in inputs[0]
            .chunks(vsize as usize)
            .zip(outputs[0].chunks_mut(vsize as usize))
        {
            /* Recursive loop 0 */
            /* Pre code */
            for j0 in 0..4 {
                fRec0_tmp[j0 as usize] = self.fRec0_perm[j0 as usize];
            }
            /* Compute code */
            for i in 0..vsize {
                fRec0_tmp[(i32::wrapping_add(4, i)) as usize] = (fSlow1
                    + (fRec0_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + 1.0
                        - fSlow0))
                    % fSlow1;
            }
            /* Post code */
            for j1 in 0..4 {
                self.fRec0_perm[j1 as usize] = fRec0_tmp[(i32::wrapping_add(vsize, j1)) as usize];
            }
            /* Vectorizable loop 1 */
            /* Compute code */
            for i in 0..vsize {
                fZec1[i as usize] = fSlow1 + fRec0_tmp[(i32::wrapping_add(4, i)) as usize];
            }
            /* Vectorizable loop 2 */
            /* Compute code */
            for i in 0..vsize {
                fZec0[i as usize] =
                    F32::min(fSlow2 * fRec0_tmp[(i32::wrapping_add(4, i)) as usize], 1.0);
            }
            /* Vectorizable loop 3 */
            /* Pre code */
            self.fYec0_idx = (i32::wrapping_add(self.fYec0_idx, self.fYec0_idx_save)) & 131071;
            /* Compute code */
            for i in 0..vsize {
                self.fYec0[((i32::wrapping_add(i, self.fYec0_idx)) & 131071) as usize] =
                    input0[i as usize];
            }
            /* Post code */
            self.fYec0_idx_save = vsize;
            /* Vectorizable loop 4 */
            /* Compute code */
            for i in 0..vsize {
                fZec3[i as usize] = F32::floor(fZec1[i as usize]);
            }
            /* Vectorizable loop 5 */
            /* Compute code */
            for i in 0..vsize {
                fZec4[i as usize] = 1.0 - fRec0_tmp[(i32::wrapping_add(4, i)) as usize];
            }
            /* Vectorizable loop 6 */
            /* Compute code */
            for i in 0..vsize {
                iZec2[i as usize] = (fZec1[i as usize]) as i32;
            }
            /* Vectorizable loop 7 */
            /* Compute code */
            for i in 0..vsize {
                iZec5[i as usize] = (fRec0_tmp[(i32::wrapping_add(4, i)) as usize]) as i32;
            }
            /* Vectorizable loop 8 */
            /* Compute code */
            for i in 0..vsize {
                fZec6[i as usize] = F32::floor(fRec0_tmp[(i32::wrapping_add(4, i)) as usize]);
            }
            /* Vectorizable loop 9 */
            /* Compute code */
            for i in 0..vsize {
                output0[i as usize] = (self.fYec0[((i32::wrapping_sub(
                    i32::wrapping_add(i, self.fYec0_idx),
                    std::cmp::min(65537, std::cmp::max(0, iZec5[i as usize])),
                )) & 131071) as usize]
                    * (fZec6[i as usize] + fZec4[i as usize])
                    + (fRec0_tmp[(i32::wrapping_add(4, i)) as usize] - fZec6[i as usize])
                        * self.fYec0[((i32::wrapping_sub(
                            i32::wrapping_add(i, self.fYec0_idx),
                            std::cmp::min(
                                65537,
                                std::cmp::max(0, i32::wrapping_add(iZec5[i as usize], 1)),
                            ),
                        )) & 131071) as usize])
                    * fZec0[i as usize]
                    + (self.fYec0[((i32::wrapping_sub(
                        i32::wrapping_add(i, self.fYec0_idx),
                        std::cmp::min(65537, std::cmp::max(0, iZec2[i as usize])),
                    )) & 131071) as usize]
                        * (fZec3[i as usize] + fZec4[i as usize] - fSlow1)
                        + (fSlow1
                            + (fRec0_tmp[(i32::wrapping_add(4, i)) as usize] - fZec3[i as usize]))
                            * self.fYec0[((i32::wrapping_sub(
                                i32::wrapping_add(i, self.fYec0_idx),
                                std::cmp::min(
                                    65537,
                                    std::cmp::max(0, i32::wrapping_add(iZec2[i as usize], 1)),
                                ),
                            )) & 131071) as usize])
                        * (1.0 - fZec0[i as usize]);
            }
        }
    }

    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    fn compute_8(&mut self, count: i32, inputs: &[&[F32]], outputs: &mut [&mut [F32]]) {
        let mut fSlow0: F32 = F32::powf(2.0, 0.083333336 * self.fHslider0);
        let mut fSlow1: F32 = self.fHslider1;
        let mut fRec0_tmp: [F32; 12] = [0.0; 12];
        let mut fSlow2: F32 = 1.0 / self.fHslider2;
        let mut fZec0: [F32; 8] = [0.0; 8];
        let mut fZec1: [F32; 8] = [0.0; 8];
        let mut iZec2: [i32; 8] = [0; 8];
        let mut fZec3: [F32; 8] = [0.0; 8];
        let mut fZec4: [F32; 8] = [0.0; 8];
        let mut iZec5: [i32; 8] = [0; 8];
        let mut fZec6: [F32; 8] = [0.0; 8];
        const vsize: i32 = 8;
        /* Main loop */
        for (input0, output0) in inputs[0]
            .chunks(vsize as usize)
            .zip(outputs[0].chunks_mut(vsize as usize))
        {
            /* Recursive loop 0 */
            /* Pre code */
            for j0 in 0..4 {
                fRec0_tmp[j0 as usize] = self.fRec0_perm[j0 as usize];
            }
            /* Compute code */
            for i in 0..vsize {
                fRec0_tmp[(i32::wrapping_add(4, i)) as usize] = (fSlow1
                    + (fRec0_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + 1.0
                        - fSlow0))
                    % fSlow1;
            }
            /* Post code */
            for j1 in 0..4 {
                self.fRec0_perm[j1 as usize] = fRec0_tmp[(i32::wrapping_add(vsize, j1)) as usize];
            }
            /* Vectorizable loop 1 */
            /* Compute code */
            for i in 0..vsize {
                fZec1[i as usize] = fSlow1 + fRec0_tmp[(i32::wrapping_add(4, i)) as usize];
            }
            /* Vectorizable loop 2 */
            /* Compute code */
            for i in 0..vsize {
                iZec2[i as usize] = (fZec1[i as usize]) as i32;
            }
            /* Vectorizable loop 3 */
            /* Compute code */
            for i in 0..vsize {
                fZec0[i as usize] =
                    F32::min(fSlow2 * fRec0_tmp[(i32::wrapping_add(4, i)) as usize], 1.0);
            }
            /* Vectorizable loop 4 */
            /* Pre code */
            self.fYec0_idx = (i32::wrapping_add(self.fYec0_idx, self.fYec0_idx_save)) & 131071;
            /* Compute code */
            for i in 0..vsize {
                self.fYec0[((i32::wrapping_add(i, self.fYec0_idx)) & 131071) as usize] =
                    input0[i as usize];
            }
            /* Post code */
            self.fYec0_idx_save = vsize;
            /* Vectorizable loop 5 */
            /* Compute code */
            for i in 0..vsize {
                fZec6[i as usize] = F32::floor(fRec0_tmp[(i32::wrapping_add(4, i)) as usize]);
            }
            /* Vectorizable loop 6 */
            /* Compute code */
            for i in 0..vsize {
                iZec5[i as usize] = (fRec0_tmp[(i32::wrapping_add(4, i)) as usize]) as i32;
            }
            /* Vectorizable loop 7 */
            /* Compute code */
            for i in 0..vsize {
                fZec4[i as usize] = 1.0 - fRec0_tmp[(i32::wrapping_add(4, i)) as usize];
            }
            /* Vectorizable loop 8 */
            /* Compute code */
            for i in 0..vsize {
                fZec3[i as usize] = F32::floor(fZec1[i as usize]);
            }
            /* Vectorizable loop 9 */
            /* Compute code */
            for i in 0..vsize {
                output0[i as usize] = (self.fYec0[((i32::wrapping_sub(
                    i32::wrapping_add(i, self.fYec0_idx),
                    std::cmp::min(65537, std::cmp::max(0, iZec5[i as usize])),
                )) & 131071) as usize]
                    * (fZec6[i as usize] + fZec4[i as usize])
                    + (fRec0_tmp[(i32::wrapping_add(4, i)) as usize] - fZec6[i as usize])
                        * self.fYec0[((i32::wrapping_sub(
                            i32::wrapping_add(i, self.fYec0_idx),
                            std::cmp::min(
                                65537,
                                std::cmp::max(0, i32::wrapping_add(iZec5[i as usize], 1)),
                            ),
                        )) & 131071) as usize])
                    * fZec0[i as usize]
                    + (self.fYec0[((i32::wrapping_sub(
                        i32::wrapping_add(i, self.fYec0_idx),
                        std::cmp::min(65537, std::cmp::max(0, iZec2[i as usize])),
                    )) & 131071) as usize]
                        * (fZec3[i as usize] + fZec4[i as usize] - fSlow1)
                        + (fSlow1
                            + (fRec0_tmp[(i32::wrapping_add(4, i)) as usize] - fZec3[i as usize]))
                            * self.fYec0[((i32::wrapping_sub(
                                i32::wrapping_add(i, self.fYec0_idx),
                                std::cmp::min(
                                    65537,
                                    std::cmp::max(0, i32::wrapping_add(iZec2[i as usize], 1)),
                                ),
                            )) & 131071) as usize])
                        * (1.0 - fZec0[i as usize]);
            }
        }
    }

    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    fn compute_16(&mut self, count: i32, inputs: &[&[F32]], outputs: &mut [&mut [F32]]) {
        let mut fSlow0: F32 = F32::powf(2.0, 0.083333336 * self.fHslider0);
        let mut fSlow1: F32 = self.fHslider1;
        let mut fRec0_tmp: [F32; 20] = [0.0; 20];
        let mut fSlow2: F32 = 1.0 / self.fHslider2;
        let mut fZec0: [F32; 16] = [0.0; 16];
        let mut fZec1: [F32; 16] = [0.0; 16];
        let mut iZec2: [i32; 16] = [0; 16];
        let mut fZec3: [F32; 16] = [0.0; 16];
        let mut fZec4: [F32; 16] = [0.0; 16];
        let mut iZec5: [i32; 16] = [0; 16];
        let mut fZec6: [F32; 16] = [0.0; 16];
        const vsize: i32 = 16;
        /* Main loop */
        for (input0, output0) in inputs[0]
            .chunks(vsize as usize)
            .zip(outputs[0].chunks_mut(vsize as usize))
        {
            /* Recursive loop 0 */
            /* Pre code */
            for j0 in 0..4 {
                fRec0_tmp[j0 as usize] = self.fRec0_perm[j0 as usize];
            }
            /* Compute code */
            for i in 0..vsize {
                fRec0_tmp[(i32::wrapping_add(4, i)) as usize] = (fSlow1
                    + (fRec0_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + 1.0
                        - fSlow0))
                    % fSlow1;
            }
            /* Post code */
            for j1 in 0..4 {
                self.fRec0_perm[j1 as usize] = fRec0_tmp[(i32::wrapping_add(vsize, j1)) as usize];
            }
            /* Vectorizable loop 1 */
            /* Compute code */
            for i in 0..vsize {
                fZec1[i as usize] = fSlow1 + fRec0_tmp[(i32::wrapping_add(4, i)) as usize];
            }
            /* Vectorizable loop 2 */
            /* Pre code */
            self.fYec0_idx = (i32::wrapping_add(self.fYec0_idx, self.fYec0_idx_save)) & 131071;
            /* Compute code */
            for i in 0..vsize {
                self.fYec0[((i32::wrapping_add(i, self.fYec0_idx)) & 131071) as usize] =
                    input0[i as usize];
            }
            /* Post code */
            self.fYec0_idx_save = vsize;
            /* Vectorizable loop 3 */
            /* Compute code */
            for i in 0..vsize {
                fZec0[i as usize] =
                    F32::min(fSlow2 * fRec0_tmp[(i32::wrapping_add(4, i)) as usize], 1.0);
            }
            /* Vectorizable loop 4 */
            /* Compute code */
            for i in 0..vsize {
                iZec2[i as usize] = (fZec1[i as usize]) as i32;
            }
            /* Vectorizable loop 5 */
            /* Compute code */
            for i in 0..vsize {
                fZec4[i as usize] = 1.0 - fRec0_tmp[(i32::wrapping_add(4, i)) as usize];
            }
            /* Vectorizable loop 6 */
            /* Compute code */
            for i in 0..vsize {
                fZec3[i as usize] = F32::floor(fZec1[i as usize]);
            }
            /* Vectorizable loop 7 */
            /* Compute code */
            for i in 0..vsize {
                iZec5[i as usize] = (fRec0_tmp[(i32::wrapping_add(4, i)) as usize]) as i32;
            }
            /* Vectorizable loop 8 */
            /* Compute code */
            for i in 0..vsize {
                fZec6[i as usize] = F32::floor(fRec0_tmp[(i32::wrapping_add(4, i)) as usize]);
            }
            /* Vectorizable loop 9 */
            /* Compute code */
            for i in 0..vsize {
                output0[i as usize] = (self.fYec0[((i32::wrapping_sub(
                    i32::wrapping_add(i, self.fYec0_idx),
                    std::cmp::min(65537, std::cmp::max(0, iZec5[i as usize])),
                )) & 131071) as usize]
                    * (fZec6[i as usize] + fZec4[i as usize])
                    + (fRec0_tmp[(i32::wrapping_add(4, i)) as usize] - fZec6[i as usize])
                        * self.fYec0[((i32::wrapping_sub(
                            i32::wrapping_add(i, self.fYec0_idx),
                            std::cmp::min(
                                65537,
                                std::cmp::max(0, i32::wrapping_add(iZec5[i as usize], 1)),
                            ),
                        )) & 131071) as usize])
                    * fZec0[i as usize]
                    + (self.fYec0[((i32::wrapping_sub(
                        i32::wrapping_add(i, self.fYec0_idx),
                        std::cmp::min(65537, std::cmp::max(0, iZec2[i as usize])),
                    )) & 131071) as usize]
                        * (fZec3[i as usize] + fZec4[i as usize] - fSlow1)
                        + (fSlow1
                            + (fRec0_tmp[(i32::wrapping_add(4, i)) as usize] - fZec3[i as usize]))
                            * self.fYec0[((i32::wrapping_sub(
                                i32::wrapping_add(i, self.fYec0_idx),
                                std::cmp::min(
                                    65537,
                                    std::cmp::max(0, i32::wrapping_add(iZec2[i as usize], 1)),
                                ),
                            )) & 131071) as usize])
                        * (1.0 - fZec0[i as usize]);
            }
        }
    }

    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    fn compute_32(&mut self, count: i32, inputs: &[&[F32]], outputs: &mut [&mut [F32]]) {
        let mut fSlow0: F32 = F32::powf(2.0, 0.083333336 * self.fHslider0);
        let mut fSlow1: F32 = self.fHslider1;
        let mut fRec0_tmp: [F32; 36] = [0.0; 36];
        let mut fSlow2: F32 = 1.0 / self.fHslider2;
        let mut fZec0: [F32; 32] = [0.0; 32];
        let mut fZec1: [F32; 32] = [0.0; 32];
        let mut iZec2: [i32; 32] = [0; 32];
        let mut fZec3: [F32; 32] = [0.0; 32];
        let mut fZec4: [F32; 32] = [0.0; 32];
        let mut iZec5: [i32; 32] = [0; 32];
        let mut fZec6: [F32; 32] = [0.0; 32];
        const vsize: i32 = 32;
        /* Main loop */
        for (input0, output0) in inputs[0]
            .chunks(vsize as usize)
            .zip(outputs[0].chunks_mut(vsize as usize))
        {
            /* Recursive loop 0 */
            /* Pre code */
            for j0 in 0..4 {
                fRec0_tmp[j0 as usize] = self.fRec0_perm[j0 as usize];
            }
            /* Compute code */
            for i in 0..vsize {
                fRec0_tmp[(i32::wrapping_add(4, i)) as usize] = (fSlow1
                    + (fRec0_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize] + 1.0
                        - fSlow0))
                    % fSlow1;
            }
            /* Post code */
            for j1 in 0..4 {
                self.fRec0_perm[j1 as usize] = fRec0_tmp[(i32::wrapping_add(vsize, j1)) as usize];
            }
            /* Vectorizable loop 1 */
            /* Compute code */
            for i in 0..vsize {
                fZec1[i as usize] = fSlow1 + fRec0_tmp[(i32::wrapping_add(4, i)) as usize];
            }
            /* Vectorizable loop 2 */
            /* Compute code */
            for i in 0..vsize {
                fZec0[i as usize] =
                    F32::min(fSlow2 * fRec0_tmp[(i32::wrapping_add(4, i)) as usize], 1.0);
            }
            /* Vectorizable loop 3 */
            /* Pre code */
            self.fYec0_idx = (i32::wrapping_add(self.fYec0_idx, self.fYec0_idx_save)) & 131071;
            /* Compute code */
            for i in 0..vsize {
                self.fYec0[((i32::wrapping_add(i, self.fYec0_idx)) & 131071) as usize] =
                    input0[i as usize];
            }
            /* Post code */
            self.fYec0_idx_save = vsize;
            /* Vectorizable loop 4 */
            /* Compute code */
            for i in 0..vsize {
                iZec2[i as usize] = (fZec1[i as usize]) as i32;
            }
            /* Vectorizable loop 5 */
            /* Compute code */
            for i in 0..vsize {
                fZec3[i as usize] = F32::floor(fZec1[i as usize]);
            }
            /* Vectorizable loop 6 */
            /* Compute code */
            for i in 0..vsize {
                iZec5[i as usize] = (fRec0_tmp[(i32::wrapping_add(4, i)) as usize]) as i32;
            }
            /* Vectorizable loop 7 */
            /* Compute code */
            for i in 0..vsize {
                fZec4[i as usize] = 1.0 - fRec0_tmp[(i32::wrapping_add(4, i)) as usize];
            }
            /* Vectorizable loop 8 */
            /* Compute code */
            for i in 0..vsize {
                fZec6[i as usize] = F32::floor(fRec0_tmp[(i32::wrapping_add(4, i)) as usize]);
            }
            /* Vectorizable loop 9 */
            /* Compute code */
            for i in 0..vsize {
                output0[i as usize] = (self.fYec0[((i32::wrapping_sub(
                    i32::wrapping_add(i, self.fYec0_idx),
                    std::cmp::min(65537, std::cmp::max(0, iZec5[i as usize])),
                )) & 131071) as usize]
                    * (fZec6[i as usize] + fZec4[i as usize])
                    + (fRec0_tmp[(i32::wrapping_add(4, i)) as usize] - fZec6[i as usize])
                        * self.fYec0[((i32::wrapping_sub(
                            i32::wrapping_add(i, self.fYec0_idx),
                            std::cmp::min(
                                65537,
                                std::cmp::max(0, i32::wrapping_add(iZec5[i as usize], 1)),
                            ),
                        )) & 131071) as usize])
                    * fZec0[i as usize]
                    + (self.fYec0[((i32::wrapping_sub(
                        i32::wrapping_add(i, self.fYec0_idx),
                        std::cmp::min(65537, std::cmp::max(0, iZec2[i as usize])),
                    )) & 131071) as usize]
                        * (fZec3[i as usize] + fZec4[i as usize] - fSlow1)
                        + (fSlow1
                            + (fRec0_tmp[(i32::wrapping_add(4, i)) as usize] - fZec3[i as usize]))
                            * self.fYec0[((i32::wrapping_sub(
                                i32::wrapping_add(i, self.fYec0_idx),
                                std::cmp::min(
                                    65537,
                                    std::cmp::max(0, i32::wrapping_add(iZec2[i as usize], 1)),
                                ),
                            )) & 131071) as usize])
                        * (1.0 - fZec0[i as usize]);
            }
        }
    }

    #[allow(non_snake_case)]
    #[inline(never)]
    fn compute_manual(&mut self, count: i32, inputs: &[&[f32]], outputs: &mut [&mut [f32]]) {
        const CHUNK_SIZE: usize = 4;
        let mut fSlow0: F32 = F32::powf(2.0, 0.083333336 * self.fHslider0);
        let mut fSlow1: F32 = self.fHslider1;
        let mut fRec0_tmp: [F32; 4 + CHUNK_SIZE] = [0.0; 4 + CHUNK_SIZE];
        let mut fSlow2: F32 = 1.0 / self.fHslider2;
        let mut fZec0: [F32; CHUNK_SIZE] = [0.0; CHUNK_SIZE];
        let mut fZec1: [F32; CHUNK_SIZE] = [0.0; CHUNK_SIZE];
        let mut iZec2: [i32; CHUNK_SIZE] = [0; CHUNK_SIZE];
        let mut fZec3: [F32; CHUNK_SIZE] = [0.0; CHUNK_SIZE];
        let mut fZec4: [F32; CHUNK_SIZE] = [0.0; CHUNK_SIZE];
        let mut iZec5: [i32; CHUNK_SIZE] = [0; CHUNK_SIZE];
        let mut fZec6: [F32; CHUNK_SIZE] = [0.0; CHUNK_SIZE];

        /* Main loop */
        for (input, output) in inputs[0]
            .chunks(CHUNK_SIZE)
            .zip(outputs[0].chunks_mut(CHUNK_SIZE))
        {
            /* Recursive loop 0 */
            /* Pre code */
            for j0 in 0..4 {
                fRec0_tmp[j0 as usize] = self.fRec0_perm[j0 as usize];
            }
            /* Compute code */
            for i in 0..CHUNK_SIZE {
                fRec0_tmp[(usize::wrapping_add(4, i)) as usize] = (fSlow1
                    + (fRec0_tmp[(usize::wrapping_add(4, usize::wrapping_sub(i, 1))) as usize]
                        + 1.0
                        - fSlow0))
                    % fSlow1;
            }
            /* Post code */
            for j1 in 0..4 {
                self.fRec0_perm[j1 as usize] =
                    fRec0_tmp[(usize::wrapping_add(CHUNK_SIZE, j1)) as usize];
            }
            /* Vectorizable loop 1 */
            /* Compute code */
            for i in 0..CHUNK_SIZE {
                fZec1[i as usize] = fSlow1 + fRec0_tmp[(usize::wrapping_add(4, i)) as usize];
            }
            /* Vectorizable loop 2 */
            /* Compute code */
            for i in 0..CHUNK_SIZE {
                fZec0[i as usize] = F32::min(
                    fSlow2 * fRec0_tmp[(usize::wrapping_add(4, i)) as usize],
                    1.0,
                );
            }
            /* Vectorizable loop 3 */
            /* Compute code */
            for i in 0..CHUNK_SIZE {
                iZec2[i as usize] = ((fZec1[i as usize]) as i32);
            }
            /* Vectorizable loop 4 */
            /* Pre code */
            self.fYec0_idx = (i32::wrapping_add(self.fYec0_idx, self.fYec0_idx_save)) & 131071;
            /* Compute code */
            for (i, &input) in input.iter().enumerate() {
                self.fYec0[((usize::wrapping_add(i, self.fYec0_idx as usize)) & 131071) as usize] =
                    input;
            }
            /* Post code */
            self.fYec0_idx_save = CHUNK_SIZE as i32;
            /* Vectorizable loop 5 */
            /* Compute code */
            // for i in 0..CHUNK_SIZE {
            //     fZec3[i as usize] = F32::floor(fZec1[i as usize]);
            // }
            fZec3.copy_from_slice(&fZec1.map(|s| s.floor()));
            /* Vectorizable loop 6 */
            /* Compute code */
            // for i in 0..CHUNK_SIZE {
            //     iZec5[i as usize] = ((fRec0_tmp[(usize::wrapping_add(4, i)) as usize]) as i32);
            // }
            for (fRec0_tmp, iZec5) in fRec0_tmp[4..].iter().zip(iZec5.iter_mut()) {
                *iZec5 = *fRec0_tmp as i32;
            }
            /* Vectorizable loop 7 */
            /* Compute code */
            for i in 0..CHUNK_SIZE {
                fZec6[i as usize] = F32::floor(fRec0_tmp[(usize::wrapping_add(4, i)) as usize]);
            }
            /* Vectorizable loop 8 */
            /* Compute code */
            for i in 0..CHUNK_SIZE {
                fZec4[i as usize] = 1.0 - fRec0_tmp[(usize::wrapping_add(4, i)) as usize];
            }
            /* Vectorizable loop 9 */
            /* Compute code */
            for (i, output) in output.iter_mut().enumerate().map(|(i, o)| (i as i32, o)) {
                *output = (self.fYec0[((i32::wrapping_sub(
                    i32::wrapping_add(i, self.fYec0_idx),
                    std::cmp::min(65537, std::cmp::max(0, iZec5[i as usize])),
                )) & 131071) as usize]
                    * (fZec6[i as usize] + fZec4[i as usize])
                    + (fRec0_tmp[(i32::wrapping_add(4, i)) as usize] - fZec6[i as usize])
                        * self.fYec0[((i32::wrapping_sub(
                            i32::wrapping_add(i, self.fYec0_idx),
                            std::cmp::min(
                                65537,
                                std::cmp::max(0, i32::wrapping_add(iZec5[i as usize], 1)),
                            ),
                        )) & 131071) as usize])
                    * fZec0[i as usize]
                    + (self.fYec0[((i32::wrapping_sub(
                        i32::wrapping_add(i, self.fYec0_idx),
                        std::cmp::min(65537, std::cmp::max(0, iZec2[i as usize])),
                    )) & 131071) as usize]
                        * (fZec3[i as usize] + fZec4[i as usize] - fSlow1)
                        + (fSlow1
                            + (fRec0_tmp[(i32::wrapping_add(4, i)) as usize] - fZec3[i as usize]))
                            * self.fYec0[((i32::wrapping_sub(
                                i32::wrapping_add(i, self.fYec0_idx),
                                std::cmp::min(
                                    65537,
                                    std::cmp::max(0, i32::wrapping_add(iZec2[i as usize], 1)),
                                ),
                            )) & 131071) as usize])
                        * (1.0 - fZec0[i as usize]);
            }
        }
    }
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("pitchshifter");
    const SIZE: usize = 4095;
    let input = [0.0; SIZE];
    let mut output = [0.0; SIZE];
    group.throughput(Throughput::Elements(SIZE as u64));

    /*group.bench_function("scalar", |b| {
        let mut dsp = mydsp::new();
        b.iter(|| {
            mydsp::compute(
                black_box(&mut dsp),
                black_box(SIZE as i32),
                black_box(&[&input]),
                black_box(&mut [&mut output]),
            )
        })
    });*/
    /*group.bench_function("scalar_slice", |b| {
        let mut dsp = mydsp::new();
        b.iter(|| {
            mydsp::compute_slice(
                black_box(&mut dsp),
                black_box(&input),
                black_box(&mut output),
            )
        })
    });
    group.bench_function("scalar_array", |b| {
        let mut dsp = mydsp::new();
        b.iter(|| {
            mydsp::compute_arr(
                black_box(&mut dsp),
                black_box(&input),
                black_box(&mut output),
            )
        })
    });*/
    /*group.bench_function("vec_old", |b| {
        let mut dsp = mydsp_vec::new();
        b.iter(|| {
            mydsp_vec::compute_old(
                black_box(&mut dsp),
                black_box(SIZE as i32),
                black_box(&[&input]),
                black_box(&mut [&mut output]),
            )
        })
    });*/
    /*group.bench_function(BenchmarkId::new("vec", 4), |b| {
        let mut dsp = mydsp_vec::new();
        b.iter(|| {
            mydsp_vec::compute_4(
                black_box(&mut dsp),
                black_box(SIZE as i32),
                black_box(&[&input]),
                black_box(&mut [&mut output]),
            )
        })
    });
    group.bench_function(BenchmarkId::new("vec", 8), |b| {
        let mut dsp = mydsp_vec::new();
        b.iter(|| {
            mydsp_vec::compute_8(
                black_box(&mut dsp),
                black_box(SIZE as i32),
                black_box(&[&input]),
                black_box(&mut [&mut output]),
            )
        })
    });
    group.bench_function(BenchmarkId::new("vec", 16), |b| {
        let mut dsp = mydsp_vec::new();
        b.iter(|| {
            mydsp_vec::compute_16(
                black_box(&mut dsp),
                black_box(SIZE as i32),
                black_box(&[&input]),
                black_box(&mut [&mut output]),
            )
        })
    });
    group.bench_function(BenchmarkId::new("vec", 32), |b| {
        let mut dsp = mydsp_vec::new();
        b.iter(|| {
            mydsp_vec::compute_32(
                black_box(&mut dsp),
                black_box(SIZE as i32),
                black_box(&[&input]),
                black_box(&mut [&mut output]),
            )
        })
    });*/
    group.bench_function("vec_manual", |b| {
        let mut dsp = mydsp_vec::new();
        b.iter(|| {
            mydsp_vec::compute_manual(
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
