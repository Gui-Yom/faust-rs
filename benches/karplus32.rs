//! https://godbolt.org/z/a3Yj6rbPq

use std::hint::black_box;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

type F32 = f32;

#[allow(non_snake_case)]
#[cfg_attr(feature = "default-boxed", derive(default_boxed::DefaultBoxed))]
#[cfg_attr(feature = "reprc", repr(C))]
pub struct mydsp {
    fHslider0: F32,
    iRec1: [i32; 2],
    fHslider1: F32,
    fButton0: F32,
    fVec0: [F32; 2],
    fRec2: [F32; 2],
    fHslider2: F32,
    IOTA0: i32,
    fVec1: [F32; 8192],
    fHslider3: F32,
    fHslider4: F32,
    fRec0: [F32; 3],
    fHslider5: F32,
    fVec2: [F32; 8192],
    fRec3: [F32; 3],
    fVec3: [F32; 8192],
    fRec4: [F32; 3],
    fVec4: [F32; 8192],
    fRec5: [F32; 3],
    fVec5: [F32; 8192],
    fRec6: [F32; 3],
    fVec6: [F32; 8192],
    fRec7: [F32; 3],
    fVec7: [F32; 8192],
    fRec8: [F32; 3],
    fVec8: [F32; 8192],
    fRec9: [F32; 3],
    fVec9: [F32; 8192],
    fRec10: [F32; 3],
    fVec10: [F32; 8192],
    fRec11: [F32; 3],
    fVec11: [F32; 4096],
    fRec12: [F32; 3],
    fVec12: [F32; 2048],
    fRec13: [F32; 3],
    fVec13: [F32; 512],
    fRec14: [F32; 3],
    fVec14: [F32; 4096],
    fRec15: [F32; 3],
    fVec15: [F32; 8192],
    fRec16: [F32; 3],
    fVec16: [F32; 8192],
    fRec17: [F32; 3],
    fHslider6: F32,
    fVec17: [F32; 8192],
    fRec18: [F32; 3],
    fVec18: [F32; 8192],
    fRec19: [F32; 3],
    fVec19: [F32; 8192],
    fRec20: [F32; 3],
    fVec20: [F32; 8192],
    fRec21: [F32; 3],
    fVec21: [F32; 8192],
    fRec22: [F32; 3],
    fVec22: [F32; 8192],
    fRec23: [F32; 3],
    fVec23: [F32; 8192],
    fRec24: [F32; 3],
    fVec24: [F32; 8192],
    fRec25: [F32; 3],
    fVec25: [F32; 8192],
    fRec26: [F32; 3],
    fVec26: [F32; 4096],
    fRec27: [F32; 3],
    fVec27: [F32; 4096],
    fRec28: [F32; 3],
    fVec28: [F32; 2048],
    fRec29: [F32; 3],
    fVec29: [F32; 1024],
    fRec30: [F32; 3],
    fVec30: [F32; 8192],
    fRec31: [F32; 3],
    fVec31: [F32; 8192],
    fRec32: [F32; 3],
    fVec32: [F32; 8192],
    fRec33: [F32; 3],
    fSampleRate: i32,
}

impl mydsp {
    fn new() -> Self {
        mydsp {
            fHslider0: 0.0,
            iRec1: [0; 2],
            fHslider1: 0.0,
            fButton0: 0.0,
            fVec0: [0.0; 2],
            fRec2: [0.0; 2],
            fHslider2: 0.0,
            IOTA0: 0,
            fVec1: [0.0; 8192],
            fHslider3: 0.0,
            fHslider4: 0.0,
            fRec0: [0.0; 3],
            fHslider5: 0.0,
            fVec2: [0.0; 8192],
            fRec3: [0.0; 3],
            fVec3: [0.0; 8192],
            fRec4: [0.0; 3],
            fVec4: [0.0; 8192],
            fRec5: [0.0; 3],
            fVec5: [0.0; 8192],
            fRec6: [0.0; 3],
            fVec6: [0.0; 8192],
            fRec7: [0.0; 3],
            fVec7: [0.0; 8192],
            fRec8: [0.0; 3],
            fVec8: [0.0; 8192],
            fRec9: [0.0; 3],
            fVec9: [0.0; 8192],
            fRec10: [0.0; 3],
            fVec10: [0.0; 8192],
            fRec11: [0.0; 3],
            fVec11: [0.0; 4096],
            fRec12: [0.0; 3],
            fVec12: [0.0; 2048],
            fRec13: [0.0; 3],
            fVec13: [0.0; 512],
            fRec14: [0.0; 3],
            fVec14: [0.0; 4096],
            fRec15: [0.0; 3],
            fVec15: [0.0; 8192],
            fRec16: [0.0; 3],
            fVec16: [0.0; 8192],
            fRec17: [0.0; 3],
            fHslider6: 0.0,
            fVec17: [0.0; 8192],
            fRec18: [0.0; 3],
            fVec18: [0.0; 8192],
            fRec19: [0.0; 3],
            fVec19: [0.0; 8192],
            fRec20: [0.0; 3],
            fVec20: [0.0; 8192],
            fRec21: [0.0; 3],
            fVec21: [0.0; 8192],
            fRec22: [0.0; 3],
            fVec22: [0.0; 8192],
            fRec23: [0.0; 3],
            fVec23: [0.0; 8192],
            fRec24: [0.0; 3],
            fVec24: [0.0; 8192],
            fRec25: [0.0; 3],
            fVec25: [0.0; 8192],
            fRec26: [0.0; 3],
            fVec26: [0.0; 4096],
            fRec27: [0.0; 3],
            fVec27: [0.0; 4096],
            fRec28: [0.0; 3],
            fVec28: [0.0; 2048],
            fRec29: [0.0; 3],
            fVec29: [0.0; 1024],
            fRec30: [0.0; 3],
            fVec30: [0.0; 8192],
            fRec31: [0.0; 3],
            fVec31: [0.0; 8192],
            fRec32: [0.0; 3],
            fVec32: [0.0; 8192],
            fRec33: [0.0; 3],
            fSampleRate: 0,
        }
    }

