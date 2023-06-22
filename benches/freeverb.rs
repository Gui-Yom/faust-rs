//! https://godbolt.org/z/a3Yj6rbPq

use std::hint::black_box;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

type F32 = f32;

#[cfg_attr(feature = "default-boxed", derive(default_boxed::DefaultBoxed))]
#[cfg_attr(feature = "reprc", repr(C))]
pub struct mydsp {
    fHslider0: F32,
    fHslider1: F32,
    fRec9: [F32; 2],
    fHslider2: F32,
    IOTA0: i32,
    fVec0: [F32; 2048],
    fRec8: [F32; 2],
    fRec11: [F32; 2],
    fVec1: [F32; 2048],
    fRec10: [F32; 2],
    fRec13: [F32; 2],
    fVec2: [F32; 2048],
    fRec12: [F32; 2],
    fRec15: [F32; 2],
    fVec3: [F32; 2048],
    fRec14: [F32; 2],
    fRec17: [F32; 2],
    fVec4: [F32; 2048],
    fRec16: [F32; 2],
    fRec19: [F32; 2],
    fVec5: [F32; 2048],
    fRec18: [F32; 2],
    fRec21: [F32; 2],
    fVec6: [F32; 2048],
    fRec20: [F32; 2],
    fRec23: [F32; 2],
    fVec7: [F32; 2048],
    fRec22: [F32; 2],
    fVec8: [F32; 1024],
    fRec6: [F32; 2],
    fVec9: [F32; 512],
    fRec4: [F32; 2],
    fVec10: [F32; 512],
    fRec2: [F32; 2],
    fVec11: [F32; 256],
    fRec0: [F32; 2],
    fRec33: [F32; 2],
    fVec12: [F32; 2048],
    fRec32: [F32; 2],
    fRec35: [F32; 2],
    fVec13: [F32; 2048],
    fRec34: [F32; 2],
    fRec37: [F32; 2],
    fVec14: [F32; 2048],
    fRec36: [F32; 2],
    fRec39: [F32; 2],
    fVec15: [F32; 2048],
    fRec38: [F32; 2],
    fRec41: [F32; 2],
    fVec16: [F32; 2048],
    fRec40: [F32; 2],
    fRec43: [F32; 2],
    fVec17: [F32; 2048],
    fRec42: [F32; 2],
    fRec45: [F32; 2],
    fVec18: [F32; 2048],
    fRec44: [F32; 2],
    fRec47: [F32; 2],
    fVec19: [F32; 2048],
    fRec46: [F32; 2],
    fVec20: [F32; 1024],
    fRec30: [F32; 2],
    fVec21: [F32; 512],
    fRec28: [F32; 2],
    fVec22: [F32; 512],
    fRec26: [F32; 2],
    fVec23: [F32; 256],
    fRec24: [F32; 2],
    fSampleRate: i32,
}

impl mydsp {
    fn new() -> Self {
        Self {
            fHslider0: 0.0,
            fHslider1: 0.0,
            fRec9: [0.0; 2],
            fHslider2: 0.0,
            IOTA0: 0,
            fVec0: [0.0; 2048],
            fRec8: [0.0; 2],
            fRec11: [0.0; 2],
            fVec1: [0.0; 2048],
            fRec10: [0.0; 2],
            fRec13: [0.0; 2],
            fVec2: [0.0; 2048],
            fRec12: [0.0; 2],
            fRec15: [0.0; 2],
            fVec3: [0.0; 2048],
            fRec14: [0.0; 2],
            fRec17: [0.0; 2],
            fVec4: [0.0; 2048],
            fRec16: [0.0; 2],
            fRec19: [0.0; 2],
            fVec5: [0.0; 2048],
            fRec18: [0.0; 2],
            fRec21: [0.0; 2],
            fVec6: [0.0; 2048],
            fRec20: [0.0; 2],
            fRec23: [0.0; 2],
            fVec7: [0.0; 2048],
            fRec22: [0.0; 2],
            fVec8: [0.0; 1024],
            fRec6: [0.0; 2],
            fVec9: [0.0; 512],
            fRec4: [0.0; 2],
            fVec10: [0.0; 512],
            fRec2: [0.0; 2],
            fVec11: [0.0; 256],
            fRec0: [0.0; 2],
            fRec33: [0.0; 2],
            fVec12: [0.0; 2048],
            fRec32: [0.0; 2],
            fRec35: [0.0; 2],
            fVec13: [0.0; 2048],
            fRec34: [0.0; 2],
            fRec37: [0.0; 2],
            fVec14: [0.0; 2048],
            fRec36: [0.0; 2],
            fRec39: [0.0; 2],
            fVec15: [0.0; 2048],
            fRec38: [0.0; 2],
            fRec41: [0.0; 2],
            fVec16: [0.0; 2048],
            fRec40: [0.0; 2],
            fRec43: [0.0; 2],
            fVec17: [0.0; 2048],
            fRec42: [0.0; 2],
            fRec45: [0.0; 2],
            fVec18: [0.0; 2048],
            fRec44: [0.0; 2],
            fRec47: [0.0; 2],
            fVec19: [0.0; 2048],
            fRec46: [0.0; 2],
            fVec20: [0.0; 1024],
            fRec30: [0.0; 2],
            fVec21: [0.0; 512],
            fRec28: [0.0; 2],
            fVec22: [0.0; 512],
            fRec26: [0.0; 2],
            fVec23: [0.0; 256],
            fRec24: [0.0; 2],
            fSampleRate: 0,
        }
    }

