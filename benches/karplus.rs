//! Karplus-strong model
//! https://godbolt.org/z/PcnoroEME

use criterion::{black_box, criterion_group, criterion_main, Criterion};

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
    fn compute(&mut self, input: &[f32], output: &mut [f32]) {
        let mut fSlow0 = f32::powf(2.0, 0.083333336 * self.fHslider0);
        let mut fSlow1 = self.fHslider1;
        let mut fSlow2 = 1.0 / self.fHslider2;
        for (&input, output) in input.iter().zip(output) {
            self.fRec0[0] = (fSlow1 + self.fRec0[1] + 1.0 - fSlow0) % fSlow1;
            let mut fTemp0 = f32::min(fSlow2 * self.fRec0[0], 1.0);
            let mut fTemp1 = input;
            self.fVec0[(self.IOTA0 & 131071) as usize] = fTemp1;
            let mut fTemp2 = fSlow1 + self.fRec0[0];
            let mut iTemp3: i32 = ((fTemp2) as i32);
            let mut fTemp4 = f32::floor(fTemp2);
            let mut fTemp5 = 1.0 - self.fRec0[0];
            let mut iTemp6: i32 = (self.fRec0[0]) as i32;
            let mut fTemp7 = f32::floor(self.fRec0[0]);
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

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("pitchshifter");
    let input = [0.0; 4096];
    let mut output = [0.0; 4096];
    group.bench_function("original", |b| {
        let mut dsp = mydsp::new();
        b.iter(|| dsp.compute(black_box(&input), black_box(&mut output)))
    });
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