    #[allow(non_snake_case)]
    #[inline(never)]
    fn compute(&mut self, count: i32, inputs: &[&[F32]], outputs: &mut [&mut [F32]]) {
        let (outputs0, outputs1) = if let [outputs0, outputs1, ..] = outputs {
            let outputs0 = outputs0[..count as usize].iter_mut();
            let outputs1 = outputs1[..count as usize].iter_mut();
            (outputs0, outputs1)
        } else {
            panic!("wrong number of outputs");
        };
        let mut fSlow0: F32 = 0.5 * (1.0 - self.fHslider0);
        let mut fSlow1: F32 = 1.0 / self.fHslider1;
        let mut fSlow2: F32 = self.fButton0;
        let mut fSlow3: F32 = 4.656613e-10 * self.fHslider2;
        let mut fSlow4: F32 = self.fHslider3;
        let mut fSlow5: F32 = self.fHslider4;
        let mut iSlow6: i32 =
            (F32::min(4096.0, F32::max(0.0, fSlow5 + 3e+01 * fSlow4 + -1.5))) as i32;
        let mut fSlow7: F32 = self.fHslider5;
        let mut fSlow8: F32 = ((fSlow7 > 3e+01) as i32) as u32 as F32;
        let mut iSlow9: i32 =
            (F32::min(4096.0, F32::max(0.0, fSlow5 + 28.0 * fSlow4 + -1.5))) as i32;
        let mut fSlow10: F32 = ((fSlow7 > 28.0) as i32) as u32 as F32;
        let mut iSlow11: i32 =
            (F32::min(4096.0, F32::max(0.0, fSlow5 + 26.0 * fSlow4 + -1.5))) as i32;
        let mut fSlow12: F32 = ((fSlow7 > 26.0) as i32) as u32 as F32;
        let mut iSlow13: i32 =
            (F32::min(4096.0, F32::max(0.0, fSlow5 + 22.0 * fSlow4 + -1.5))) as i32;
        let mut fSlow14: F32 = ((fSlow7 > 22.0) as i32) as u32 as F32;
        let mut iSlow15: i32 =
            (F32::min(4096.0, F32::max(0.0, fSlow5 + 2e+01 * fSlow4 + -1.5))) as i32;
        let mut fSlow16: F32 = ((fSlow7 > 2e+01) as i32) as u32 as F32;
        let mut iSlow17: i32 =
            (F32::min(4096.0, F32::max(0.0, fSlow5 + 18.0 * fSlow4 + -1.5))) as i32;
        let mut fSlow18: F32 = ((fSlow7 > 18.0) as i32) as u32 as F32;
        let mut iSlow19: i32 =
            (F32::min(4096.0, F32::max(0.0, fSlow5 + 16.0 * fSlow4 + -1.5))) as i32;
        let mut fSlow20: F32 = ((fSlow7 > 16.0) as i32) as u32 as F32;
        let mut iSlow21: i32 =
            (F32::min(4096.0, F32::max(0.0, fSlow5 + 14.0 * fSlow4 + -1.5))) as i32;
        let mut fSlow22: F32 = ((fSlow7 > 14.0) as i32) as u32 as F32;
        let mut iSlow23: i32 =
            (F32::min(4096.0, F32::max(0.0, fSlow5 + 12.0 * fSlow4 + -1.5))) as i32;
        let mut fSlow24: F32 = ((fSlow7 > 12.0) as i32) as u32 as F32;
        let mut iSlow25: i32 =
            (F32::min(4096.0, F32::max(0.0, fSlow5 + 8.0 * fSlow4 + -1.5))) as i32;
        let mut fSlow26: F32 = ((fSlow7 > 8.0) as i32) as u32 as F32;
        let mut iSlow27: i32 =
            (F32::min(4096.0, F32::max(0.0, fSlow5 + 6.0 * fSlow4 + -1.5))) as i32;
        let mut fSlow28: F32 = ((fSlow7 > 6.0) as i32) as u32 as F32;
        let mut iSlow29: i32 =
            (F32::min(4096.0, F32::max(0.0, fSlow5 + 2.0 * fSlow4 + -1.5))) as i32;
        let mut fSlow30: F32 = ((fSlow7 > 2.0) as i32) as u32 as F32;
        let mut iSlow31: i32 = (F32::min(4096.0, F32::max(0.0, fSlow5 + -1.5))) as i32;
        let mut fSlow32: F32 = ((fSlow7 > 0.0) as i32) as u32 as F32;
        let mut iSlow33: i32 =
            (F32::min(4096.0, F32::max(0.0, fSlow5 + 4.0 * fSlow4 + -1.5))) as i32;
        let mut fSlow34: F32 = ((fSlow7 > 4.0) as i32) as u32 as F32;
        let mut iSlow35: i32 =
            (F32::min(4096.0, F32::max(0.0, fSlow5 + 1e+01 * fSlow4 + -1.5))) as i32;
        let mut fSlow36: F32 = ((fSlow7 > 1e+01) as i32) as u32 as F32;
        let mut iSlow37: i32 =
            (F32::min(4096.0, F32::max(0.0, fSlow5 + 24.0 * fSlow4 + -1.5))) as i32;
        let mut fSlow38: F32 = ((fSlow7 > 24.0) as i32) as u32 as F32;
        let mut fSlow39: F32 = self.fHslider6;
        let mut iSlow40: i32 =
            (F32::min(4096.0, F32::max(0.0, fSlow5 + 31.0 * fSlow4 + -1.5))) as i32;
        let mut fSlow41: F32 = ((fSlow7 > 31.0) as i32) as u32 as F32;
        let mut iSlow42: i32 =
            (F32::min(4096.0, F32::max(0.0, fSlow5 + 29.0 * fSlow4 + -1.5))) as i32;
        let mut fSlow43: F32 = ((fSlow7 > 29.0) as i32) as u32 as F32;
        let mut iSlow44: i32 =
            (F32::min(4096.0, F32::max(0.0, fSlow5 + 27.0 * fSlow4 + -1.5))) as i32;
        let mut fSlow45: F32 = ((fSlow7 > 27.0) as i32) as u32 as F32;
        let mut iSlow46: i32 =
            (F32::min(4096.0, F32::max(0.0, fSlow5 + 25.0 * fSlow4 + -1.5))) as i32;
        let mut fSlow47: F32 = ((fSlow7 > 25.0) as i32) as u32 as F32;
        let mut iSlow48: i32 =
            (F32::min(4096.0, F32::max(0.0, fSlow5 + 23.0 * fSlow4 + -1.5))) as i32;
        let mut fSlow49: F32 = ((fSlow7 > 23.0) as i32) as u32 as F32;
        let mut iSlow50: i32 =
            (F32::min(4096.0, F32::max(0.0, fSlow5 + 19.0 * fSlow4 + -1.5))) as i32;
        let mut fSlow51: F32 = ((fSlow7 > 19.0) as i32) as u32 as F32;
        let mut iSlow52: i32 =
            (F32::min(4096.0, F32::max(0.0, fSlow5 + 17.0 * fSlow4 + -1.5))) as i32;
        let mut fSlow53: F32 = ((fSlow7 > 17.0) as i32) as u32 as F32;
        let mut iSlow54: i32 =
            (F32::min(4096.0, F32::max(0.0, fSlow5 + 15.0 * fSlow4 + -1.5))) as i32;
        let mut fSlow55: F32 = ((fSlow7 > 15.0) as i32) as u32 as F32;
        let mut iSlow56: i32 =
            (F32::min(4096.0, F32::max(0.0, fSlow5 + 11.0 * fSlow4 + -1.5))) as i32;
        let mut fSlow57: F32 = ((fSlow7 > 11.0) as i32) as u32 as F32;
        let mut iSlow58: i32 =
            (F32::min(4096.0, F32::max(0.0, fSlow5 + 7.0 * fSlow4 + -1.5))) as i32;
        let mut fSlow59: F32 = ((fSlow7 > 7.0) as i32) as u32 as F32;
        let mut iSlow60: i32 =
            (F32::min(4096.0, F32::max(0.0, fSlow5 + 5.0 * fSlow4 + -1.5))) as i32;
        let mut fSlow61: F32 = ((fSlow7 > 5.0) as i32) as u32 as F32;
        let mut iSlow62: i32 =
            (F32::min(4096.0, F32::max(0.0, fSlow5 + 3.0 * fSlow4 + -1.5))) as i32;
        let mut fSlow63: F32 = ((fSlow7 > 3.0) as i32) as u32 as F32;
        let mut iSlow64: i32 = (F32::min(4096.0, F32::max(0.0, fSlow4 + fSlow5 + -1.5))) as i32;
        let mut fSlow65: F32 = ((fSlow7 > 1.0) as i32) as u32 as F32;
        let mut iSlow66: i32 =
            (F32::min(4096.0, F32::max(0.0, fSlow5 + 9.0 * fSlow4 + -1.5))) as i32;
        let mut fSlow67: F32 = ((fSlow7 > 9.0) as i32) as u32 as F32;
        let mut iSlow68: i32 =
            (F32::min(4096.0, F32::max(0.0, fSlow5 + 13.0 * fSlow4 + -1.5))) as i32;
        let mut fSlow69: F32 = ((fSlow7 > 13.0) as i32) as u32 as F32;
        let mut iSlow70: i32 =
            (F32::min(4096.0, F32::max(0.0, fSlow5 + 21.0 * fSlow4 + -1.5))) as i32;
        let mut fSlow71: F32 = ((fSlow7 > 21.0) as i32) as u32 as F32;
        let zipped_iterators = outputs0.zip(outputs1);
        for (output0, output1) in zipped_iterators {
            self.iRec1[0] = i32::wrapping_add(i32::wrapping_mul(1103515245, self.iRec1[1]), 12345);
            self.fVec0[0] = fSlow2;
            self.fRec2[0] = self.fRec2[1] + (((fSlow2 - self.fVec0[1]) > 0.0) as i32) as u32 as F32
                - fSlow1 * ((self.fRec2[1] > 0.0) as i32) as u32 as F32;
            let mut fTemp0: F32 = fSlow3
                * (((self.fRec2[0] > 0.0) as i32) as u32 as F32 + 1.5258789e-05)
                * (self.iRec1[0]) as F32;
            self.fVec1[(self.IOTA0 & 8191) as usize] =
                fTemp0 + fSlow0 * (self.fRec0[1] + self.fRec0[2]);
            self.fRec0[0] = self.fVec1[((i32::wrapping_sub(self.IOTA0, iSlow6)) & 8191) as usize];
            self.fVec2[(self.IOTA0 & 8191) as usize] =
                fTemp0 + fSlow0 * (self.fRec3[1] + self.fRec3[2]);
            self.fRec3[0] = self.fVec2[((i32::wrapping_sub(self.IOTA0, iSlow9)) & 8191) as usize];
            self.fVec3[(self.IOTA0 & 8191) as usize] =
                fTemp0 + fSlow0 * (self.fRec4[1] + self.fRec4[2]);
            self.fRec4[0] = self.fVec3[((i32::wrapping_sub(self.IOTA0, iSlow11)) & 8191) as usize];
            self.fVec4[(self.IOTA0 & 8191) as usize] =
                fTemp0 + fSlow0 * (self.fRec5[1] + self.fRec5[2]);
            self.fRec5[0] = self.fVec4[((i32::wrapping_sub(self.IOTA0, iSlow13)) & 8191) as usize];
            self.fVec5[(self.IOTA0 & 8191) as usize] =
                fTemp0 + fSlow0 * (self.fRec6[1] + self.fRec6[2]);
            self.fRec6[0] = self.fVec5[((i32::wrapping_sub(self.IOTA0, iSlow15)) & 8191) as usize];
            self.fVec6[(self.IOTA0 & 8191) as usize] =
                fTemp0 + fSlow0 * (self.fRec7[1] + self.fRec7[2]);
            self.fRec7[0] = self.fVec6[((i32::wrapping_sub(self.IOTA0, iSlow17)) & 8191) as usize];
            self.fVec7[(self.IOTA0 & 8191) as usize] =
                fTemp0 + fSlow0 * (self.fRec8[1] + self.fRec8[2]);
            self.fRec8[0] = self.fVec7[((i32::wrapping_sub(self.IOTA0, iSlow19)) & 8191) as usize];
            self.fVec8[(self.IOTA0 & 8191) as usize] =
                fTemp0 + fSlow0 * (self.fRec9[1] + self.fRec9[2]);
            self.fRec9[0] = self.fVec8[((i32::wrapping_sub(self.IOTA0, iSlow21)) & 8191) as usize];
            self.fVec9[(self.IOTA0 & 8191) as usize] =
                fTemp0 + fSlow0 * (self.fRec10[1] + self.fRec10[2]);
            self.fRec10[0] = self.fVec9[((i32::wrapping_sub(self.IOTA0, iSlow23)) & 8191) as usize];
            self.fVec10[(self.IOTA0 & 8191) as usize] =
                fTemp0 + fSlow0 * (self.fRec11[1] + self.fRec11[2]);
            self.fRec11[0] =
                self.fVec10[((i32::wrapping_sub(self.IOTA0, iSlow25)) & 8191) as usize];
            self.fVec11[(self.IOTA0 & 4095) as usize] =
                fTemp0 + fSlow0 * (self.fRec12[1] + self.fRec12[2]);
            self.fRec12[0] =
                self.fVec11[((i32::wrapping_sub(self.IOTA0, iSlow27)) & 4095) as usize];
            self.fVec12[(self.IOTA0 & 2047) as usize] =
                fTemp0 + fSlow0 * (self.fRec13[1] + self.fRec13[2]);
            self.fRec13[0] =
                self.fVec12[((i32::wrapping_sub(self.IOTA0, iSlow29)) & 2047) as usize];
            self.fVec13[(self.IOTA0 & 511) as usize] =
                fSlow0 * (self.fRec14[1] + self.fRec14[2]) + fTemp0;
            self.fRec14[0] = self.fVec13[((i32::wrapping_sub(self.IOTA0, iSlow31)) & 511) as usize];
            self.fVec14[(self.IOTA0 & 4095) as usize] =
                fTemp0 + fSlow0 * (self.fRec15[1] + self.fRec15[2]);
            self.fRec15[0] =
                self.fVec14[((i32::wrapping_sub(self.IOTA0, iSlow33)) & 4095) as usize];
            self.fVec15[(self.IOTA0 & 8191) as usize] =
                fTemp0 + fSlow0 * (self.fRec16[1] + self.fRec16[2]);
            self.fRec16[0] =
                self.fVec15[((i32::wrapping_sub(self.IOTA0, iSlow35)) & 8191) as usize];
            self.fVec16[(self.IOTA0 & 8191) as usize] =
                fTemp0 + fSlow0 * (self.fRec17[1] + self.fRec17[2]);
            self.fRec17[0] =
                self.fVec16[((i32::wrapping_sub(self.IOTA0, iSlow37)) & 8191) as usize];
            *output0 = fSlow39
                * (fSlow38 * self.fRec17[0]
                    + fSlow36 * self.fRec16[0]
                    + fSlow34 * self.fRec15[0]
                    + fSlow32 * self.fRec14[0]
                    + fSlow30 * self.fRec13[0]
                    + fSlow28 * self.fRec12[0]
                    + fSlow26 * self.fRec11[0]
                    + fSlow24 * self.fRec10[0]
                    + fSlow22 * self.fRec9[0]
                    + fSlow20 * self.fRec8[0]
                    + fSlow18 * self.fRec7[0]
                    + fSlow16 * self.fRec6[0]
                    + fSlow14 * self.fRec5[0]
                    + fSlow12 * self.fRec4[0]
                    + fSlow10 * self.fRec3[0]
                    + fSlow8 * self.fRec0[0]);
            self.fVec17[(self.IOTA0 & 8191) as usize] =
                fTemp0 + fSlow0 * (self.fRec18[1] + self.fRec18[2]);
            self.fRec18[0] =
                self.fVec17[((i32::wrapping_sub(self.IOTA0, iSlow40)) & 8191) as usize];
            self.fVec18[(self.IOTA0 & 8191) as usize] =
                fTemp0 + fSlow0 * (self.fRec19[1] + self.fRec19[2]);
            self.fRec19[0] =
                self.fVec18[((i32::wrapping_sub(self.IOTA0, iSlow42)) & 8191) as usize];
            self.fVec19[(self.IOTA0 & 8191) as usize] =
                fTemp0 + fSlow0 * (self.fRec20[1] + self.fRec20[2]);
            self.fRec20[0] =
                self.fVec19[((i32::wrapping_sub(self.IOTA0, iSlow44)) & 8191) as usize];
            self.fVec20[(self.IOTA0 & 8191) as usize] =
                fTemp0 + fSlow0 * (self.fRec21[1] + self.fRec21[2]);
            self.fRec21[0] =
                self.fVec20[((i32::wrapping_sub(self.IOTA0, iSlow46)) & 8191) as usize];
            self.fVec21[(self.IOTA0 & 8191) as usize] =
                fTemp0 + fSlow0 * (self.fRec22[1] + self.fRec22[2]);
            self.fRec22[0] =
                self.fVec21[((i32::wrapping_sub(self.IOTA0, iSlow48)) & 8191) as usize];
            self.fVec22[(self.IOTA0 & 8191) as usize] =
                fTemp0 + fSlow0 * (self.fRec23[1] + self.fRec23[2]);
            self.fRec23[0] =
                self.fVec22[((i32::wrapping_sub(self.IOTA0, iSlow50)) & 8191) as usize];
            self.fVec23[(self.IOTA0 & 8191) as usize] =
                fTemp0 + fSlow0 * (self.fRec24[1] + self.fRec24[2]);
            self.fRec24[0] =
                self.fVec23[((i32::wrapping_sub(self.IOTA0, iSlow52)) & 8191) as usize];
            self.fVec24[(self.IOTA0 & 8191) as usize] =
                fTemp0 + fSlow0 * (self.fRec25[1] + self.fRec25[2]);
            self.fRec25[0] =
                self.fVec24[((i32::wrapping_sub(self.IOTA0, iSlow54)) & 8191) as usize];
            self.fVec25[(self.IOTA0 & 8191) as usize] =
                fTemp0 + fSlow0 * (self.fRec26[1] + self.fRec26[2]);
            self.fRec26[0] =
                self.fVec25[((i32::wrapping_sub(self.IOTA0, iSlow56)) & 8191) as usize];
            self.fVec26[(self.IOTA0 & 4095) as usize] =
                fTemp0 + fSlow0 * (self.fRec27[1] + self.fRec27[2]);
            self.fRec27[0] =
                self.fVec26[((i32::wrapping_sub(self.IOTA0, iSlow58)) & 4095) as usize];
            self.fVec27[(self.IOTA0 & 4095) as usize] =
                fTemp0 + fSlow0 * (self.fRec28[1] + self.fRec28[2]);
            self.fRec28[0] =
                self.fVec27[((i32::wrapping_sub(self.IOTA0, iSlow60)) & 4095) as usize];
            self.fVec28[(self.IOTA0 & 2047) as usize] =
                fTemp0 + fSlow0 * (self.fRec29[1] + self.fRec29[2]);
            self.fRec29[0] =
                self.fVec28[((i32::wrapping_sub(self.IOTA0, iSlow62)) & 2047) as usize];
            self.fVec29[(self.IOTA0 & 1023) as usize] =
                fTemp0 + fSlow0 * (self.fRec30[1] + self.fRec30[2]);
            self.fRec30[0] =
                self.fVec29[((i32::wrapping_sub(self.IOTA0, iSlow64)) & 1023) as usize];
            self.fVec30[(self.IOTA0 & 8191) as usize] =
                fTemp0 + fSlow0 * (self.fRec31[1] + self.fRec31[2]);
            self.fRec31[0] =
                self.fVec30[((i32::wrapping_sub(self.IOTA0, iSlow66)) & 8191) as usize];
            self.fVec31[(self.IOTA0 & 8191) as usize] =
                fTemp0 + fSlow0 * (self.fRec32[1] + self.fRec32[2]);
            self.fRec32[0] =
                self.fVec31[((i32::wrapping_sub(self.IOTA0, iSlow68)) & 8191) as usize];
            self.fVec32[(self.IOTA0 & 8191) as usize] =
                fTemp0 + fSlow0 * (self.fRec33[1] + self.fRec33[2]);
            self.fRec33[0] =
                self.fVec32[((i32::wrapping_sub(self.IOTA0, iSlow70)) & 8191) as usize];
            *output1 = fSlow39
                * (fSlow71 * self.fRec33[0]
                    + fSlow69 * self.fRec32[0]
                    + fSlow67 * self.fRec31[0]
                    + fSlow65 * self.fRec30[0]
                    + fSlow63 * self.fRec29[0]
                    + fSlow61 * self.fRec28[0]
                    + fSlow59 * self.fRec27[0]
                    + fSlow57 * self.fRec26[0]
                    + fSlow55 * self.fRec25[0]
                    + fSlow53 * self.fRec24[0]
                    + fSlow51 * self.fRec23[0]
                    + fSlow49 * self.fRec22[0]
                    + fSlow47 * self.fRec21[0]
                    + fSlow45 * self.fRec20[0]
                    + fSlow43 * self.fRec19[0]
                    + fSlow41 * self.fRec18[0]);
            self.iRec1[1] = self.iRec1[0];
            self.fVec0[1] = self.fVec0[0];
            self.fRec2[1] = self.fRec2[0];
            self.IOTA0 = i32::wrapping_add(self.IOTA0, 1);
            self.fRec0[2] = self.fRec0[1];
            self.fRec0[1] = self.fRec0[0];
            self.fRec3[2] = self.fRec3[1];
            self.fRec3[1] = self.fRec3[0];
            self.fRec4[2] = self.fRec4[1];
            self.fRec4[1] = self.fRec4[0];
            self.fRec5[2] = self.fRec5[1];
            self.fRec5[1] = self.fRec5[0];
            self.fRec6[2] = self.fRec6[1];
            self.fRec6[1] = self.fRec6[0];
            self.fRec7[2] = self.fRec7[1];
            self.fRec7[1] = self.fRec7[0];
            self.fRec8[2] = self.fRec8[1];
            self.fRec8[1] = self.fRec8[0];
            self.fRec9[2] = self.fRec9[1];
            self.fRec9[1] = self.fRec9[0];
            self.fRec10[2] = self.fRec10[1];
            self.fRec10[1] = self.fRec10[0];
            self.fRec11[2] = self.fRec11[1];
            self.fRec11[1] = self.fRec11[0];
            self.fRec12[2] = self.fRec12[1];
            self.fRec12[1] = self.fRec12[0];
            self.fRec13[2] = self.fRec13[1];
            self.fRec13[1] = self.fRec13[0];
            self.fRec14[2] = self.fRec14[1];
            self.fRec14[1] = self.fRec14[0];
            self.fRec15[2] = self.fRec15[1];
            self.fRec15[1] = self.fRec15[0];
            self.fRec16[2] = self.fRec16[1];
            self.fRec16[1] = self.fRec16[0];
            self.fRec17[2] = self.fRec17[1];
            self.fRec17[1] = self.fRec17[0];
            self.fRec18[2] = self.fRec18[1];
            self.fRec18[1] = self.fRec18[0];
            self.fRec19[2] = self.fRec19[1];
            self.fRec19[1] = self.fRec19[0];
            self.fRec20[2] = self.fRec20[1];
            self.fRec20[1] = self.fRec20[0];
            self.fRec21[2] = self.fRec21[1];
            self.fRec21[1] = self.fRec21[0];
            self.fRec22[2] = self.fRec22[1];
            self.fRec22[1] = self.fRec22[0];
            self.fRec23[2] = self.fRec23[1];
            self.fRec23[1] = self.fRec23[0];
            self.fRec24[2] = self.fRec24[1];
            self.fRec24[1] = self.fRec24[0];
            self.fRec25[2] = self.fRec25[1];
            self.fRec25[1] = self.fRec25[0];
            self.fRec26[2] = self.fRec26[1];
            self.fRec26[1] = self.fRec26[0];
            self.fRec27[2] = self.fRec27[1];
            self.fRec27[1] = self.fRec27[0];
            self.fRec28[2] = self.fRec28[1];
            self.fRec28[1] = self.fRec28[0];
            self.fRec29[2] = self.fRec29[1];
            self.fRec29[1] = self.fRec29[0];
            self.fRec30[2] = self.fRec30[1];
            self.fRec30[1] = self.fRec30[0];
            self.fRec31[2] = self.fRec31[1];
            self.fRec31[1] = self.fRec31[0];
            self.fRec32[2] = self.fRec32[1];
            self.fRec32[1] = self.fRec32[0];
            self.fRec33[2] = self.fRec33[1];
            self.fRec33[1] = self.fRec33[0];
        }
    }
}