    #[allow(non_snake_case)]
    #[inline(never)]
    fn compute(&mut self, count: i32, inputs: &[&[F32]], outputs: &mut [&mut [F32]]) {
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
        let mut fSlow0: F32 = self.fHslider0;
        let mut fSlow1: F32 = 1.0 - fSlow0;
        let mut fSlow2: F32 = 0.4 * self.fHslider1;
        let mut fSlow3: F32 = 1.0 - fSlow2;
        let mut fSlow4: F32 = 0.28 * self.fHslider2 + 0.7;
        let zipped_iterators = inputs0.zip(inputs1).zip(outputs0).zip(outputs1);
        for (((input0, input1), output0), output1) in zipped_iterators {
            let mut fTemp0: F32 = *input0;
            self.fRec9[0] = fSlow2 * self.fRec9[1] + fSlow3 * self.fRec8[1];
            let mut fTemp1: F32 = *input1;
            let mut fTemp2: F32 = 0.015 * (fTemp0 + fTemp1);
            self.fVec0[(self.IOTA0 & 2047) as usize] = fTemp2 + fSlow4 * self.fRec9[0];
            self.fRec8[0] = self.fVec0[((i32::wrapping_sub(self.IOTA0, 1356)) & 2047) as usize];
            self.fRec11[0] = fSlow2 * self.fRec11[1] + fSlow3 * self.fRec10[1];
            self.fVec1[(self.IOTA0 & 2047) as usize] = fTemp2 + fSlow4 * self.fRec11[0];
            self.fRec10[0] = self.fVec1[((i32::wrapping_sub(self.IOTA0, 1188)) & 2047) as usize];
            self.fRec13[0] = fSlow2 * self.fRec13[1] + fSlow3 * self.fRec12[1];
            self.fVec2[(self.IOTA0 & 2047) as usize] = fSlow4 * self.fRec13[0] + fTemp2;
            self.fRec12[0] = self.fVec2[((i32::wrapping_sub(self.IOTA0, 1116)) & 2047) as usize];
            self.fRec15[0] = fSlow2 * self.fRec15[1] + fSlow3 * self.fRec14[1];
            self.fVec3[(self.IOTA0 & 2047) as usize] = fTemp2 + fSlow4 * self.fRec15[0];
            self.fRec14[0] = self.fVec3[((i32::wrapping_sub(self.IOTA0, 1277)) & 2047) as usize];
            self.fRec17[0] = fSlow2 * self.fRec17[1] + fSlow3 * self.fRec16[1];
            self.fVec4[(self.IOTA0 & 2047) as usize] = fTemp2 + fSlow4 * self.fRec17[0];
            self.fRec16[0] = self.fVec4[((i32::wrapping_sub(self.IOTA0, 1491)) & 2047) as usize];
            self.fRec19[0] = fSlow2 * self.fRec19[1] + fSlow3 * self.fRec18[1];
            self.fVec5[(self.IOTA0 & 2047) as usize] = fTemp2 + fSlow4 * self.fRec19[0];
            self.fRec18[0] = self.fVec5[((i32::wrapping_sub(self.IOTA0, 1422)) & 2047) as usize];
            self.fRec21[0] = fSlow2 * self.fRec21[1] + fSlow3 * self.fRec20[1];
            self.fVec6[(self.IOTA0 & 2047) as usize] = fTemp2 + fSlow4 * self.fRec21[0];
            self.fRec20[0] = self.fVec6[((i32::wrapping_sub(self.IOTA0, 1617)) & 2047) as usize];
            self.fRec23[0] = fSlow2 * self.fRec23[1] + fSlow3 * self.fRec22[1];
            self.fVec7[(self.IOTA0 & 2047) as usize] = fTemp2 + fSlow4 * self.fRec23[0];
            self.fRec22[0] = self.fVec7[((i32::wrapping_sub(self.IOTA0, 1557)) & 2047) as usize];
            let mut fTemp3: F32 = self.fRec22[0]
                + self.fRec20[0]
                + self.fRec18[0]
                + self.fRec16[0]
                + self.fRec14[0]
                + self.fRec12[0]
                + self.fRec10[0]
                + self.fRec8[0];
            self.fVec8[(self.IOTA0 & 1023) as usize] = 0.5 * self.fRec6[1] + fTemp3;
            self.fRec6[0] = self.fVec8[((i32::wrapping_sub(self.IOTA0, 556)) & 1023) as usize];
            let mut fRec7: F32 = self.fRec6[1] - fTemp3;
            self.fVec9[(self.IOTA0 & 511) as usize] = fRec7 + 0.5 * self.fRec4[1];
            self.fRec4[0] = self.fVec9[((i32::wrapping_sub(self.IOTA0, 441)) & 511) as usize];
            let mut fRec5: F32 = self.fRec4[1] - fRec7;
            self.fVec10[(self.IOTA0 & 511) as usize] = fRec5 + 0.5 * self.fRec2[1];
            self.fRec2[0] = self.fVec10[((i32::wrapping_sub(self.IOTA0, 341)) & 511) as usize];
            let mut fRec3: F32 = self.fRec2[1] - fRec5;
            self.fVec11[(self.IOTA0 & 255) as usize] = fRec3 + 0.5 * self.fRec0[1];
            self.fRec0[0] = self.fVec11[((i32::wrapping_sub(self.IOTA0, 225)) & 255) as usize];
            let mut fRec1: F32 = self.fRec0[1] - fRec3;
            *output0 = fSlow0 * fRec1 + fSlow1 * fTemp0;
            self.fRec33[0] = fSlow2 * self.fRec33[1] + fSlow3 * self.fRec32[1];
            self.fVec12[(self.IOTA0 & 2047) as usize] = fTemp2 + fSlow4 * self.fRec33[0];
            self.fRec32[0] = self.fVec12[((i32::wrapping_sub(self.IOTA0, 1640)) & 2047) as usize];
            self.fRec35[0] = fSlow2 * self.fRec35[1] + fSlow3 * self.fRec34[1];
            self.fVec13[(self.IOTA0 & 2047) as usize] = fTemp2 + fSlow4 * self.fRec35[0];
            self.fRec34[0] = self.fVec13[((i32::wrapping_sub(self.IOTA0, 1514)) & 2047) as usize];
            self.fRec37[0] = fSlow2 * self.fRec37[1] + fSlow3 * self.fRec36[1];
            self.fVec14[(self.IOTA0 & 2047) as usize] = fTemp2 + fSlow4 * self.fRec37[0];
            self.fRec36[0] = self.fVec14[((i32::wrapping_sub(self.IOTA0, 1445)) & 2047) as usize];
            self.fRec39[0] = fSlow2 * self.fRec39[1] + fSlow3 * self.fRec38[1];
            self.fVec15[(self.IOTA0 & 2047) as usize] = fTemp2 + fSlow4 * self.fRec39[0];
            self.fRec38[0] = self.fVec15[((i32::wrapping_sub(self.IOTA0, 1379)) & 2047) as usize];
            self.fRec41[0] = fSlow2 * self.fRec41[1] + fSlow3 * self.fRec40[1];
            self.fVec16[(self.IOTA0 & 2047) as usize] = fTemp2 + fSlow4 * self.fRec41[0];
            self.fRec40[0] = self.fVec16[((i32::wrapping_sub(self.IOTA0, 1300)) & 2047) as usize];
            self.fRec43[0] = fSlow2 * self.fRec43[1] + fSlow3 * self.fRec42[1];
            self.fVec17[(self.IOTA0 & 2047) as usize] = fTemp2 + fSlow4 * self.fRec43[0];
            self.fRec42[0] = self.fVec17[((i32::wrapping_sub(self.IOTA0, 1211)) & 2047) as usize];
            self.fRec45[0] = fSlow2 * self.fRec45[1] + fSlow3 * self.fRec44[1];
            self.fVec18[(self.IOTA0 & 2047) as usize] = fTemp2 + fSlow4 * self.fRec45[0];
            self.fRec44[0] = self.fVec18[((i32::wrapping_sub(self.IOTA0, 1139)) & 2047) as usize];
            self.fRec47[0] = fSlow2 * self.fRec47[1] + fSlow3 * self.fRec46[1];
            self.fVec19[(self.IOTA0 & 2047) as usize] = fTemp2 + fSlow4 * self.fRec47[0];
            self.fRec46[0] = self.fVec19[((i32::wrapping_sub(self.IOTA0, 1580)) & 2047) as usize];
            let mut fTemp4: F32 = self.fRec46[0]
                + self.fRec44[0]
                + self.fRec42[0]
                + self.fRec40[0]
                + self.fRec38[0]
                + self.fRec36[0]
                + self.fRec34[0]
                + self.fRec32[0];
            self.fVec20[(self.IOTA0 & 1023) as usize] = 0.5 * self.fRec30[1] + fTemp4;
            self.fRec30[0] = self.fVec20[((i32::wrapping_sub(self.IOTA0, 579)) & 1023) as usize];
            let mut fRec31: F32 = self.fRec30[1] - fTemp4;
            self.fVec21[(self.IOTA0 & 511) as usize] = fRec31 + 0.5 * self.fRec28[1];
            self.fRec28[0] = self.fVec21[((i32::wrapping_sub(self.IOTA0, 464)) & 511) as usize];
            let mut fRec29: F32 = self.fRec28[1] - fRec31;
            self.fVec22[(self.IOTA0 & 511) as usize] = fRec29 + 0.5 * self.fRec26[1];
            self.fRec26[0] = self.fVec22[((i32::wrapping_sub(self.IOTA0, 364)) & 511) as usize];
            let mut fRec27: F32 = self.fRec26[1] - fRec29;
            self.fVec23[(self.IOTA0 & 255) as usize] = fRec27 + 0.5 * self.fRec24[1];
            self.fRec24[0] = self.fVec23[((i32::wrapping_sub(self.IOTA0, 248)) & 255) as usize];
            let mut fRec25: F32 = self.fRec24[1] - fRec27;
            *output1 = fSlow0 * fRec25 + fSlow1 * fTemp1;
            self.fRec9[1] = self.fRec9[0];
            self.IOTA0 = i32::wrapping_add(self.IOTA0, 1);
            self.fRec8[1] = self.fRec8[0];
            self.fRec11[1] = self.fRec11[0];
            self.fRec10[1] = self.fRec10[0];
            self.fRec13[1] = self.fRec13[0];
            self.fRec12[1] = self.fRec12[0];
            self.fRec15[1] = self.fRec15[0];
            self.fRec14[1] = self.fRec14[0];
            self.fRec17[1] = self.fRec17[0];
            self.fRec16[1] = self.fRec16[0];
            self.fRec19[1] = self.fRec19[0];
            self.fRec18[1] = self.fRec18[0];
            self.fRec21[1] = self.fRec21[0];
            self.fRec20[1] = self.fRec20[0];
            self.fRec23[1] = self.fRec23[0];
            self.fRec22[1] = self.fRec22[0];
            self.fRec6[1] = self.fRec6[0];
            self.fRec4[1] = self.fRec4[0];
            self.fRec2[1] = self.fRec2[0];
            self.fRec0[1] = self.fRec0[0];
            self.fRec33[1] = self.fRec33[0];
            self.fRec32[1] = self.fRec32[0];
            self.fRec35[1] = self.fRec35[0];
            self.fRec34[1] = self.fRec34[0];
            self.fRec37[1] = self.fRec37[0];
            self.fRec36[1] = self.fRec36[0];
            self.fRec39[1] = self.fRec39[0];
            self.fRec38[1] = self.fRec38[0];
            self.fRec41[1] = self.fRec41[0];
            self.fRec40[1] = self.fRec40[0];
            self.fRec43[1] = self.fRec43[0];
            self.fRec42[1] = self.fRec42[0];
            self.fRec45[1] = self.fRec45[0];
            self.fRec44[1] = self.fRec44[0];
            self.fRec47[1] = self.fRec47[0];
            self.fRec46[1] = self.fRec46[0];
            self.fRec30[1] = self.fRec30[0];
            self.fRec28[1] = self.fRec28[0];
            self.fRec26[1] = self.fRec26[0];
            self.fRec24[1] = self.fRec24[0];
        }
    }
}

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[cfg_attr(feature = "default-boxed", derive(default_boxed::DefaultBoxed))]
#[cfg_attr(feature = "reprc", repr(C))]
pub struct mydsp_vec {
    fHslider0: F32,
    fRec9_perm: [F32; 4],
    fHslider1: F32,
    fYec0: [F32; 2048],
    fYec0_idx: i32,
    fYec0_idx_save: i32,
    fRec8_perm: [F32; 4],
    fRec11_perm: [F32; 4],
    fYec1: [F32; 2048],
    fYec1_idx: i32,
    fYec1_idx_save: i32,
    fRec10_perm: [F32; 4],
    fRec13_perm: [F32; 4],
    fYec2: [F32; 2048],
    fYec2_idx: i32,
    fYec2_idx_save: i32,
    fRec12_perm: [F32; 4],
    fRec15_perm: [F32; 4],
    fYec3: [F32; 2048],
    fYec3_idx: i32,
    fYec3_idx_save: i32,
    fRec14_perm: [F32; 4],
    fRec17_perm: [F32; 4],
    fYec4: [F32; 2048],
    fYec4_idx: i32,
    fYec4_idx_save: i32,
    fRec16_perm: [F32; 4],
    fRec19_perm: [F32; 4],
    fYec5: [F32; 2048],
    fYec5_idx: i32,
    fYec5_idx_save: i32,
    fRec18_perm: [F32; 4],
    fRec21_perm: [F32; 4],
    fYec6: [F32; 2048],
    fYec6_idx: i32,
    fYec6_idx_save: i32,
    fRec20_perm: [F32; 4],
    fRec23_perm: [F32; 4],
    fYec7: [F32; 2048],
    fYec7_idx: i32,
    fYec7_idx_save: i32,
    fRec22_perm: [F32; 4],
    fYec8: [F32; 1024],
    fYec8_idx: i32,
    fYec8_idx_save: i32,
    fRec6_perm: [F32; 4],
    fYec9: [F32; 512],
    fYec9_idx: i32,
    fYec9_idx_save: i32,
    fRec4_perm: [F32; 4],
    fYec10: [F32; 512],
    fYec10_idx: i32,
    fYec10_idx_save: i32,
    fRec2_perm: [F32; 4],
    fYec11: [F32; 256],
    fYec11_idx: i32,
    fYec11_idx_save: i32,
    fRec0_perm: [F32; 4],
    fHslider2: F32,
    fRec33_perm: [F32; 4],
    fYec12: [F32; 2048],
    fYec12_idx: i32,
    fYec12_idx_save: i32,
    fRec32_perm: [F32; 4],
    fRec35_perm: [F32; 4],
    fYec13: [F32; 2048],
    fYec13_idx: i32,
    fYec13_idx_save: i32,
    fRec34_perm: [F32; 4],
    fRec37_perm: [F32; 4],
    fYec14: [F32; 2048],
    fYec14_idx: i32,
    fYec14_idx_save: i32,
    fRec36_perm: [F32; 4],
    fRec39_perm: [F32; 4],
    fYec15: [F32; 2048],
    fYec15_idx: i32,
    fYec15_idx_save: i32,
    fRec38_perm: [F32; 4],
    fRec41_perm: [F32; 4],
    fYec16: [F32; 2048],
    fYec16_idx: i32,
    fYec16_idx_save: i32,
    fRec40_perm: [F32; 4],
    fRec43_perm: [F32; 4],
    fYec17: [F32; 2048],
    fYec17_idx: i32,
    fYec17_idx_save: i32,
    fRec42_perm: [F32; 4],
    fRec45_perm: [F32; 4],
    fYec18: [F32; 2048],
    fYec18_idx: i32,
    fYec18_idx_save: i32,
    fRec44_perm: [F32; 4],
    fRec47_perm: [F32; 4],
    fYec19: [F32; 2048],
    fYec19_idx: i32,
    fYec19_idx_save: i32,
    fRec46_perm: [F32; 4],
    fYec20: [F32; 1024],
    fYec20_idx: i32,
    fYec20_idx_save: i32,
    fRec30_perm: [F32; 4],
    fYec21: [F32; 512],
    fYec21_idx: i32,
    fYec21_idx_save: i32,
    fRec28_perm: [F32; 4],
    fYec22: [F32; 512],
    fYec22_idx: i32,
    fYec22_idx_save: i32,
    fRec26_perm: [F32; 4],
    fYec23: [F32; 256],
    fYec23_idx: i32,
    fYec23_idx_save: i32,
    fRec24_perm: [F32; 4],
    fSampleRate: i32,
}