#[allow(non_snake_case)]
#[cfg_attr(feature = "default-boxed", derive(default_boxed::DefaultBoxed))]
#[cfg_attr(feature = "reprc", repr(C))]
pub struct mydsp_vec {
    fHslider0: F32,
    fButton0: F32,
    fVec0_perm: [F32; 4],
    fRec1_perm: [F32; 4],
    iRec2_perm: [i32; 4],
    fHslider1: F32,
    fHslider2: F32,
    fYec0: [F32; 8192],
    fYec0_idx: i32,
    fYec0_idx_save: i32,
    fHslider3: F32,
    fHslider4: F32,
    fRec0_perm: [F32; 4],
    fYec1: [F32; 8192],
    fYec1_idx: i32,
    fYec1_idx_save: i32,
    fRec3_perm: [F32; 4],
    fYec2: [F32; 8192],
    fYec2_idx: i32,
    fYec2_idx_save: i32,
    fRec4_perm: [F32; 4],
    fYec3: [F32; 8192],
    fYec3_idx: i32,
    fYec3_idx_save: i32,
    fRec5_perm: [F32; 4],
    fYec4: [F32; 8192],
    fYec4_idx: i32,
    fYec4_idx_save: i32,
    fRec6_perm: [F32; 4],
    fYec5: [F32; 4096],
    fYec5_idx: i32,
    fYec5_idx_save: i32,
    fRec7_perm: [F32; 4],
    fYec6: [F32; 1024],
    fYec6_idx: i32,
    fYec6_idx_save: i32,
    fRec8_perm: [F32; 4],
    fYec7: [F32; 2048],
    fYec7_idx: i32,
    fYec7_idx_save: i32,
    fRec9_perm: [F32; 4],
    fYec8: [F32; 4096],
    fYec8_idx: i32,
    fYec8_idx_save: i32,
    fRec10_perm: [F32; 4],
    fYec9: [F32; 8192],
    fYec9_idx: i32,
    fYec9_idx_save: i32,
    fRec11_perm: [F32; 4],
    fYec10: [F32; 8192],
    fYec10_idx: i32,
    fYec10_idx_save: i32,
    fRec12_perm: [F32; 4],
    fYec11: [F32; 8192],
    fYec11_idx: i32,
    fYec11_idx_save: i32,
    fRec13_perm: [F32; 4],
    fYec12: [F32; 8192],
    fYec12_idx: i32,
    fYec12_idx_save: i32,
    fRec14_perm: [F32; 4],
    fYec13: [F32; 8192],
    fYec13_idx: i32,
    fYec13_idx_save: i32,
    fRec15_perm: [F32; 4],
    fYec14: [F32; 8192],
    fYec14_idx: i32,
    fYec14_idx_save: i32,
    fRec16_perm: [F32; 4],
    fYec15: [F32; 8192],
    fYec15_idx: i32,
    fYec15_idx_save: i32,
    fRec17_perm: [F32; 4],
    fHslider5: F32,
    fHslider6: F32,
    fYec16: [F32; 8192],
    fYec16_idx: i32,
    fYec16_idx_save: i32,
    fRec18_perm: [F32; 4],
    fYec17: [F32; 8192],
    fYec17_idx: i32,
    fYec17_idx_save: i32,
    fRec19_perm: [F32; 4],
    fYec18: [F32; 8192],
    fYec18_idx: i32,
    fYec18_idx_save: i32,
    fRec20_perm: [F32; 4],
    fYec19: [F32; 8192],
    fYec19_idx: i32,
    fYec19_idx_save: i32,
    fRec21_perm: [F32; 4],
    fYec20: [F32; 4096],
    fYec20_idx: i32,
    fYec20_idx_save: i32,
    fRec22_perm: [F32; 4],
    fYec21: [F32; 8192],
    fYec21_idx: i32,
    fYec21_idx_save: i32,
    fRec23_perm: [F32; 4],
    fYec22: [F32; 2048],
    fYec22_idx: i32,
    fYec22_idx_save: i32,
    fRec24_perm: [F32; 4],
    fYec23: [F32; 4096],
    fYec23_idx: i32,
    fYec23_idx_save: i32,
    fRec25_perm: [F32; 4],
    fYec24: [F32; 8192],
    fYec24_idx: i32,
    fYec24_idx_save: i32,
    fRec26_perm: [F32; 4],
    fYec25: [F32; 8192],
    fYec25_idx: i32,
    fYec25_idx_save: i32,
    fRec27_perm: [F32; 4],
    fYec26: [F32; 8192],
    fYec26_idx: i32,
    fYec26_idx_save: i32,
    fRec28_perm: [F32; 4],
    fYec27: [F32; 8192],
    fYec27_idx: i32,
    fYec27_idx_save: i32,
    fRec29_perm: [F32; 4],
    fYec28: [F32; 8192],
    fYec28_idx: i32,
    fYec28_idx_save: i32,
    fRec30_perm: [F32; 4],
    fYec29: [F32; 8192],
    fYec29_idx: i32,
    fYec29_idx_save: i32,
    fRec31_perm: [F32; 4],
    fYec30: [F32; 8192],
    fYec30_idx: i32,
    fYec30_idx_save: i32,
    fRec32_perm: [F32; 4],
    fYec31: [F32; 8192],
    fYec31_idx: i32,
    fYec31_idx_save: i32,
    fRec33_perm: [F32; 4],
    fSampleRate: i32,
}