impl mydsp_vec {
    fn new() -> Self {
        Self {
            fHslider0: 0.0,
            fRec9_perm: [0.0; 4],
            fHslider1: 0.0,
            fYec0: [0.0; 2048],
            fYec0_idx: 0,
            fYec0_idx_save: 0,
            fRec8_perm: [0.0; 4],
            fRec11_perm: [0.0; 4],
            fYec1: [0.0; 2048],
            fYec1_idx: 0,
            fYec1_idx_save: 0,
            fRec10_perm: [0.0; 4],
            fRec13_perm: [0.0; 4],
            fYec2: [0.0; 2048],
            fYec2_idx: 0,
            fYec2_idx_save: 0,
            fRec12_perm: [0.0; 4],
            fRec15_perm: [0.0; 4],
            fYec3: [0.0; 2048],
            fYec3_idx: 0,
            fYec3_idx_save: 0,
            fRec14_perm: [0.0; 4],
            fRec17_perm: [0.0; 4],
            fYec4: [0.0; 2048],
            fYec4_idx: 0,
            fYec4_idx_save: 0,
            fRec16_perm: [0.0; 4],
            fRec19_perm: [0.0; 4],
            fYec5: [0.0; 2048],
            fYec5_idx: 0,
            fYec5_idx_save: 0,
            fRec18_perm: [0.0; 4],
            fRec21_perm: [0.0; 4],
            fYec6: [0.0; 2048],
            fYec6_idx: 0,
            fYec6_idx_save: 0,
            fRec20_perm: [0.0; 4],
            fRec23_perm: [0.0; 4],
            fYec7: [0.0; 2048],
            fYec7_idx: 0,
            fYec7_idx_save: 0,
            fRec22_perm: [0.0; 4],
            fYec8: [0.0; 1024],
            fYec8_idx: 0,
            fYec8_idx_save: 0,
            fRec6_perm: [0.0; 4],
            fYec9: [0.0; 512],
            fYec9_idx: 0,
            fYec9_idx_save: 0,
            fRec4_perm: [0.0; 4],
            fYec10: [0.0; 512],
            fYec10_idx: 0,
            fYec10_idx_save: 0,
            fRec2_perm: [0.0; 4],
            fYec11: [0.0; 256],
            fYec11_idx: 0,
            fYec11_idx_save: 0,
            fRec0_perm: [0.0; 4],
            fHslider2: 0.0,
            fRec33_perm: [0.0; 4],
            fYec12: [0.0; 2048],
            fYec12_idx: 0,
            fYec12_idx_save: 0,
            fRec32_perm: [0.0; 4],
            fRec35_perm: [0.0; 4],
            fYec13: [0.0; 2048],
            fYec13_idx: 0,
            fYec13_idx_save: 0,
            fRec34_perm: [0.0; 4],
            fRec37_perm: [0.0; 4],
            fYec14: [0.0; 2048],
            fYec14_idx: 0,
            fYec14_idx_save: 0,
            fRec36_perm: [0.0; 4],
            fRec39_perm: [0.0; 4],
            fYec15: [0.0; 2048],
            fYec15_idx: 0,
            fYec15_idx_save: 0,
            fRec38_perm: [0.0; 4],
            fRec41_perm: [0.0; 4],
            fYec16: [0.0; 2048],
            fYec16_idx: 0,
            fYec16_idx_save: 0,
            fRec40_perm: [0.0; 4],
            fRec43_perm: [0.0; 4],
            fYec17: [0.0; 2048],
            fYec17_idx: 0,
            fYec17_idx_save: 0,
            fRec42_perm: [0.0; 4],
            fRec45_perm: [0.0; 4],
            fYec18: [0.0; 2048],
            fYec18_idx: 0,
            fYec18_idx_save: 0,
            fRec44_perm: [0.0; 4],
            fRec47_perm: [0.0; 4],
            fYec19: [0.0; 2048],
            fYec19_idx: 0,
            fYec19_idx_save: 0,
            fRec46_perm: [0.0; 4],
            fYec20: [0.0; 1024],
            fYec20_idx: 0,
            fYec20_idx_save: 0,
            fRec30_perm: [0.0; 4],
            fYec21: [0.0; 512],
            fYec21_idx: 0,
            fYec21_idx_save: 0,
            fRec28_perm: [0.0; 4],
            fYec22: [0.0; 512],
            fYec22_idx: 0,
            fYec22_idx_save: 0,
            fRec26_perm: [0.0; 4],
            fYec23: [0.0; 256],
            fYec23_idx: 0,
            fYec23_idx_save: 0,
            fRec24_perm: [0.0; 4],
            fSampleRate: 0,
        }
    }

    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    #[inline(never)]
    #[allow(non_snake_case)]
    #[allow(unused_mut)]
    fn compute_4(&mut self, count: i32, inputs: &[&[F32]], outputs: &mut [&mut [F32]]) {
        const vsize: i32 = 4;
        let (inputs0, inputs1) = if let [inputs0, inputs1, ..] = inputs {
            let inputs0 = inputs0[..count as usize].chunks(vsize as usize);
            let inputs1 = inputs1[..count as usize].chunks(vsize as usize);
            (inputs0, inputs1)
        } else {
            panic!("wrong number of inputs");
        };
        let (outputs0, outputs1) = if let [outputs0, outputs1, ..] = outputs {
            let outputs0 = outputs0[..count as usize].chunks_mut(vsize as usize);
            let outputs1 = outputs1[..count as usize].chunks_mut(vsize as usize);
            (outputs0, outputs1)
        } else {
            panic!("wrong number of outputs");
        };
        let mut fSlow0: F32 = 0.4 * self.fHslider0;
        let mut fSlow1: F32 = 1.0 - fSlow0;
        let mut fRec9_tmp: [F32; 8] = [0.0; 8];
        let mut fZec0: [F32; 4] = [0.0; 4];
        let mut fSlow2: F32 = 0.28 * self.fHslider1 + 0.7;
        let mut fRec8_tmp: [F32; 8] = [0.0; 8];
        let mut fRec11_tmp: [F32; 8] = [0.0; 8];
        let mut fRec10_tmp: [F32; 8] = [0.0; 8];
        let mut fRec13_tmp: [F32; 8] = [0.0; 8];
        let mut fRec12_tmp: [F32; 8] = [0.0; 8];
        let mut fRec15_tmp: [F32; 8] = [0.0; 8];
        let mut fRec14_tmp: [F32; 8] = [0.0; 8];
        let mut fRec17_tmp: [F32; 8] = [0.0; 8];
        let mut fRec16_tmp: [F32; 8] = [0.0; 8];
        let mut fRec19_tmp: [F32; 8] = [0.0; 8];
        let mut fRec18_tmp: [F32; 8] = [0.0; 8];
        let mut fRec21_tmp: [F32; 8] = [0.0; 8];
        let mut fRec20_tmp: [F32; 8] = [0.0; 8];
        let mut fRec23_tmp: [F32; 8] = [0.0; 8];
        let mut fRec22_tmp: [F32; 8] = [0.0; 8];
        let mut fZec1: [F32; 4] = [0.0; 4];
        let mut fRec6_tmp: [F32; 8] = [0.0; 8];
        let mut fRec7: [F32; 4] = [0.0; 4];
        let mut fRec4_tmp: [F32; 8] = [0.0; 8];
        let mut fRec5: [F32; 4] = [0.0; 4];
        let mut fRec2_tmp: [F32; 8] = [0.0; 8];
        let mut fRec3: [F32; 4] = [0.0; 4];
        let mut fRec0_tmp: [F32; 8] = [0.0; 8];
        let mut fRec1: [F32; 4] = [0.0; 4];
        let mut fSlow3: F32 = self.fHslider2;
        let mut fSlow4: F32 = 1.0 - fSlow3;
        let mut fRec33_tmp: [F32; 8] = [0.0; 8];
        let mut fRec32_tmp: [F32; 8] = [0.0; 8];
        let mut fRec35_tmp: [F32; 8] = [0.0; 8];
        let mut fRec34_tmp: [F32; 8] = [0.0; 8];
        let mut fRec37_tmp: [F32; 8] = [0.0; 8];
        let mut fRec36_tmp: [F32; 8] = [0.0; 8];
        let mut fRec39_tmp: [F32; 8] = [0.0; 8];
        let mut fRec38_tmp: [F32; 8] = [0.0; 8];
        let mut fRec41_tmp: [F32; 8] = [0.0; 8];
        let mut fRec40_tmp: [F32; 8] = [0.0; 8];
        let mut fRec43_tmp: [F32; 8] = [0.0; 8];
        let mut fRec42_tmp: [F32; 8] = [0.0; 8];
        let mut fRec45_tmp: [F32; 8] = [0.0; 8];
        let mut fRec44_tmp: [F32; 8] = [0.0; 8];
        let mut fRec47_tmp: [F32; 8] = [0.0; 8];
        let mut fRec46_tmp: [F32; 8] = [0.0; 8];
        let mut fZec2: [F32; 4] = [0.0; 4];
        let mut fRec30_tmp: [F32; 8] = [0.0; 8];
        let mut fRec31: [F32; 4] = [0.0; 4];
        let mut fRec28_tmp: [F32; 8] = [0.0; 8];
        let mut fRec29: [F32; 4] = [0.0; 4];
        let mut fRec26_tmp: [F32; 8] = [0.0; 8];
        let mut fRec27: [F32; 4] = [0.0; 4];
        let mut fRec24_tmp: [F32; 8] = [0.0; 8];
        let mut fRec25: [F32; 4] = [0.0; 4];
        /* Main loop */
        let zipped_iterators = inputs0.zip(inputs1).zip(outputs0).zip(outputs1);
        for (((input0, input1), output0), output1) in zipped_iterators {
            /* Vectorizable loop 0 */
            /* Compute code */
            for i in 0..vsize {
                fZec0[i as usize] = 0.015 * (input0[i as usize] + input1[i as usize]);
            }
            /* Recursive loop 1 */
            /* Pre code */
            for j16 in 0..4 {
                fRec17_tmp[j16 as usize] = self.fRec17_perm[j16 as usize];
            }
            self.fYec4_idx = (i32::wrapping_add(self.fYec4_idx, self.fYec4_idx_save)) & 2047;
            for j18 in 0..4 {
                fRec16_tmp[j18 as usize] = self.fRec16_perm[j18 as usize];
            }
            /* Compute code */
            for i in 0..vsize {
                fRec17_tmp[(i32::wrapping_add(4, i)) as usize] = fSlow0
                    * fRec17_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]
                    + fSlow1 * fRec16_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
                self.fYec4[((i32::wrapping_add(i, self.fYec4_idx)) & 2047) as usize] =
                    fZec0[i as usize] + fSlow2 * fRec17_tmp[(i32::wrapping_add(4, i)) as usize];
                fRec16_tmp[(i32::wrapping_add(4, i)) as usize] =
                    self.fYec4[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec4_idx), 1422))
                        & 2047) as usize];
            }
            /* Post code */
            self.fYec4_idx_save = vsize;
            for j17 in 0..4 {
                self.fRec17_perm[j17 as usize] =
                    fRec17_tmp[(i32::wrapping_add(vsize, j17)) as usize];
            }
            for j19 in 0..4 {
                self.fRec16_perm[j19 as usize] =
                    fRec16_tmp[(i32::wrapping_add(vsize, j19)) as usize];
            }
            /* Recursive loop 2 */
            /* Pre code */
            for j20 in 0..4 {
                fRec19_tmp[j20 as usize] = self.fRec19_perm[j20 as usize];
            }
            self.fYec5_idx = (i32::wrapping_add(self.fYec5_idx, self.fYec5_idx_save)) & 2047;
            for j22 in 0..4 {
                fRec18_tmp[j22 as usize] = self.fRec18_perm[j22 as usize];
            }
            /* Compute code */
            for i in 0..vsize {
                fRec19_tmp[(i32::wrapping_add(4, i)) as usize] = fSlow0
                    * fRec19_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]
                    + fSlow1 * fRec18_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
                self.fYec5[((i32::wrapping_add(i, self.fYec5_idx)) & 2047) as usize] =
                    fZec0[i as usize] + fSlow2 * fRec19_tmp[(i32::wrapping_add(4, i)) as usize];
                fRec18_tmp[(i32::wrapping_add(4, i)) as usize] =
                    self.fYec5[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec5_idx), 1491))
                        & 2047) as usize];
            }
            /* Post code */
            self.fYec5_idx_save = vsize;
            for j21 in 0..4 {
                self.fRec19_perm[j21 as usize] =
                    fRec19_tmp[(i32::wrapping_add(vsize, j21)) as usize];
            }
            for j23 in 0..4 {
                self.fRec18_perm[j23 as usize] =
                    fRec18_tmp[(i32::wrapping_add(vsize, j23)) as usize];
            }
            /* Recursive loop 3 */
            /* Pre code */
            for j24 in 0..4 {
                fRec21_tmp[j24 as usize] = self.fRec21_perm[j24 as usize];
            }
            self.fYec6_idx = (i32::wrapping_add(self.fYec6_idx, self.fYec6_idx_save)) & 2047;
            for j26 in 0..4 {
                fRec20_tmp[j26 as usize] = self.fRec20_perm[j26 as usize];
            }
            /* Compute code */
            for i in 0..vsize {
                fRec21_tmp[(i32::wrapping_add(4, i)) as usize] = fSlow0
                    * fRec21_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]
                    + fSlow1 * fRec20_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
                self.fYec6[((i32::wrapping_add(i, self.fYec6_idx)) & 2047) as usize] =
                    fZec0[i as usize] + fSlow2 * fRec21_tmp[(i32::wrapping_add(4, i)) as usize];
                fRec20_tmp[(i32::wrapping_add(4, i)) as usize] =
                    self.fYec6[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec6_idx), 1557))
                        & 2047) as usize];
            }
            /* Post code */
            self.fYec6_idx_save = vsize;
            for j25 in 0..4 {
                self.fRec21_perm[j25 as usize] =
                    fRec21_tmp[(i32::wrapping_add(vsize, j25)) as usize];
            }
            for j27 in 0..4 {
                self.fRec20_perm[j27 as usize] =
                    fRec20_tmp[(i32::wrapping_add(vsize, j27)) as usize];
            }
            /* Recursive loop 4 */
            /* Pre code */
            for j4 in 0..4 {
                fRec11_tmp[j4 as usize] = self.fRec11_perm[j4 as usize];
            }
            self.fYec1_idx = (i32::wrapping_add(self.fYec1_idx, self.fYec1_idx_save)) & 2047;
            for j6 in 0..4 {
                fRec10_tmp[j6 as usize] = self.fRec10_perm[j6 as usize];
            }
            /* Compute code */
            for i in 0..vsize {
                fRec11_tmp[(i32::wrapping_add(4, i)) as usize] = fSlow0
                    * fRec11_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]
                    + fSlow1 * fRec10_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
                self.fYec1[((i32::wrapping_add(i, self.fYec1_idx)) & 2047) as usize] =
                    fZec0[i as usize] + fSlow2 * fRec11_tmp[(i32::wrapping_add(4, i)) as usize];
                fRec10_tmp[(i32::wrapping_add(4, i)) as usize] =
                    self.fYec1[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec1_idx), 1188))
                        & 2047) as usize];
            }
            /* Post code */
            self.fYec1_idx_save = vsize;
            for j5 in 0..4 {
                self.fRec11_perm[j5 as usize] = fRec11_tmp[(i32::wrapping_add(vsize, j5)) as usize];
            }
            for j7 in 0..4 {
                self.fRec10_perm[j7 as usize] = fRec10_tmp[(i32::wrapping_add(vsize, j7)) as usize];
            }
            /* Recursive loop 5 */
            /* Pre code */
            for j8 in 0..4 {
                fRec13_tmp[j8 as usize] = self.fRec13_perm[j8 as usize];
            }
            self.fYec2_idx = (i32::wrapping_add(self.fYec2_idx, self.fYec2_idx_save)) & 2047;
            for j10 in 0..4 {
                fRec12_tmp[j10 as usize] = self.fRec12_perm[j10 as usize];
            }
            /* Compute code */
            for i in 0..vsize {
                fRec13_tmp[(i32::wrapping_add(4, i)) as usize] = fSlow0
                    * fRec13_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]
                    + fSlow1 * fRec12_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
                self.fYec2[((i32::wrapping_add(i, self.fYec2_idx)) & 2047) as usize] =
                    fZec0[i as usize] + fSlow2 * fRec13_tmp[(i32::wrapping_add(4, i)) as usize];
                fRec12_tmp[(i32::wrapping_add(4, i)) as usize] =
                    self.fYec2[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec2_idx), 1277))
                        & 2047) as usize];
            }
            /* Post code */
            self.fYec2_idx_save = vsize;
            for j9 in 0..4 {
                self.fRec13_perm[j9 as usize] = fRec13_tmp[(i32::wrapping_add(vsize, j9)) as usize];
            }
            for j11 in 0..4 {
                self.fRec12_perm[j11 as usize] =
                    fRec12_tmp[(i32::wrapping_add(vsize, j11)) as usize];
            }
            /* Recursive loop 6 */
            /* Pre code */
            for j0 in 0..4 {
                fRec9_tmp[j0 as usize] = self.fRec9_perm[j0 as usize];
            }
            self.fYec0_idx = (i32::wrapping_add(self.fYec0_idx, self.fYec0_idx_save)) & 2047;
            for j2 in 0..4 {
                fRec8_tmp[j2 as usize] = self.fRec8_perm[j2 as usize];
            }
            /* Compute code */
            for i in 0..vsize {
                fRec9_tmp[(i32::wrapping_add(4, i)) as usize] = fSlow0
                    * fRec9_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]
                    + fSlow1 * fRec8_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
                self.fYec0[((i32::wrapping_add(i, self.fYec0_idx)) & 2047) as usize] =
                    fSlow2 * fRec9_tmp[(i32::wrapping_add(4, i)) as usize] + fZec0[i as usize];
                fRec8_tmp[(i32::wrapping_add(4, i)) as usize] =
                    self.fYec0[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec0_idx), 1116))
                        & 2047) as usize];
            }
            /* Post code */
            self.fYec0_idx_save = vsize;
            for j1 in 0..4 {
                self.fRec9_perm[j1 as usize] = fRec9_tmp[(i32::wrapping_add(vsize, j1)) as usize];
            }
            for j3 in 0..4 {
                self.fRec8_perm[j3 as usize] = fRec8_tmp[(i32::wrapping_add(vsize, j3)) as usize];
            }
            /* Recursive loop 7 */
            /* Pre code */
            for j12 in 0..4 {
                fRec15_tmp[j12 as usize] = self.fRec15_perm[j12 as usize];
            }
            self.fYec3_idx = (i32::wrapping_add(self.fYec3_idx, self.fYec3_idx_save)) & 2047;
            for j14 in 0..4 {
                fRec14_tmp[j14 as usize] = self.fRec14_perm[j14 as usize];
            }
            /* Compute code */
            for i in 0..vsize {
                fRec15_tmp[(i32::wrapping_add(4, i)) as usize] = fSlow0
                    * fRec15_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]
                    + fSlow1 * fRec14_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
                self.fYec3[((i32::wrapping_add(i, self.fYec3_idx)) & 2047) as usize] =
                    fZec0[i as usize] + fSlow2 * fRec15_tmp[(i32::wrapping_add(4, i)) as usize];
                fRec14_tmp[(i32::wrapping_add(4, i)) as usize] =
                    self.fYec3[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec3_idx), 1356))
                        & 2047) as usize];
            }
            /* Post code */
            self.fYec3_idx_save = vsize;
            for j13 in 0..4 {
                self.fRec15_perm[j13 as usize] =
                    fRec15_tmp[(i32::wrapping_add(vsize, j13)) as usize];
            }
            for j15 in 0..4 {
                self.fRec14_perm[j15 as usize] =
                    fRec14_tmp[(i32::wrapping_add(vsize, j15)) as usize];
            }
            /* Recursive loop 8 */
            /* Pre code */
            for j28 in 0..4 {
                fRec23_tmp[j28 as usize] = self.fRec23_perm[j28 as usize];
            }
            self.fYec7_idx = (i32::wrapping_add(self.fYec7_idx, self.fYec7_idx_save)) & 2047;
            for j30 in 0..4 {
                fRec22_tmp[j30 as usize] = self.fRec22_perm[j30 as usize];
            }
            /* Compute code */
            for i in 0..vsize {
                fRec23_tmp[(i32::wrapping_add(4, i)) as usize] = fSlow0
                    * fRec23_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]
                    + fSlow1 * fRec22_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
                self.fYec7[((i32::wrapping_add(i, self.fYec7_idx)) & 2047) as usize] =
                    fZec0[i as usize] + fSlow2 * fRec23_tmp[(i32::wrapping_add(4, i)) as usize];
                fRec22_tmp[(i32::wrapping_add(4, i)) as usize] =
                    self.fYec7[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec7_idx), 1617))
                        & 2047) as usize];
            }
            /* Post code */
            self.fYec7_idx_save = vsize;
            for j29 in 0..4 {
                self.fRec23_perm[j29 as usize] =
                    fRec23_tmp[(i32::wrapping_add(vsize, j29)) as usize];
            }
            for j31 in 0..4 {
                self.fRec22_perm[j31 as usize] =
                    fRec22_tmp[(i32::wrapping_add(vsize, j31)) as usize];
            }
            /* Recursive loop 9 */
            /* Pre code */
            for j40 in 0..4 {
                fRec33_tmp[j40 as usize] = self.fRec33_perm[j40 as usize];
            }
            self.fYec12_idx = (i32::wrapping_add(self.fYec12_idx, self.fYec12_idx_save)) & 2047;
            for j42 in 0..4 {
                fRec32_tmp[j42 as usize] = self.fRec32_perm[j42 as usize];
            }
            /* Compute code */
            for i in 0..vsize {
                fRec33_tmp[(i32::wrapping_add(4, i)) as usize] = fSlow0
                    * fRec33_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]
                    + fSlow1 * fRec32_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
                self.fYec12[((i32::wrapping_add(i, self.fYec12_idx)) & 2047) as usize] =
                    fZec0[i as usize] + fSlow2 * fRec33_tmp[(i32::wrapping_add(4, i)) as usize];
                fRec32_tmp[(i32::wrapping_add(4, i)) as usize] =
                    self.fYec12[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec12_idx), 1640))
                        & 2047) as usize];
            }
            /* Post code */
            self.fYec12_idx_save = vsize;
            for j41 in 0..4 {
                self.fRec33_perm[j41 as usize] =
                    fRec33_tmp[(i32::wrapping_add(vsize, j41)) as usize];
            }
            for j43 in 0..4 {
                self.fRec32_perm[j43 as usize] =
                    fRec32_tmp[(i32::wrapping_add(vsize, j43)) as usize];
            }
            /* Recursive loop 10 */
            /* Pre code */
            for j44 in 0..4 {
                fRec35_tmp[j44 as usize] = self.fRec35_perm[j44 as usize];
            }
            self.fYec13_idx = (i32::wrapping_add(self.fYec13_idx, self.fYec13_idx_save)) & 2047;
            for j46 in 0..4 {
                fRec34_tmp[j46 as usize] = self.fRec34_perm[j46 as usize];
            }
            /* Compute code */
            for i in 0..vsize {
                fRec35_tmp[(i32::wrapping_add(4, i)) as usize] = fSlow0
                    * fRec35_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]
                    + fSlow1 * fRec34_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
                self.fYec13[((i32::wrapping_add(i, self.fYec13_idx)) & 2047) as usize] =
                    fZec0[i as usize] + fSlow2 * fRec35_tmp[(i32::wrapping_add(4, i)) as usize];
                fRec34_tmp[(i32::wrapping_add(4, i)) as usize] =
                    self.fYec13[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec13_idx), 1580))
                        & 2047) as usize];
            }
            /* Post code */
            self.fYec13_idx_save = vsize;
            for j45 in 0..4 {
                self.fRec35_perm[j45 as usize] =
                    fRec35_tmp[(i32::wrapping_add(vsize, j45)) as usize];
            }
            for j47 in 0..4 {
                self.fRec34_perm[j47 as usize] =
                    fRec34_tmp[(i32::wrapping_add(vsize, j47)) as usize];
            }
            /* Recursive loop 11 */
            /* Pre code */
            for j48 in 0..4 {
                fRec37_tmp[j48 as usize] = self.fRec37_perm[j48 as usize];
            }
            self.fYec14_idx = (i32::wrapping_add(self.fYec14_idx, self.fYec14_idx_save)) & 2047;
            for j50 in 0..4 {
                fRec36_tmp[j50 as usize] = self.fRec36_perm[j50 as usize];
            }
            /* Compute code */
            for i in 0..vsize {
                fRec37_tmp[(i32::wrapping_add(4, i)) as usize] = fSlow0
                    * fRec37_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]
                    + fSlow1 * fRec36_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
                self.fYec14[((i32::wrapping_add(i, self.fYec14_idx)) & 2047) as usize] =
                    fZec0[i as usize] + fSlow2 * fRec37_tmp[(i32::wrapping_add(4, i)) as usize];
                fRec36_tmp[(i32::wrapping_add(4, i)) as usize] =
                    self.fYec14[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec14_idx), 1445))
                        & 2047) as usize];
            }
            /* Post code */
            self.fYec14_idx_save = vsize;
            for j49 in 0..4 {
                self.fRec37_perm[j49 as usize] =
                    fRec37_tmp[(i32::wrapping_add(vsize, j49)) as usize];
            }
            for j51 in 0..4 {
                self.fRec36_perm[j51 as usize] =
                    fRec36_tmp[(i32::wrapping_add(vsize, j51)) as usize];
            }
            /* Recursive loop 12 */
            /* Pre code */
            for j52 in 0..4 {
                fRec39_tmp[j52 as usize] = self.fRec39_perm[j52 as usize];
            }
            self.fYec15_idx = (i32::wrapping_add(self.fYec15_idx, self.fYec15_idx_save)) & 2047;
            for j54 in 0..4 {
                fRec38_tmp[j54 as usize] = self.fRec38_perm[j54 as usize];
            }
            /* Compute code */
            for i in 0..vsize {
                fRec39_tmp[(i32::wrapping_add(4, i)) as usize] = fSlow0
                    * fRec39_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]
                    + fSlow1 * fRec38_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
                self.fYec15[((i32::wrapping_add(i, self.fYec15_idx)) & 2047) as usize] =
                    fZec0[i as usize] + fSlow2 * fRec39_tmp[(i32::wrapping_add(4, i)) as usize];
                fRec38_tmp[(i32::wrapping_add(4, i)) as usize] =
                    self.fYec15[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec15_idx), 1514))
                        & 2047) as usize];
            }
            /* Post code */
            self.fYec15_idx_save = vsize;
            for j53 in 0..4 {
                self.fRec39_perm[j53 as usize] =
                    fRec39_tmp[(i32::wrapping_add(vsize, j53)) as usize];
            }
            for j55 in 0..4 {
                self.fRec38_perm[j55 as usize] =
                    fRec38_tmp[(i32::wrapping_add(vsize, j55)) as usize];
            }
            /* Recursive loop 13 */
            /* Pre code */
            for j56 in 0..4 {
                fRec41_tmp[j56 as usize] = self.fRec41_perm[j56 as usize];
            }
            self.fYec16_idx = (i32::wrapping_add(self.fYec16_idx, self.fYec16_idx_save)) & 2047;
            for j58 in 0..4 {
                fRec40_tmp[j58 as usize] = self.fRec40_perm[j58 as usize];
            }
            /* Compute code */
            for i in 0..vsize {
                fRec41_tmp[(i32::wrapping_add(4, i)) as usize] = fSlow0
                    * fRec41_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]
                    + fSlow1 * fRec40_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
                self.fYec16[((i32::wrapping_add(i, self.fYec16_idx)) & 2047) as usize] =
                    fZec0[i as usize] + fSlow2 * fRec41_tmp[(i32::wrapping_add(4, i)) as usize];
                fRec40_tmp[(i32::wrapping_add(4, i)) as usize] =
                    self.fYec16[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec16_idx), 1300))
                        & 2047) as usize];
            }
            /* Post code */
            self.fYec16_idx_save = vsize;
            for j57 in 0..4 {
                self.fRec41_perm[j57 as usize] =
                    fRec41_tmp[(i32::wrapping_add(vsize, j57)) as usize];
            }
            for j59 in 0..4 {
                self.fRec40_perm[j59 as usize] =
                    fRec40_tmp[(i32::wrapping_add(vsize, j59)) as usize];
            }
            /* Recursive loop 14 */
            /* Pre code */
            for j60 in 0..4 {
                fRec43_tmp[j60 as usize] = self.fRec43_perm[j60 as usize];
            }
            self.fYec17_idx = (i32::wrapping_add(self.fYec17_idx, self.fYec17_idx_save)) & 2047;
            for j62 in 0..4 {
                fRec42_tmp[j62 as usize] = self.fRec42_perm[j62 as usize];
            }
            /* Compute code */
            for i in 0..vsize {
                fRec43_tmp[(i32::wrapping_add(4, i)) as usize] = fSlow0
                    * fRec43_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]
                    + fSlow1 * fRec42_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
                self.fYec17[((i32::wrapping_add(i, self.fYec17_idx)) & 2047) as usize] =
                    fZec0[i as usize] + fSlow2 * fRec43_tmp[(i32::wrapping_add(4, i)) as usize];
                fRec42_tmp[(i32::wrapping_add(4, i)) as usize] =
                    self.fYec17[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec17_idx), 1139))
                        & 2047) as usize];
            }
            /* Post code */
            self.fYec17_idx_save = vsize;
            for j61 in 0..4 {
                self.fRec43_perm[j61 as usize] =
                    fRec43_tmp[(i32::wrapping_add(vsize, j61)) as usize];
            }
            for j63 in 0..4 {
                self.fRec42_perm[j63 as usize] =
                    fRec42_tmp[(i32::wrapping_add(vsize, j63)) as usize];
            }
            /* Recursive loop 15 */
            /* Pre code */
            for j64 in 0..4 {
                fRec45_tmp[j64 as usize] = self.fRec45_perm[j64 as usize];
            }
            self.fYec18_idx = (i32::wrapping_add(self.fYec18_idx, self.fYec18_idx_save)) & 2047;
            for j66 in 0..4 {
                fRec44_tmp[j66 as usize] = self.fRec44_perm[j66 as usize];
            }
            /* Compute code */
            for i in 0..vsize {
                fRec45_tmp[(i32::wrapping_add(4, i)) as usize] = fSlow0
                    * fRec45_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]
                    + fSlow1 * fRec44_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
                self.fYec18[((i32::wrapping_add(i, self.fYec18_idx)) & 2047) as usize] =
                    fZec0[i as usize] + fSlow2 * fRec45_tmp[(i32::wrapping_add(4, i)) as usize];
                fRec44_tmp[(i32::wrapping_add(4, i)) as usize] =
                    self.fYec18[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec18_idx), 1211))
                        & 2047) as usize];
            }
            /* Post code */
            self.fYec18_idx_save = vsize;
            for j65 in 0..4 {
                self.fRec45_perm[j65 as usize] =
                    fRec45_tmp[(i32::wrapping_add(vsize, j65)) as usize];
            }
            for j67 in 0..4 {
                self.fRec44_perm[j67 as usize] =
                    fRec44_tmp[(i32::wrapping_add(vsize, j67)) as usize];
            }
            /* Recursive loop 16 */
            /* Pre code */
            for j68 in 0..4 {
                fRec47_tmp[j68 as usize] = self.fRec47_perm[j68 as usize];
            }
            self.fYec19_idx = (i32::wrapping_add(self.fYec19_idx, self.fYec19_idx_save)) & 2047;
            for j70 in 0..4 {
                fRec46_tmp[j70 as usize] = self.fRec46_perm[j70 as usize];
            }
            /* Compute code */
            for i in 0..vsize {
                fRec47_tmp[(i32::wrapping_add(4, i)) as usize] = fSlow0
                    * fRec47_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]
                    + fSlow1 * fRec46_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
                self.fYec19[((i32::wrapping_add(i, self.fYec19_idx)) & 2047) as usize] =
                    fZec0[i as usize] + fSlow2 * fRec47_tmp[(i32::wrapping_add(4, i)) as usize];
                fRec46_tmp[(i32::wrapping_add(4, i)) as usize] =
                    self.fYec19[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec19_idx), 1379))
                        & 2047) as usize];
            }
            /* Post code */
            self.fYec19_idx_save = vsize;
            for j69 in 0..4 {
                self.fRec47_perm[j69 as usize] =
                    fRec47_tmp[(i32::wrapping_add(vsize, j69)) as usize];
            }
            for j71 in 0..4 {
                self.fRec46_perm[j71 as usize] =
                    fRec46_tmp[(i32::wrapping_add(vsize, j71)) as usize];
            }
            /* Vectorizable loop 17 */
            /* Compute code */
            for i in 0..vsize {
                fZec1[i as usize] = fRec8_tmp[(i32::wrapping_add(4, i)) as usize]
                    + fRec10_tmp[(i32::wrapping_add(4, i)) as usize]
                    + fRec12_tmp[(i32::wrapping_add(4, i)) as usize]
                    + fRec14_tmp[(i32::wrapping_add(4, i)) as usize]
                    + fRec16_tmp[(i32::wrapping_add(4, i)) as usize]
                    + fRec18_tmp[(i32::wrapping_add(4, i)) as usize]
                    + fRec20_tmp[(i32::wrapping_add(4, i)) as usize]
                    + fRec22_tmp[(i32::wrapping_add(4, i)) as usize];
            }
            /* Vectorizable loop 18 */
            /* Compute code */
            for i in 0..vsize {
                fZec2[i as usize] = fRec32_tmp[(i32::wrapping_add(4, i)) as usize]
                    + fRec34_tmp[(i32::wrapping_add(4, i)) as usize]
                    + fRec36_tmp[(i32::wrapping_add(4, i)) as usize]
                    + fRec38_tmp[(i32::wrapping_add(4, i)) as usize]
                    + fRec40_tmp[(i32::wrapping_add(4, i)) as usize]
                    + fRec42_tmp[(i32::wrapping_add(4, i)) as usize]
                    + fRec44_tmp[(i32::wrapping_add(4, i)) as usize]
                    + fRec46_tmp[(i32::wrapping_add(4, i)) as usize];
            }
            /* Recursive loop 19 */
            /* Pre code */
            self.fYec8_idx = (i32::wrapping_add(self.fYec8_idx, self.fYec8_idx_save)) & 1023;
            for j32 in 0..4 {
                fRec6_tmp[j32 as usize] = self.fRec6_perm[j32 as usize];
            }
            /* Compute code */
            for i in 0..vsize {
                self.fYec8[((i32::wrapping_add(i, self.fYec8_idx)) & 1023) as usize] = fZec1
                    [i as usize]
                    + 0.5 * fRec6_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
                fRec6_tmp[(i32::wrapping_add(4, i)) as usize] =
                    self.fYec8[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec8_idx), 556))
                        & 1023) as usize];
                fRec7[i as usize] = fRec6_tmp
                    [(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]
                    - fZec1[i as usize];
            }
            /* Post code */
            self.fYec8_idx_save = vsize;
            for j33 in 0..4 {
                self.fRec6_perm[j33 as usize] = fRec6_tmp[(i32::wrapping_add(vsize, j33)) as usize];
            }
            /* Recursive loop 20 */
            /* Pre code */
            self.fYec20_idx = (i32::wrapping_add(self.fYec20_idx, self.fYec20_idx_save)) & 1023;
            for j72 in 0..4 {
                fRec30_tmp[j72 as usize] = self.fRec30_perm[j72 as usize];
            }
            /* Compute code */
            for i in 0..vsize {
                self.fYec20[((i32::wrapping_add(i, self.fYec20_idx)) & 1023) as usize] = 0.5
                    * fRec30_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]
                    + fZec2[i as usize];
                fRec30_tmp[(i32::wrapping_add(4, i)) as usize] =
                    self.fYec20[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec20_idx), 579))
                        & 1023) as usize];
                fRec31[i as usize] = fRec30_tmp
                    [(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]
                    - fZec2[i as usize];
            }
            /* Post code */
            self.fYec20_idx_save = vsize;
            for j73 in 0..4 {
                self.fRec30_perm[j73 as usize] =
                    fRec30_tmp[(i32::wrapping_add(vsize, j73)) as usize];
            }
            /* Recursive loop 21 */
            /* Pre code */
            self.fYec9_idx = (i32::wrapping_add(self.fYec9_idx, self.fYec9_idx_save)) & 511;
            for j34 in 0..4 {
                fRec4_tmp[j34 as usize] = self.fRec4_perm[j34 as usize];
            }
            /* Compute code */
            for i in 0..vsize {
                self.fYec9[((i32::wrapping_add(i, self.fYec9_idx)) & 511) as usize] = fRec7
                    [i as usize]
                    + 0.5 * fRec4_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
                fRec4_tmp[(i32::wrapping_add(4, i)) as usize] =
                    self.fYec9[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec9_idx), 441))
                        & 511) as usize];
                fRec5[i as usize] = fRec4_tmp
                    [(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]
                    - fRec7[i as usize];
            }
            /* Post code */
            self.fYec9_idx_save = vsize;
            for j35 in 0..4 {
                self.fRec4_perm[j35 as usize] = fRec4_tmp[(i32::wrapping_add(vsize, j35)) as usize];
            }
            /* Recursive loop 22 */
            /* Pre code */
            self.fYec21_idx = (i32::wrapping_add(self.fYec21_idx, self.fYec21_idx_save)) & 511;
            for j74 in 0..4 {
                fRec28_tmp[j74 as usize] = self.fRec28_perm[j74 as usize];
            }
            /* Compute code */
            for i in 0..vsize {
                self.fYec21[((i32::wrapping_add(i, self.fYec21_idx)) & 511) as usize] = fRec31
                    [i as usize]
                    + 0.5 * fRec28_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
                fRec28_tmp[(i32::wrapping_add(4, i)) as usize] =
                    self.fYec21[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec21_idx), 464))
                        & 511) as usize];
                fRec29[i as usize] = fRec28_tmp
                    [(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]
                    - fRec31[i as usize];
            }
            /* Post code */
            self.fYec21_idx_save = vsize;
            for j75 in 0..4 {
                self.fRec28_perm[j75 as usize] =
                    fRec28_tmp[(i32::wrapping_add(vsize, j75)) as usize];
            }
            /* Recursive loop 23 */
            /* Pre code */
            self.fYec10_idx = (i32::wrapping_add(self.fYec10_idx, self.fYec10_idx_save)) & 511;
            for j36 in 0..4 {
                fRec2_tmp[j36 as usize] = self.fRec2_perm[j36 as usize];
            }
            /* Compute code */
            for i in 0..vsize {
                self.fYec10[((i32::wrapping_add(i, self.fYec10_idx)) & 511) as usize] = fRec5
                    [i as usize]
                    + 0.5 * fRec2_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
                fRec2_tmp[(i32::wrapping_add(4, i)) as usize] =
                    self.fYec10[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec10_idx), 341))
                        & 511) as usize];
                fRec3[i as usize] = fRec2_tmp
                    [(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]
                    - fRec5[i as usize];
            }
            /* Post code */
            self.fYec10_idx_save = vsize;
            for j37 in 0..4 {
                self.fRec2_perm[j37 as usize] = fRec2_tmp[(i32::wrapping_add(vsize, j37)) as usize];
            }
            /* Recursive loop 24 */
            /* Pre code */
            self.fYec22_idx = (i32::wrapping_add(self.fYec22_idx, self.fYec22_idx_save)) & 511;
            for j76 in 0..4 {
                fRec26_tmp[j76 as usize] = self.fRec26_perm[j76 as usize];
            }
            /* Compute code */
            for i in 0..vsize {
                self.fYec22[((i32::wrapping_add(i, self.fYec22_idx)) & 511) as usize] = fRec29
                    [i as usize]
                    + 0.5 * fRec26_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
                fRec26_tmp[(i32::wrapping_add(4, i)) as usize] =
                    self.fYec22[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec22_idx), 364))
                        & 511) as usize];
                fRec27[i as usize] = fRec26_tmp
                    [(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]
                    - fRec29[i as usize];
            }
            /* Post code */
            self.fYec22_idx_save = vsize;
            for j77 in 0..4 {
                self.fRec26_perm[j77 as usize] =
                    fRec26_tmp[(i32::wrapping_add(vsize, j77)) as usize];
            }
            /* Recursive loop 25 */
            /* Pre code */
            self.fYec11_idx = (i32::wrapping_add(self.fYec11_idx, self.fYec11_idx_save)) & 255;
            for j38 in 0..4 {
                fRec0_tmp[j38 as usize] = self.fRec0_perm[j38 as usize];
            }
            /* Compute code */
            for i in 0..vsize {
                self.fYec11[((i32::wrapping_add(i, self.fYec11_idx)) & 255) as usize] = fRec3
                    [i as usize]
                    + 0.5 * fRec0_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
                fRec0_tmp[(i32::wrapping_add(4, i)) as usize] =
                    self.fYec11[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec11_idx), 225))
                        & 255) as usize];
                fRec1[i as usize] = fRec0_tmp
                    [(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]
                    - fRec3[i as usize];
            }
            /* Post code */
            self.fYec11_idx_save = vsize;
            for j39 in 0..4 {
                self.fRec0_perm[j39 as usize] = fRec0_tmp[(i32::wrapping_add(vsize, j39)) as usize];
            }
            /* Recursive loop 26 */
            /* Pre code */
            self.fYec23_idx = (i32::wrapping_add(self.fYec23_idx, self.fYec23_idx_save)) & 255;
            for j78 in 0..4 {
                fRec24_tmp[j78 as usize] = self.fRec24_perm[j78 as usize];
            }
            /* Compute code */
            for i in 0..vsize {
                self.fYec23[((i32::wrapping_add(i, self.fYec23_idx)) & 255) as usize] = fRec27
                    [i as usize]
                    + 0.5 * fRec24_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
                fRec24_tmp[(i32::wrapping_add(4, i)) as usize] =
                    self.fYec23[((i32::wrapping_sub(i32::wrapping_add(i, self.fYec23_idx), 248))
                        & 255) as usize];
                fRec25[i as usize] = fRec24_tmp
                    [(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize]
                    - fRec27[i as usize];
            }
            /* Post code */
            self.fYec23_idx_save = vsize;
            for j79 in 0..4 {
                self.fRec24_perm[j79 as usize] =
                    fRec24_tmp[(i32::wrapping_add(vsize, j79)) as usize];
            }
            /* Vectorizable loop 27 */
            /* Compute code */
            for i in 0..vsize {
                output0[i as usize] = fSlow3 * fRec1[i as usize] + fSlow4 * input0[i as usize];
            }
            /* Vectorizable loop 28 */
            /* Compute code */
            for i in 0..vsize {
                output1[i as usize] = fSlow3 * fRec25[i as usize] + fSlow4 * input1[i as usize];
            }
        }
    }
}