impl mydsp_vec {
    fn new() -> Self {
        Self {
            fHslider0: 0.0,
            fButton0: 0.0,
            fVec0_perm: [0.0; 4],
            fRec1_perm: [0.0; 4],
            iRec2_perm: [0; 4],
            fHslider1: 0.0,
            fHslider2: 0.0,
            fYec0: [0.0; 8192],
            fYec0_idx: 0,
            fYec0_idx_save: 0,
            fHslider3: 0.0,
            fHslider4: 0.0,
            fRec0_perm: [0.0; 4],
            fYec1: [0.0; 8192],
            fYec1_idx: 0,
            fYec1_idx_save: 0,
            fRec3_perm: [0.0; 4],
            fYec2: [0.0; 8192],
            fYec2_idx: 0,
            fYec2_idx_save: 0,
            fRec4_perm: [0.0; 4],
            fYec3: [0.0; 8192],
            fYec3_idx: 0,
            fYec3_idx_save: 0,
            fRec5_perm: [0.0; 4],
            fYec4: [0.0; 8192],
            fYec4_idx: 0,
            fYec4_idx_save: 0,
            fRec6_perm: [0.0; 4],
            fYec5: [0.0; 4096],
            fYec5_idx: 0,
            fYec5_idx_save: 0,
            fRec7_perm: [0.0; 4],
            fYec6: [0.0; 1024],
            fYec6_idx: 0,
            fYec6_idx_save: 0,
            fRec8_perm: [0.0; 4],
            fYec7: [0.0; 2048],
            fYec7_idx: 0,
            fYec7_idx_save: 0,
            fRec9_perm: [0.0; 4],
            fYec8: [0.0; 4096],
            fYec8_idx: 0,
            fYec8_idx_save: 0,
            fRec10_perm: [0.0; 4],
            fYec9: [0.0; 8192],
            fYec9_idx: 0,
            fYec9_idx_save: 0,
            fRec11_perm: [0.0; 4],
            fYec10: [0.0; 8192],
            fYec10_idx: 0,
            fYec10_idx_save: 0,
            fRec12_perm: [0.0; 4],
            fYec11: [0.0; 8192],
            fYec11_idx: 0,
            fYec11_idx_save: 0,
            fRec13_perm: [0.0; 4],
            fYec12: [0.0; 8192],
            fYec12_idx: 0,
            fYec12_idx_save: 0,
            fRec14_perm: [0.0; 4],
            fYec13: [0.0; 8192],
            fYec13_idx: 0,
            fYec13_idx_save: 0,
            fRec15_perm: [0.0; 4],
            fYec14: [0.0; 8192],
            fYec14_idx: 0,
            fYec14_idx_save: 0,
            fRec16_perm: [0.0; 4],
            fYec15: [0.0; 8192],
            fYec15_idx: 0,
            fYec15_idx_save: 0,
            fRec17_perm: [0.0; 4],
            fHslider5: 0.0,
            fHslider6: 0.0,
            fYec16: [0.0; 8192],
            fYec16_idx: 0,
            fYec16_idx_save: 0,
            fRec18_perm: [0.0; 4],
            fYec17: [0.0; 8192],
            fYec17_idx: 0,
            fYec17_idx_save: 0,
            fRec19_perm: [0.0; 4],
            fYec18: [0.0; 8192],
            fYec18_idx: 0,
            fYec18_idx_save: 0,
            fRec20_perm: [0.0; 4],
            fYec19: [0.0; 8192],
            fYec19_idx: 0,
            fYec19_idx_save: 0,
            fRec21_perm: [0.0; 4],
            fYec20: [0.0; 4096],
            fYec20_idx: 0,
            fYec20_idx_save: 0,
            fRec22_perm: [0.0; 4],
            fYec21: [0.0; 8192],
            fYec21_idx: 0,
            fYec21_idx_save: 0,
            fRec23_perm: [0.0; 4],
            fYec22: [0.0; 2048],
            fYec22_idx: 0,
            fYec22_idx_save: 0,
            fRec24_perm: [0.0; 4],
            fYec23: [0.0; 4096],
            fYec23_idx: 0,
            fYec23_idx_save: 0,
            fRec25_perm: [0.0; 4],
            fYec24: [0.0; 8192],
            fYec24_idx: 0,
            fYec24_idx_save: 0,
            fRec26_perm: [0.0; 4],
            fYec25: [0.0; 8192],
            fYec25_idx: 0,
            fYec25_idx_save: 0,
            fRec27_perm: [0.0; 4],
            fYec26: [0.0; 8192],
            fYec26_idx: 0,
            fYec26_idx_save: 0,
            fRec28_perm: [0.0; 4],
            fYec27: [0.0; 8192],
            fYec27_idx: 0,
            fYec27_idx_save: 0,
            fRec29_perm: [0.0; 4],
            fYec28: [0.0; 8192],
            fYec28_idx: 0,
            fYec28_idx_save: 0,
            fRec30_perm: [0.0; 4],
            fYec29: [0.0; 8192],
            fYec29_idx: 0,
            fYec29_idx_save: 0,
            fRec31_perm: [0.0; 4],
            fYec30: [0.0; 8192],
            fYec30_idx: 0,
            fYec30_idx_save: 0,
            fRec32_perm: [0.0; 4],
            fYec31: [0.0; 8192],
            fYec31_idx: 0,
            fYec31_idx_save: 0,
            fRec33_perm: [0.0; 4],
            fSampleRate: 0,
        }
    }