pub fn criterion_benchmark(c: &mut Criterion) {
    const SIZE: usize = 4096;
    let mut group = c.benchmark_group("freeverb");
    group.throughput(Throughput::Elements(SIZE as u64));
    let mut dsp = default_boxed::DefaultBoxed::default_boxed();
    let buf0 = [0.0; SIZE];
    let buf1 = [0.0; SIZE];
    let mut buf2 = [0.0; SIZE];
    let mut buf3 = [0.0; SIZE];

    group.bench_function("scalar", |b| {
        b.iter(|| {
            black_box(mydsp::compute(
                black_box(&mut dsp),
                black_box(SIZE as i32),
                black_box(&[&buf0, &buf1]),
                black_box(&mut [&mut buf2, &mut buf3]),
            ))
        })
    });

    let mut dsp_vec = default_boxed::DefaultBoxed::default_boxed();
    group.bench_with_input(BenchmarkId::new("vec", 4), &4, |b, i| {
        b.iter(|| {
            black_box(mydsp_vec::compute_4(
                black_box(&mut dsp_vec),
                black_box(SIZE as i32),
                black_box(&[&buf0, &buf1]),
                black_box(&mut [&mut buf2, &mut buf3]),
            ))
        })
    });
    // group.bench_with_input(BenchmarkId::new("original vec", 32), &32, |b, i| {
    //     b.iter(|| {
    //         black_box(mydsp_vec::compute_32(
    //             black_box(&mut dsp_vec),
    //             black_box(SIZE as i32),
    //             black_box(&[&buf0, &buf1]),
    //             black_box(&mut [&mut buf2, &mut buf3]),
    //         ))
    //     })
    // });
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