    #[inline(never)]
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    fn compute_4(&mut self, count: i32, inputs: &[&[F32]], outputs: &mut [&mut [F32]]) {
        const vsize: i32 = 4;
        let (outputs0, outputs1) = if let [outputs0, outputs1, ..] = outputs {
            let outputs0 = outputs0[..count as usize].chunks_mut(vsize as usize);
            let outputs1 = outputs1[..count as usize].chunks_mut(vsize as usize);
            (outputs0, outputs1)
        } else {
            panic!("wrong number of outputs");
        };
        let mut fSlow0: F32 = 1.0 / self.fHslider0;
        let mut fSlow1: F32 = self.fButton0;
        let mut fVec0_tmp: [F32; 8] = [0.0; 8];
        let mut fRec1_tmp: [F32; 8] = [0.0; 8];
        let mut iRec2_tmp: [i32; 8] = [0; 8];
        let mut fSlow2: F32 = 0.5 * (1.0 - self.fHslider1);
        let mut fSlow3: F32 = 4.656613e-10 * self.fHslider2;
        let mut fZec0: [F32; 4] = [0.0; 4];
        let mut fSlow4: F32 = self.fHslider3;
        let mut fSlow5: F32 = self.fHslider4;
        let mut iSlow6: i32 =
            (F32::min(4096.0, F32::max(0.0, fSlow5 + 26.0 * fSlow4 + -1.5))) as i32;
        let mut fRec0_tmp: [F32; 8] = [0.0; 8];
        let mut iSlow7: i32 =
            (F32::min(4096.0, F32::max(0.0, fSlow5 + 22.0 * fSlow4 + -1.5))) as i32;
        let mut fRec3_tmp: [F32; 8] = [0.0; 8];
        let mut iSlow8: i32 =
            (F32::min(4096.0, F32::max(0.0, fSlow5 + 24.0 * fSlow4 + -1.5))) as i32;
        let mut fRec4_tmp: [F32; 8] = [0.0; 8];
        let mut iSlow9: i32 =
            (F32::min(4096.0, F32::max(0.0, fSlow5 + 12.0 * fSlow4 + -1.5))) as i32;
        let mut fRec5_tmp: [F32; 8] = [0.0; 8];
        let mut iSlow10: i32 =
            (F32::min(4096.0, F32::max(0.0, fSlow5 + 8.0 * fSlow4 + -1.5))) as i32;
        let mut fRec6_tmp: [F32; 8] = [0.0; 8];
        let mut iSlow11: i32 =
            (F32::min(4096.0, F32::max(0.0, fSlow5 + 6.0 * fSlow4 + -1.5))) as i32;
        let mut fRec7_tmp: [F32; 8] = [0.0; 8];
        let mut iSlow12: i32 = (F32::min(4096.0, F32::max(0.0, fSlow5 + -1.5))) as i32;
        let mut fRec8_tmp: [F32; 8] = [0.0; 8];
        let mut iSlow13: i32 =
            (F32::min(4096.0, F32::max(0.0, fSlow5 + 2.0 * fSlow4 + -1.5))) as i32;
        let mut fRec9_tmp: [F32; 8] = [0.0; 8];
        let mut iSlow14: i32 =
            (F32::min(4096.0, F32::max(0.0, fSlow5 + 4.0 * fSlow4 + -1.5))) as i32;
        let mut fRec10_tmp: [F32; 8] = [0.0; 8];
        let mut iSlow15: i32 =
            (F32::min(4096.0, F32::max(0.0, fSlow5 + 1e+01 * fSlow4 + -1.5))) as i32;
        let mut fRec11_tmp: [F32; 8] = [0.0; 8];
        let mut iSlow16: i32 =
            (F32::min(4096.0, F32::max(0.0, fSlow5 + 14.0 * fSlow4 + -1.5))) as i32;
        let mut fRec12_tmp: [F32; 8] = [0.0; 8];
        let mut iSlow17: i32 =
            (F32::min(4096.0, F32::max(0.0, fSlow5 + 16.0 * fSlow4 + -1.5))) as i32;
        let mut fRec13_tmp: [F32; 8] = [0.0; 8];
        let mut iSlow18: i32 =
            (F32::min(4096.0, F32::max(0.0, fSlow5 + 18.0 * fSlow4 + -1.5))) as i32;
        let mut fRec14_tmp: [F32; 8] = [0.0; 8];
        let mut iSlow19: i32 =
            (F32::min(4096.0, F32::max(0.0, fSlow5 + 2e+01 * fSlow4 + -1.5))) as i32;
        let mut fRec15_tmp: [F32; 8] = [0.0; 8];
        let mut iSlow20: i32 =
            (F32::min(4096.0, F32::max(0.0, fSlow5 + 28.0 * fSlow4 + -1.5))) as i32;
        let mut fRec16_tmp: [F32; 8] = [0.0; 8];
        let mut iSlow21: i32 =
            (F32::min(4096.0, F32::max(0.0, fSlow5 + 3e+01 * fSlow4 + -1.5))) as i32;
        let mut fRec17_tmp: [F32; 8] = [0.0; 8];
        let mut fSlow22: F32 = self.fHslider5;
        let mut fSlow23: F32 = ((fSlow22 > 3e+01) as i32) as u32 as F32;
        let mut fSlow24: F32 = ((fSlow22 > 28.0) as i32) as u32 as F32;
        let mut fSlow25: F32 = ((fSlow22 > 2e+01) as i32) as u32 as F32;
        let mut fSlow26: F32 = ((fSlow22 > 18.0) as i32) as u32 as F32;
        let mut fSlow27: F32 = ((fSlow22 > 16.0) as i32) as u32 as F32;
        let mut fSlow28: F32 = ((fSlow22 > 14.0) as i32) as u32 as F32;
        let mut fSlow29: F32 = ((fSlow22 > 1e+01) as i32) as u32 as F32;
        let mut fSlow30: F32 = ((fSlow22 > 4.0) as i32) as u32 as F32;
        let mut fSlow31: F32 = ((fSlow22 > 2.0) as i32) as u32 as F32;
        let mut fSlow32: F32 = ((fSlow22 > 0.0) as i32) as u32 as F32;
        let mut fSlow33: F32 = ((fSlow22 > 6.0) as i32) as u32 as F32;
        let mut fSlow34: F32 = ((fSlow22 > 8.0) as i32) as u32 as F32;
        let mut fSlow35: F32 = ((fSlow22 > 12.0) as i32) as u32 as F32;
        let mut fSlow36: F32 = ((fSlow22 > 24.0) as i32) as u32 as F32;
        let mut fSlow37: F32 = ((fSlow22 > 22.0) as i32) as u32 as F32;
        let mut fSlow38: F32 = ((fSlow22 > 26.0) as i32) as u32 as F32;
        let mut fSlow39: F32 = self.fHslider6;
        let mut iSlow40: i32 =
            (F32::min(4096.0, F32::max(0.0, fSlow5 + 25.0 * fSlow4 + -1.5))) as i32;
        let mut fRec18_tmp: [F32; 8] = [0.0; 8];
        let mut iSlow41: i32 =
            (F32::min(4096.0, F32::max(0.0, fSlow5 + 17.0 * fSlow4 + -1.5))) as i32;
        let mut fRec19_tmp: [F32; 8] = [0.0; 8];
        let mut iSlow42: i32 =
            (F32::min(4096.0, F32::max(0.0, fSlow5 + 19.0 * fSlow4 + -1.5))) as i32;
        let mut fRec20_tmp: [F32; 8] = [0.0; 8];
        let mut iSlow43: i32 =
            (F32::min(4096.0, F32::max(0.0, fSlow5 + 11.0 * fSlow4 + -1.5))) as i32;
        let mut fRec21_tmp: [F32; 8] = [0.0; 8];
        let mut iSlow44: i32 =
            (F32::min(4096.0, F32::max(0.0, fSlow5 + 3.0 * fSlow4 + -1.5))) as i32;
        let mut fRec22_tmp: [F32; 8] = [0.0; 8];
        let mut iSlow45: i32 =
            (F32::min(4096.0, F32::max(0.0, fSlow5 + 7.0 * fSlow4 + -1.5))) as i32;
        let mut fRec23_tmp: [F32; 8] = [0.0; 8];
        let mut iSlow46: i32 = (F32::min(4096.0, F32::max(0.0, fSlow4 + fSlow5 + -1.5))) as i32;
        let mut fRec24_tmp: [F32; 8] = [0.0; 8];
        let mut iSlow47: i32 =
            (F32::min(4096.0, F32::max(0.0, fSlow5 + 5.0 * fSlow4 + -1.5))) as i32;
        let mut fRec25_tmp: [F32; 8] = [0.0; 8];
        let mut iSlow48: i32 =
            (F32::min(4096.0, F32::max(0.0, fSlow5 + 9.0 * fSlow4 + -1.5))) as i32;
        let mut fRec26_tmp: [F32; 8] = [0.0; 8];
        let mut iSlow49: i32 =
            (F32::min(4096.0, F32::max(0.0, fSlow5 + 13.0 * fSlow4 + -1.5))) as i32;
        let mut fRec27_tmp: [F32; 8] = [0.0; 8];
        let mut iSlow50: i32 =
            (F32::min(4096.0, F32::max(0.0, fSlow5 + 15.0 * fSlow4 + -1.5))) as i32;
        let mut fRec28_tmp: [F32; 8] = [0.0; 8];
        let mut iSlow51: i32 =
            (F32::min(4096.0, F32::max(0.0, fSlow5 + 21.0 * fSlow4 + -1.5))) as i32;
        let mut fRec29_tmp: [F32; 8] = [0.0; 8];
        let mut iSlow52: i32 =
            (F32::min(4096.0, F32::max(0.0, fSlow5 + 23.0 * fSlow4 + -1.5))) as i32;
        let mut fRec30_tmp: [F32; 8] = [0.0; 8];
        let mut iSlow53: i32 =
            (F32::min(4096.0, F32::max(0.0, fSlow5 + 27.0 * fSlow4 + -1.5))) as i32;
        let mut fRec31_tmp: [F32; 8] = [0.0; 8];
        let mut iSlow54: i32 =
            (F32::min(4096.0, F32::max(0.0, fSlow5 + 29.0 * fSlow4 + -1.5))) as i32;
        let mut fRec32_tmp: [F32; 8] = [0.0; 8];
        let mut iSlow55: i32 =
            (F32::min(4096.0, F32::max(0.0, fSlow5 + 31.0 * fSlow4 + -1.5))) as i32;
        let mut fRec33_tmp: [F32; 8] = [0.0; 8];
        let mut fSlow56: F32 = ((fSlow22 > 31.0) as i32) as u32 as F32;
        let mut fSlow57: F32 = ((fSlow22 > 29.0) as i32) as u32 as F32;
        let mut fSlow58: F32 = ((fSlow22 > 27.0) as i32) as u32 as F32;
        let mut fSlow59: F32 = ((fSlow22 > 23.0) as i32) as u32 as F32;
        let mut fSlow60: F32 = ((fSlow22 > 21.0) as i32) as u32 as F32;
        let mut fSlow61: F32 = ((fSlow22 > 15.0) as i32) as u32 as F32;
        let mut fSlow62: F32 = ((fSlow22 > 13.0) as i32) as u32 as F32;
        let mut fSlow63: F32 = ((fSlow22 > 9.0) as i32) as u32 as F32;
        let mut fSlow64: F32 = ((fSlow22 > 5.0) as i32) as u32 as F32;
        let mut fSlow65: F32 = ((fSlow22 > 1.0) as i32) as u32 as F32;
        let mut fSlow66: F32 = ((fSlow22 > 7.0) as i32) as u32 as F32;
        let mut fSlow67: F32 = ((fSlow22 > 3.0) as i32) as u32 as F32;
        let mut fSlow68: F32 = ((fSlow22 > 11.0) as i32) as u32 as F32;
        let mut fSlow69: F32 = ((fSlow22 > 19.0) as i32) as u32 as F32;
        let mut fSlow70: F32 = ((fSlow22 > 17.0) as i32) as u32 as F32;
        let mut fSlow71: F32 = ((fSlow22 > 25.0) as i32) as u32 as F32;
        /* Main loop */
        let zipped_iterators = outputs0.zip(outputs1);
        for (output0, output1) in zipped_iterators {
            /* Vectorizable loop 0 */
            /* Pre code */
            for j0 in 0..4 {
                fVec0_tmp[j0 as usize] = self.fVec0_perm[j0 as usize];
            }
            /* Compute code */
            for i in 0..output0.len() as i32 {
                fVec0_tmp[(i32::wrapping_add(4, i)) as usize] = fSlow1;
            }
            /* Post code */
            for j1 in 0..4 {
                self.fVec0_perm[j1 as usize] = fVec0_tmp[(i32::wrapping_add(vsize, j1)) as usize];
            }
            /* Recursive loop 1 */
            /* Pre code */
            for j2 in 0..4 {
                fRec1_tmp[j2 as usize] = self.fRec1_perm[j2 as usize];
            }
            /* Compute code */
            for i in 0..output0.len() as i32 {
                fRec1_tmp[(i32::wrapping_add(4, i)) as usize] = fRec1_tmp
                    [(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]
                    + (((fSlow1
                        - fVec0_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize])
                        > 0.0) as i32) as u32 as F32
                    - fSlow0
                        * ((fRec1_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]
                            > 0.0) as i32) as u32 as F32;
            }
            /* Post code */
            for j3 in 0..4 {
                self.fRec1_perm[j3 as usize] = fRec1_tmp[(i32::wrapping_add(vsize, j3)) as usize];
            }
            /* Recursive loop 2 */
            /* Pre code */
            for j4 in 0..4 {
                iRec2_tmp[j4 as usize] = self.iRec2_perm[j4 as usize];
            }
            /* Compute code */
            for i in 0..output0.len() as i32 {
                iRec2_tmp[(i32::wrapping_add(4, i)) as usize] = i32::wrapping_add(
                    i32::wrapping_mul(
                        1103515245,
                        iRec2_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize],
                    ),
                    12345,
                );
            }
            /* Post code */
            for j5 in 0..4 {
                self.iRec2_perm[j5 as usize] = iRec2_tmp[(i32::wrapping_add(vsize, j5)) as usize];
            }
            /* Vectorizable loop 3 */
            /* Compute code */
            for i in 0..output0.len() as i32 {
                fZec0[i as usize] = fSlow3
                    * (((fRec1_tmp[(i32::wrapping_add(4, i)) as usize] > 0.0) as i32) as u32
                        as F32
                        + 1.5258789e-05)
                    * (iRec2_tmp[(i32::wrapping_add(4, i)) as usize]) as F32;
            }
            /* Recursive loop 4 */
            /* Pre code */
            self.fYec0_idx = (i32::wrapping_add(self.fYec0_idx, self.fYec0_idx_save)) & 8191;
            for j6 in 0..4 {
                fRec0_tmp[j6 as usize] = self.fRec0_perm[j6 as usize];
            }
            /* Compute code */
            for i in 0..output0.len() as i32 {
                self.fYec0[((i32::wrapping_add(i, self.fYec0_idx)) & 8191) as usize] = fZec0
                    [i as usize]
                    + fSlow2
                        * (fRec0_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]
                            + fRec0_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize]);
                fRec0_tmp[(i32::wrapping_add(4, i)) as usize] =
                    self.fYec0[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec0_idx), iSlow6))
                        & 8191) as usize];
            }
            /* Post code */
            self.fYec0_idx_save = vsize;
            for j7 in 0..4 {
                self.fRec0_perm[j7 as usize] = fRec0_tmp[(i32::wrapping_add(vsize, j7)) as usize];
            }
            /* Recursive loop 5 */
            /* Pre code */
            self.fYec6_idx = (i32::wrapping_add(self.fYec6_idx, self.fYec6_idx_save)) & 1023;
            for j18 in 0..4 {
                fRec8_tmp[j18 as usize] = self.fRec8_perm[j18 as usize];
            }
            /* Compute code */
            for i in 0..output0.len() as i32 {
                self.fYec6[((i32::wrapping_add(i, self.fYec6_idx)) & 1023) as usize] = fSlow2
                    * (fRec8_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]
                        + fRec8_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize])
                    + fZec0[i as usize];
                fRec8_tmp[(i32::wrapping_add(4, i)) as usize] =
                    self.fYec6[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec6_idx), iSlow12))
                        & 1023) as usize];
            }
            /* Post code */
            self.fYec6_idx_save = vsize;
            for j19 in 0..4 {
                self.fRec8_perm[j19 as usize] = fRec8_tmp[(i32::wrapping_add(vsize, j19)) as usize];
            }
            /* Recursive loop 6 */
            /* Pre code */
            self.fYec9_idx = (i32::wrapping_add(self.fYec9_idx, self.fYec9_idx_save)) & 8191;
            for j24 in 0..4 {
                fRec11_tmp[j24 as usize] = self.fRec11_perm[j24 as usize];
            }
            /* Compute code */
            for i in 0..output0.len() as i32 {
                self.fYec9[((i32::wrapping_add(i, self.fYec9_idx)) & 8191) as usize] = fZec0
                    [i as usize]
                    + fSlow2
                        * (fRec11_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]
                            + fRec11_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize]);
                fRec11_tmp[(i32::wrapping_add(4, i)) as usize] =
                    self.fYec9[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec9_idx), iSlow15))
                        & 8191) as usize];
            }
            /* Post code */
            self.fYec9_idx_save = vsize;
            for j25 in 0..4 {
                self.fRec11_perm[j25 as usize] =
                    fRec11_tmp[(i32::wrapping_add(vsize, j25)) as usize];
            }
            /* Recursive loop 7 */
            /* Pre code */
            self.fYec10_idx = (i32::wrapping_add(self.fYec10_idx, self.fYec10_idx_save)) & 8191;
            for j26 in 0..4 {
                fRec12_tmp[j26 as usize] = self.fRec12_perm[j26 as usize];
            }
            /* Compute code */
            for i in 0..output0.len() as i32 {
                self.fYec10[((i32::wrapping_add(i, self.fYec10_idx)) & 8191) as usize] = fZec0
                    [i as usize]
                    + fSlow2
                        * (fRec12_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]
                            + fRec12_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize]);
                fRec12_tmp[(i32::wrapping_add(4, i)) as usize] = self.fYec10[((i32::wrapping_sub(
                    i32::wrapping_add(i, self.fYec10_idx),
                    iSlow16,
                )) & 8191)
                    as usize];
            }
            /* Post code */
            self.fYec10_idx_save = vsize;
            for j27 in 0..4 {
                self.fRec12_perm[j27 as usize] =
                    fRec12_tmp[(i32::wrapping_add(vsize, j27)) as usize];
            }
            /* Recursive loop 8 */
            /* Pre code */
            self.fYec11_idx = (i32::wrapping_add(self.fYec11_idx, self.fYec11_idx_save)) & 8191;
            for j28 in 0..4 {
                fRec13_tmp[j28 as usize] = self.fRec13_perm[j28 as usize];
            }
            /* Compute code */
            for i in 0..output0.len() as i32 {
                self.fYec11[((i32::wrapping_add(i, self.fYec11_idx)) & 8191) as usize] = fZec0
                    [i as usize]
                    + fSlow2
                        * (fRec13_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]
                            + fRec13_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize]);
                fRec13_tmp[(i32::wrapping_add(4, i)) as usize] = self.fYec11[((i32::wrapping_sub(
                    i32::wrapping_add(i, self.fYec11_idx),
                    iSlow17,
                )) & 8191)
                    as usize];
            }
            /* Post code */
            self.fYec11_idx_save = vsize;
            for j29 in 0..4 {
                self.fRec13_perm[j29 as usize] =
                    fRec13_tmp[(i32::wrapping_add(vsize, j29)) as usize];
            }
            /* Recursive loop 9 */
            /* Pre code */
            self.fYec12_idx = (i32::wrapping_add(self.fYec12_idx, self.fYec12_idx_save)) & 8191;
            for j30 in 0..4 {
                fRec14_tmp[j30 as usize] = self.fRec14_perm[j30 as usize];
            }
            /* Compute code */
            for i in 0..output0.len() as i32 {
                self.fYec12[((i32::wrapping_add(i, self.fYec12_idx)) & 8191) as usize] = fZec0
                    [i as usize]
                    + fSlow2
                        * (fRec14_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]
                            + fRec14_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize]);
                fRec14_tmp[(i32::wrapping_add(4, i)) as usize] = self.fYec12[((i32::wrapping_sub(
                    i32::wrapping_add(i, self.fYec12_idx),
                    iSlow18,
                )) & 8191)
                    as usize];
            }
            /* Post code */
            self.fYec12_idx_save = vsize;
            for j31 in 0..4 {
                self.fRec14_perm[j31 as usize] =
                    fRec14_tmp[(i32::wrapping_add(vsize, j31)) as usize];
            }
            /* Recursive loop 10 */
            /* Pre code */
            self.fYec3_idx = (i32::wrapping_add(self.fYec3_idx, self.fYec3_idx_save)) & 8191;
            for j12 in 0..4 {
                fRec5_tmp[j12 as usize] = self.fRec5_perm[j12 as usize];
            }
            /* Compute code */
            for i in 0..output0.len() as i32 {
                self.fYec3[((i32::wrapping_add(i, self.fYec3_idx)) & 8191) as usize] = fZec0
                    [i as usize]
                    + fSlow2
                        * (fRec5_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]
                            + fRec5_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize]);
                fRec5_tmp[(i32::wrapping_add(4, i)) as usize] =
                    self.fYec3[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec3_idx), iSlow9))
                        & 8191) as usize];
            }
            /* Post code */
            self.fYec3_idx_save = vsize;
            for j13 in 0..4 {
                self.fRec5_perm[j13 as usize] = fRec5_tmp[(i32::wrapping_add(vsize, j13)) as usize];
            }
            /* Recursive loop 11 */
            /* Pre code */
            self.fYec5_idx = (i32::wrapping_add(self.fYec5_idx, self.fYec5_idx_save)) & 4095;
            for j16 in 0..4 {
                fRec7_tmp[j16 as usize] = self.fRec7_perm[j16 as usize];
            }
            /* Compute code */
            for i in 0..output0.len() as i32 {
                self.fYec5[((i32::wrapping_add(i, self.fYec5_idx)) & 4095) as usize] = fZec0
                    [i as usize]
                    + fSlow2
                        * (fRec7_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]
                            + fRec7_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize]);
                fRec7_tmp[(i32::wrapping_add(4, i)) as usize] =
                    self.fYec5[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec5_idx), iSlow11))
                        & 4095) as usize];
            }
            /* Post code */
            self.fYec5_idx_save = vsize;
            for j17 in 0..4 {
                self.fRec7_perm[j17 as usize] = fRec7_tmp[(i32::wrapping_add(vsize, j17)) as usize];
            }
            /* Recursive loop 12 */
            /* Pre code */
            self.fYec2_idx = (i32::wrapping_add(self.fYec2_idx, self.fYec2_idx_save)) & 8191;
            for j10 in 0..4 {
                fRec4_tmp[j10 as usize] = self.fRec4_perm[j10 as usize];
            }
            /* Compute code */
            for i in 0..output0.len() as i32 {
                self.fYec2[((i32::wrapping_add(i, self.fYec2_idx)) & 8191) as usize] = fZec0
                    [i as usize]
                    + fSlow2
                        * (fRec4_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]
                            + fRec4_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize]);
                fRec4_tmp[(i32::wrapping_add(4, i)) as usize] =
                    self.fYec2[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec2_idx), iSlow8))
                        & 8191) as usize];
            }
            /* Post code */
            self.fYec2_idx_save = vsize;
            for j11 in 0..4 {
                self.fRec4_perm[j11 as usize] = fRec4_tmp[(i32::wrapping_add(vsize, j11)) as usize];
            }
            /* Recursive loop 13 */
            /* Pre code */
            self.fYec8_idx = (i32::wrapping_add(self.fYec8_idx, self.fYec8_idx_save)) & 4095;
            for j22 in 0..4 {
                fRec10_tmp[j22 as usize] = self.fRec10_perm[j22 as usize];
            }
            /* Compute code */
            for i in 0..output0.len() as i32 {
                self.fYec8[((i32::wrapping_add(i, self.fYec8_idx)) & 4095) as usize] = fZec0
                    [i as usize]
                    + fSlow2
                        * (fRec10_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]
                            + fRec10_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize]);
                fRec10_tmp[(i32::wrapping_add(4, i)) as usize] =
                    self.fYec8[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec8_idx), iSlow14))
                        & 4095) as usize];
            }
            /* Post code */
            self.fYec8_idx_save = vsize;
            for j23 in 0..4 {
                self.fRec10_perm[j23 as usize] =
                    fRec10_tmp[(i32::wrapping_add(vsize, j23)) as usize];
            }
            /* Recursive loop 14 */
            /* Pre code */
            self.fYec7_idx = (i32::wrapping_add(self.fYec7_idx, self.fYec7_idx_save)) & 2047;
            for j20 in 0..4 {
                fRec9_tmp[j20 as usize] = self.fRec9_perm[j20 as usize];
            }
            /* Compute code */
            for i in 0..output0.len() as i32 {
                self.fYec7[((i32::wrapping_add(i, self.fYec7_idx)) & 2047) as usize] = fZec0
                    [i as usize]
                    + fSlow2
                        * (fRec9_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]
                            + fRec9_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize]);
                fRec9_tmp[(i32::wrapping_add(4, i)) as usize] =
                    self.fYec7[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec7_idx), iSlow13))
                        & 2047) as usize];
            }
            /* Post code */
            self.fYec7_idx_save = vsize;
            for j21 in 0..4 {
                self.fRec9_perm[j21 as usize] = fRec9_tmp[(i32::wrapping_add(vsize, j21)) as usize];
            }
            /* Recursive loop 15 */
            /* Pre code */
            self.fYec1_idx = (i32::wrapping_add(self.fYec1_idx, self.fYec1_idx_save)) & 8191;
            for j8 in 0..4 {
                fRec3_tmp[j8 as usize] = self.fRec3_perm[j8 as usize];
            }
            /* Compute code */
            for i in 0..output0.len() as i32 {
                self.fYec1[((i32::wrapping_add(i, self.fYec1_idx)) & 8191) as usize] = fZec0
                    [i as usize]
                    + fSlow2
                        * (fRec3_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]
                            + fRec3_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize]);
                fRec3_tmp[(i32::wrapping_add(4, i)) as usize] =
                    self.fYec1[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec1_idx), iSlow7))
                        & 8191) as usize];
            }
            /* Post code */
            self.fYec1_idx_save = vsize;
            for j9 in 0..4 {
                self.fRec3_perm[j9 as usize] = fRec3_tmp[(i32::wrapping_add(vsize, j9)) as usize];
            }
            /* Recursive loop 16 */
            /* Pre code */
            self.fYec4_idx = (i32::wrapping_add(self.fYec4_idx, self.fYec4_idx_save)) & 8191;
            for j14 in 0..4 {
                fRec6_tmp[j14 as usize] = self.fRec6_perm[j14 as usize];
            }
            /* Compute code */
            for i in 0..output0.len() as i32 {
                self.fYec4[((i32::wrapping_add(i, self.fYec4_idx)) & 8191) as usize] = fZec0
                    [i as usize]
                    + fSlow2
                        * (fRec6_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]
                            + fRec6_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize]);
                fRec6_tmp[(i32::wrapping_add(4, i)) as usize] =
                    self.fYec4[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec4_idx), iSlow10))
                        & 8191) as usize];
            }
            /* Post code */
            self.fYec4_idx_save = vsize;
            for j15 in 0..4 {
                self.fRec6_perm[j15 as usize] = fRec6_tmp[(i32::wrapping_add(vsize, j15)) as usize];
            }
            /* Recursive loop 17 */
            /* Pre code */
            self.fYec24_idx = (i32::wrapping_add(self.fYec24_idx, self.fYec24_idx_save)) & 8191;
            for j54 in 0..4 {
                fRec26_tmp[j54 as usize] = self.fRec26_perm[j54 as usize];
            }
            /* Compute code */
            for i in 0..output0.len() as i32 {
                self.fYec24[((i32::wrapping_add(i, self.fYec24_idx)) & 8191) as usize] = fZec0
                    [i as usize]
                    + fSlow2
                        * (fRec26_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]
                            + fRec26_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize]);
                fRec26_tmp[(i32::wrapping_add(4, i)) as usize] = self.fYec24[((i32::wrapping_sub(
                    i32::wrapping_add(i, self.fYec24_idx),
                    iSlow48,
                )) & 8191)
                    as usize];
            }
            /* Post code */
            self.fYec24_idx_save = vsize;
            for j55 in 0..4 {
                self.fRec26_perm[j55 as usize] =
                    fRec26_tmp[(i32::wrapping_add(vsize, j55)) as usize];
            }
            /* Recursive loop 18 */
            /* Pre code */
            self.fYec18_idx = (i32::wrapping_add(self.fYec18_idx, self.fYec18_idx_save)) & 8191;
            for j42 in 0..4 {
                fRec20_tmp[j42 as usize] = self.fRec20_perm[j42 as usize];
            }
            /* Compute code */
            for i in 0..output0.len() as i32 {
                self.fYec18[((i32::wrapping_add(i, self.fYec18_idx)) & 8191) as usize] = fZec0
                    [i as usize]
                    + fSlow2
                        * (fRec20_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]
                            + fRec20_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize]);
                fRec20_tmp[(i32::wrapping_add(4, i)) as usize] = self.fYec18[((i32::wrapping_sub(
                    i32::wrapping_add(i, self.fYec18_idx),
                    iSlow42,
                )) & 8191)
                    as usize];
            }
            /* Post code */
            self.fYec18_idx_save = vsize;
            for j43 in 0..4 {
                self.fRec20_perm[j43 as usize] =
                    fRec20_tmp[(i32::wrapping_add(vsize, j43)) as usize];
            }
            /* Recursive loop 19 */
            /* Pre code */
            self.fYec16_idx = (i32::wrapping_add(self.fYec16_idx, self.fYec16_idx_save)) & 8191;
            for j38 in 0..4 {
                fRec18_tmp[j38 as usize] = self.fRec18_perm[j38 as usize];
            }
            /* Compute code */
            for i in 0..output0.len() as i32 {
                self.fYec16[((i32::wrapping_add(i, self.fYec16_idx)) & 8191) as usize] = fZec0
                    [i as usize]
                    + fSlow2
                        * (fRec18_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]
                            + fRec18_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize]);
                fRec18_tmp[(i32::wrapping_add(4, i)) as usize] = self.fYec16[((i32::wrapping_sub(
                    i32::wrapping_add(i, self.fYec16_idx),
                    iSlow40,
                )) & 8191)
                    as usize];
            }
            /* Post code */
            self.fYec16_idx_save = vsize;
            for j39 in 0..4 {
                self.fRec18_perm[j39 as usize] =
                    fRec18_tmp[(i32::wrapping_add(vsize, j39)) as usize];
            }
            /* Recursive loop 20 */
            /* Pre code */
            self.fYec23_idx = (i32::wrapping_add(self.fYec23_idx, self.fYec23_idx_save)) & 4095;
            for j52 in 0..4 {
                fRec25_tmp[j52 as usize] = self.fRec25_perm[j52 as usize];
            }
            /* Compute code */
            for i in 0..output0.len() as i32 {
                self.fYec23[((i32::wrapping_add(i, self.fYec23_idx)) & 4095) as usize] = fZec0
                    [i as usize]
                    + fSlow2
                        * (fRec25_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]
                            + fRec25_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize]);
                fRec25_tmp[(i32::wrapping_add(4, i)) as usize] = self.fYec23[((i32::wrapping_sub(
                    i32::wrapping_add(i, self.fYec23_idx),
                    iSlow47,
                )) & 4095)
                    as usize];
            }
            /* Post code */
            self.fYec23_idx_save = vsize;
            for j53 in 0..4 {
                self.fRec25_perm[j53 as usize] =
                    fRec25_tmp[(i32::wrapping_add(vsize, j53)) as usize];
            }
            /* Recursive loop 21 */
            /* Pre code */
            self.fYec20_idx = (i32::wrapping_add(self.fYec20_idx, self.fYec20_idx_save)) & 4095;
            for j46 in 0..4 {
                fRec22_tmp[j46 as usize] = self.fRec22_perm[j46 as usize];
            }
            /* Compute code */
            for i in 0..output0.len() as i32 {
                self.fYec20[((i32::wrapping_add(i, self.fYec20_idx)) & 4095) as usize] = fZec0
                    [i as usize]
                    + fSlow2
                        * (fRec22_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]
                            + fRec22_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize]);
                fRec22_tmp[(i32::wrapping_add(4, i)) as usize] = self.fYec20[((i32::wrapping_sub(
                    i32::wrapping_add(i, self.fYec20_idx),
                    iSlow44,
                )) & 4095)
                    as usize];
            }
            /* Post code */
            self.fYec20_idx_save = vsize;
            for j47 in 0..4 {
                self.fRec22_perm[j47 as usize] =
                    fRec22_tmp[(i32::wrapping_add(vsize, j47)) as usize];
            }
            /* Recursive loop 22 */
            /* Pre code */
            self.fYec13_idx = (i32::wrapping_add(self.fYec13_idx, self.fYec13_idx_save)) & 8191;
            for j32 in 0..4 {
                fRec15_tmp[j32 as usize] = self.fRec15_perm[j32 as usize];
            }
            /* Compute code */
            for i in 0..output0.len() as i32 {
                self.fYec13[((i32::wrapping_add(i, self.fYec13_idx)) & 8191) as usize] = fZec0
                    [i as usize]
                    + fSlow2
                        * (fRec15_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]
                            + fRec15_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize]);
                fRec15_tmp[(i32::wrapping_add(4, i)) as usize] = self.fYec13[((i32::wrapping_sub(
                    i32::wrapping_add(i, self.fYec13_idx),
                    iSlow19,
                )) & 8191)
                    as usize];
            }
            /* Post code */
            self.fYec13_idx_save = vsize;
            for j33 in 0..4 {
                self.fRec15_perm[j33 as usize] =
                    fRec15_tmp[(i32::wrapping_add(vsize, j33)) as usize];
            }
            /* Recursive loop 23 */
            /* Pre code */
            self.fYec22_idx = (i32::wrapping_add(self.fYec22_idx, self.fYec22_idx_save)) & 2047;
            for j50 in 0..4 {
                fRec24_tmp[j50 as usize] = self.fRec24_perm[j50 as usize];
            }
            /* Compute code */
            for i in 0..output0.len() as i32 {
                self.fYec22[((i32::wrapping_add(i, self.fYec22_idx)) & 2047) as usize] = fZec0
                    [i as usize]
                    + fSlow2
                        * (fRec24_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]
                            + fRec24_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize]);
                fRec24_tmp[(i32::wrapping_add(4, i)) as usize] = self.fYec22[((i32::wrapping_sub(
                    i32::wrapping_add(i, self.fYec22_idx),
                    iSlow46,
                )) & 2047)
                    as usize];
            }
            /* Post code */
            self.fYec22_idx_save = vsize;
            for j51 in 0..4 {
                self.fRec24_perm[j51 as usize] =
                    fRec24_tmp[(i32::wrapping_add(vsize, j51)) as usize];
            }
            /* Recursive loop 24 */
            /* Pre code */
            self.fYec14_idx = (i32::wrapping_add(self.fYec14_idx, self.fYec14_idx_save)) & 8191;
            for j34 in 0..4 {
                fRec16_tmp[j34 as usize] = self.fRec16_perm[j34 as usize];
            }
            /* Compute code */
            for i in 0..output0.len() as i32 {
                self.fYec14[((i32::wrapping_add(i, self.fYec14_idx)) & 8191) as usize] = fZec0
                    [i as usize]
                    + fSlow2
                        * (fRec16_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]
                            + fRec16_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize]);
                fRec16_tmp[(i32::wrapping_add(4, i)) as usize] = self.fYec14[((i32::wrapping_sub(
                    i32::wrapping_add(i, self.fYec14_idx),
                    iSlow20,
                )) & 8191)
                    as usize];
            }
            /* Post code */
            self.fYec14_idx_save = vsize;
            for j35 in 0..4 {
                self.fRec16_perm[j35 as usize] =
                    fRec16_tmp[(i32::wrapping_add(vsize, j35)) as usize];
            }
            /* Recursive loop 25 */
            /* Pre code */
            self.fYec15_idx = (i32::wrapping_add(self.fYec15_idx, self.fYec15_idx_save)) & 8191;
            for j36 in 0..4 {
                fRec17_tmp[j36 as usize] = self.fRec17_perm[j36 as usize];
            }
            /* Compute code */
            for i in 0..output0.len() as i32 {
                self.fYec15[((i32::wrapping_add(i, self.fYec15_idx)) & 8191) as usize] = fZec0
                    [i as usize]
                    + fSlow2
                        * (fRec17_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]
                            + fRec17_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize]);
                fRec17_tmp[(i32::wrapping_add(4, i)) as usize] = self.fYec15[((i32::wrapping_sub(
                    i32::wrapping_add(i, self.fYec15_idx),
                    iSlow21,
                )) & 8191)
                    as usize];
            }
            /* Post code */
            self.fYec15_idx_save = vsize;
            for j37 in 0..4 {
                self.fRec17_perm[j37 as usize] =
                    fRec17_tmp[(i32::wrapping_add(vsize, j37)) as usize];
            }
            /* Recursive loop 26 */
            /* Pre code */
            self.fYec21_idx = (i32::wrapping_add(self.fYec21_idx, self.fYec21_idx_save)) & 8191;
            for j48 in 0..4 {
                fRec23_tmp[j48 as usize] = self.fRec23_perm[j48 as usize];
            }
            /* Compute code */
            for i in 0..output0.len() as i32 {
                self.fYec21[((i32::wrapping_add(i, self.fYec21_idx)) & 8191) as usize] = fZec0
                    [i as usize]
                    + fSlow2
                        * (fRec23_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]
                            + fRec23_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize]);
                fRec23_tmp[(i32::wrapping_add(4, i)) as usize] = self.fYec21[((i32::wrapping_sub(
                    i32::wrapping_add(i, self.fYec21_idx),
                    iSlow45,
                )) & 8191)
                    as usize];
            }
            /* Post code */
            self.fYec21_idx_save = vsize;
            for j49 in 0..4 {
                self.fRec23_perm[j49 as usize] =
                    fRec23_tmp[(i32::wrapping_add(vsize, j49)) as usize];
            }
            /* Recursive loop 27 */
            /* Pre code */
            self.fYec19_idx = (i32::wrapping_add(self.fYec19_idx, self.fYec19_idx_save)) & 8191;
            for j44 in 0..4 {
                fRec21_tmp[j44 as usize] = self.fRec21_perm[j44 as usize];
            }
            /* Compute code */
            for i in 0..output0.len() as i32 {
                self.fYec19[((i32::wrapping_add(i, self.fYec19_idx)) & 8191) as usize] = fZec0
                    [i as usize]
                    + fSlow2
                        * (fRec21_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]
                            + fRec21_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize]);
                fRec21_tmp[(i32::wrapping_add(4, i)) as usize] = self.fYec19[((i32::wrapping_sub(
                    i32::wrapping_add(i, self.fYec19_idx),
                    iSlow43,
                )) & 8191)
                    as usize];
            }
            /* Post code */
            self.fYec19_idx_save = vsize;
            for j45 in 0..4 {
                self.fRec21_perm[j45 as usize] =
                    fRec21_tmp[(i32::wrapping_add(vsize, j45)) as usize];
            }
            /* Recursive loop 28 */
            /* Pre code */
            self.fYec17_idx = (i32::wrapping_add(self.fYec17_idx, self.fYec17_idx_save)) & 8191;
            for j40 in 0..4 {
                fRec19_tmp[j40 as usize] = self.fRec19_perm[j40 as usize];
            }
            /* Compute code */
            for i in 0..output0.len() as i32 {
                self.fYec17[((i32::wrapping_add(i, self.fYec17_idx)) & 8191) as usize] = fZec0
                    [i as usize]
                    + fSlow2
                        * (fRec19_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]
                            + fRec19_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize]);
                fRec19_tmp[(i32::wrapping_add(4, i)) as usize] = self.fYec17[((i32::wrapping_sub(
                    i32::wrapping_add(i, self.fYec17_idx),
                    iSlow41,
                )) & 8191)
                    as usize];
            }
            /* Post code */
            self.fYec17_idx_save = vsize;
            for j41 in 0..4 {
                self.fRec19_perm[j41 as usize] =
                    fRec19_tmp[(i32::wrapping_add(vsize, j41)) as usize];
            }
            /* Recursive loop 29 */
            /* Pre code */
            self.fYec26_idx = (i32::wrapping_add(self.fYec26_idx, self.fYec26_idx_save)) & 8191;
            for j58 in 0..4 {
                fRec28_tmp[j58 as usize] = self.fRec28_perm[j58 as usize];
            }
            /* Compute code */
            for i in 0..output0.len() as i32 {
                self.fYec26[((i32::wrapping_add(i, self.fYec26_idx)) & 8191) as usize] = fZec0
                    [i as usize]
                    + fSlow2
                        * (fRec28_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]
                            + fRec28_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize]);
                fRec28_tmp[(i32::wrapping_add(4, i)) as usize] = self.fYec26[((i32::wrapping_sub(
                    i32::wrapping_add(i, self.fYec26_idx),
                    iSlow50,
                )) & 8191)
                    as usize];
            }
            /* Post code */
            self.fYec26_idx_save = vsize;
            for j59 in 0..4 {
                self.fRec28_perm[j59 as usize] =
                    fRec28_tmp[(i32::wrapping_add(vsize, j59)) as usize];
            }
            /* Recursive loop 30 */
            /* Pre code */
            self.fYec25_idx = (i32::wrapping_add(self.fYec25_idx, self.fYec25_idx_save)) & 8191;
            for j56 in 0..4 {
                fRec27_tmp[j56 as usize] = self.fRec27_perm[j56 as usize];
            }
            /* Compute code */
            for i in 0..output0.len() as i32 {
                self.fYec25[((i32::wrapping_add(i, self.fYec25_idx)) & 8191) as usize] = fZec0
                    [i as usize]
                    + fSlow2
                        * (fRec27_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]
                            + fRec27_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize]);
                fRec27_tmp[(i32::wrapping_add(4, i)) as usize] = self.fYec25[((i32::wrapping_sub(
                    i32::wrapping_add(i, self.fYec25_idx),
                    iSlow49,
                )) & 8191)
                    as usize];
            }
            /* Post code */
            self.fYec25_idx_save = vsize;
            for j57 in 0..4 {
                self.fRec27_perm[j57 as usize] =
                    fRec27_tmp[(i32::wrapping_add(vsize, j57)) as usize];
            }
            /* Recursive loop 31 */
            /* Pre code */
            self.fYec28_idx = (i32::wrapping_add(self.fYec28_idx, self.fYec28_idx_save)) & 8191;
            for j62 in 0..4 {
                fRec30_tmp[j62 as usize] = self.fRec30_perm[j62 as usize];
            }
            /* Compute code */
            for i in 0..output0.len() as i32 {
                self.fYec28[((i32::wrapping_add(i, self.fYec28_idx)) & 8191) as usize] = fZec0
                    [i as usize]
                    + fSlow2
                        * (fRec30_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]
                            + fRec30_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize]);
                fRec30_tmp[(i32::wrapping_add(4, i)) as usize] = self.fYec28[((i32::wrapping_sub(
                    i32::wrapping_add(i, self.fYec28_idx),
                    iSlow52,
                )) & 8191)
                    as usize];
            }
            /* Post code */
            self.fYec28_idx_save = vsize;
            for j63 in 0..4 {
                self.fRec30_perm[j63 as usize] =
                    fRec30_tmp[(i32::wrapping_add(vsize, j63)) as usize];
            }
            /* Recursive loop 32 */
            /* Pre code */
            self.fYec31_idx = (i32::wrapping_add(self.fYec31_idx, self.fYec31_idx_save)) & 8191;
            for j68 in 0..4 {
                fRec33_tmp[j68 as usize] = self.fRec33_perm[j68 as usize];
            }
            /* Compute code */
            for i in 0..output0.len() as i32 {
                self.fYec31[((i32::wrapping_add(i, self.fYec31_idx)) & 8191) as usize] = fZec0
                    [i as usize]
                    + fSlow2
                        * (fRec33_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]
                            + fRec33_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize]);
                fRec33_tmp[(i32::wrapping_add(4, i)) as usize] = self.fYec31[((i32::wrapping_sub(
                    i32::wrapping_add(i, self.fYec31_idx),
                    iSlow55,
                )) & 8191)
                    as usize];
            }
            /* Post code */
            self.fYec31_idx_save = vsize;
            for j69 in 0..4 {
                self.fRec33_perm[j69 as usize] =
                    fRec33_tmp[(i32::wrapping_add(vsize, j69)) as usize];
            }
            /* Recursive loop 33 */
            /* Pre code */
            self.fYec27_idx = (i32::wrapping_add(self.fYec27_idx, self.fYec27_idx_save)) & 8191;
            for j60 in 0..4 {
                fRec29_tmp[j60 as usize] = self.fRec29_perm[j60 as usize];
            }
            /* Compute code */
            for i in 0..output0.len() as i32 {
                self.fYec27[((i32::wrapping_add(i, self.fYec27_idx)) & 8191) as usize] = fZec0
                    [i as usize]
                    + fSlow2
                        * (fRec29_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]
                            + fRec29_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize]);
                fRec29_tmp[(i32::wrapping_add(4, i)) as usize] = self.fYec27[((i32::wrapping_sub(
                    i32::wrapping_add(i, self.fYec27_idx),
                    iSlow51,
                )) & 8191)
                    as usize];
            }
            /* Post code */
            self.fYec27_idx_save = vsize;
            for j61 in 0..4 {
                self.fRec29_perm[j61 as usize] =
                    fRec29_tmp[(i32::wrapping_add(vsize, j61)) as usize];
            }
            /* Recursive loop 34 */
            /* Pre code */
            self.fYec30_idx = (i32::wrapping_add(self.fYec30_idx, self.fYec30_idx_save)) & 8191;
            for j66 in 0..4 {
                fRec32_tmp[j66 as usize] = self.fRec32_perm[j66 as usize];
            }
            /* Compute code */
            for i in 0..output0.len() as i32 {
                self.fYec30[((i32::wrapping_add(i, self.fYec30_idx)) & 8191) as usize] = fZec0
                    [i as usize]
                    + fSlow2
                        * (fRec32_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]
                            + fRec32_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize]);
                fRec32_tmp[(i32::wrapping_add(4, i)) as usize] = self.fYec30[((i32::wrapping_sub(
                    i32::wrapping_add(i, self.fYec30_idx),
                    iSlow54,
                )) & 8191)
                    as usize];
            }
            /* Post code */
            self.fYec30_idx_save = vsize;
            for j67 in 0..4 {
                self.fRec32_perm[j67 as usize] =
                    fRec32_tmp[(i32::wrapping_add(vsize, j67)) as usize];
            }
            /* Recursive loop 35 */
            /* Pre code */
            self.fYec29_idx = (i32::wrapping_add(self.fYec29_idx, self.fYec29_idx_save)) & 8191;
            for j64 in 0..4 {
                fRec31_tmp[j64 as usize] = self.fRec31_perm[j64 as usize];
            }
            /* Compute code */
            for i in 0..output0.len() as i32 {
                self.fYec29[((i32::wrapping_add(i, self.fYec29_idx)) & 8191) as usize] = fZec0
                    [i as usize]
                    + fSlow2
                        * (fRec31_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]
                            + fRec31_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize]);
                fRec31_tmp[(i32::wrapping_add(4, i)) as usize] = self.fYec29[((i32::wrapping_sub(
                    i32::wrapping_add(i, self.fYec29_idx),
                    iSlow53,
                )) & 8191)
                    as usize];
            }
            /* Post code */
            self.fYec29_idx_save = vsize;
            for j65 in 0..4 {
                self.fRec31_perm[j65 as usize] =
                    fRec31_tmp[(i32::wrapping_add(vsize, j65)) as usize];
            }
            /* Vectorizable loop 36 */
            /* Compute code */
            for i in 0..output0.len() as i32 {
                output0[i as usize] = fSlow39
                    * (fSlow38 * fRec0_tmp[(i32::wrapping_add(4, i)) as usize]
                        + fSlow37 * fRec3_tmp[(i32::wrapping_add(4, i)) as usize]
                        + fSlow36 * fRec4_tmp[(i32::wrapping_add(4, i)) as usize]
                        + fSlow35 * fRec5_tmp[(i32::wrapping_add(4, i)) as usize]
                        + fSlow34 * fRec6_tmp[(i32::wrapping_add(4, i)) as usize]
                        + fSlow33 * fRec7_tmp[(i32::wrapping_add(4, i)) as usize]
                        + fSlow32 * fRec8_tmp[(i32::wrapping_add(4, i)) as usize]
                        + fSlow31 * fRec9_tmp[(i32::wrapping_add(4, i)) as usize]
                        + fSlow30 * fRec10_tmp[(i32::wrapping_add(4, i)) as usize]
                        + fSlow29 * fRec11_tmp[(i32::wrapping_add(4, i)) as usize]
                        + fSlow28 * fRec12_tmp[(i32::wrapping_add(4, i)) as usize]
                        + fSlow27 * fRec13_tmp[(i32::wrapping_add(4, i)) as usize]
                        + fSlow26 * fRec14_tmp[(i32::wrapping_add(4, i)) as usize]
                        + fSlow25 * fRec15_tmp[(i32::wrapping_add(4, i)) as usize]
                        + fSlow24 * fRec16_tmp[(i32::wrapping_add(4, i)) as usize]
                        + fSlow23 * fRec17_tmp[(i32::wrapping_add(4, i)) as usize]);
            }
            /* Vectorizable loop 37 */
            /* Compute code */
            for i in 0..output0.len() as i32 {
                output1[i as usize] = fSlow39
                    * (fSlow71 * fRec18_tmp[(i32::wrapping_add(4, i)) as usize]
                        + fSlow70 * fRec19_tmp[(i32::wrapping_add(4, i)) as usize]
                        + fSlow69 * fRec20_tmp[(i32::wrapping_add(4, i)) as usize]
                        + fSlow68 * fRec21_tmp[(i32::wrapping_add(4, i)) as usize]
                        + fSlow67 * fRec22_tmp[(i32::wrapping_add(4, i)) as usize]
                        + fSlow66 * fRec23_tmp[(i32::wrapping_add(4, i)) as usize]
                        + fSlow65 * fRec24_tmp[(i32::wrapping_add(4, i)) as usize]
                        + fSlow64 * fRec25_tmp[(i32::wrapping_add(4, i)) as usize]
                        + fSlow63 * fRec26_tmp[(i32::wrapping_add(4, i)) as usize]
                        + fSlow62 * fRec27_tmp[(i32::wrapping_add(4, i)) as usize]
                        + fSlow61 * fRec28_tmp[(i32::wrapping_add(4, i)) as usize]
                        + fSlow60 * fRec29_tmp[(i32::wrapping_add(4, i)) as usize]
                        + fSlow59 * fRec30_tmp[(i32::wrapping_add(4, i)) as usize]
                        + fSlow58 * fRec31_tmp[(i32::wrapping_add(4, i)) as usize]
                        + fSlow57 * fRec32_tmp[(i32::wrapping_add(4, i)) as usize]
                        + fSlow56 * fRec33_tmp[(i32::wrapping_add(4, i)) as usize]);
            }
        }
    }
}

pub fn criterion_benchmark(c: &mut Criterion) {
    const SIZE: usize = 4095;
    let mut group = c.benchmark_group("karplus32");
    group.throughput(Throughput::Elements(SIZE as u64));
    let mut dsp = default_boxed::DefaultBoxed::default_boxed();
    let mut buf0 = [0.0; SIZE];
    let mut buf1 = [0.0; SIZE];

    group.bench_function("scalar", |b| {
        b.iter(|| {
            black_box(mydsp::compute(
                black_box(&mut dsp),
                black_box(SIZE as i32),
                black_box(&[]),
                black_box(&mut [&mut buf0, &mut buf1]),
            ))
        })
    });

    let mut dsp_vec = default_boxed::DefaultBoxed::default_boxed();

    group.bench_with_input(BenchmarkId::new("vec", 4), &4, |b, &s| {
        b.iter(|| {
            black_box(mydsp_vec::compute_4(
                black_box(&mut dsp_vec),
                black_box(SIZE as i32),
                black_box(&[]),
                black_box(&mut [&mut buf0, &mut buf1]),
            ));
        })
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
