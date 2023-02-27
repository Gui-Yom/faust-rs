/************************************************************************
FAUST Architecture File
Copyright (C) 2003-2019 GRAME, Centre National de Creation Musicale
---------------------------------------------------------------------
This Architecture section is free software; you can redistribute it
and/or modify it under the terms of the GNU General Public License
as published by the Free Software Foundation; either version 3 of
the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program; If not, see <http://www.gnu.org/licenses/>.

EXCEPTION : As a special exception, you may create a larger work
that contains this FAUST architecture section and distribute
that work under terms of your choice, so long as this FAUST
architecture section is not modified.

************************************************************************
************************************************************************/

#![allow(unused_parens)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
//#![allow(unused_mut)]
#![allow(non_upper_case_globals)]

//! Faust CPAL architecture file
extern crate anyhow;
extern crate cpal;

use std::marker::PhantomData;
use std::thread;
use std::time::Duration;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{BufferSize, SampleRate, StreamConfig};

use faust_traits::{FaustDsp, Meta, ParamIndex, UI};

pub struct mydsp {
    iRec0: [i32; 2],
    fSampleRate: u32,
}

impl FaustDsp for mydsp {
    type SampleType = f32;

    fn new() -> mydsp {
        mydsp {
            iRec0: [0; 2],
            fSampleRate: 0,
        }
    }
    fn metadata(&self, m: &mut dyn Meta) {
        m.declare("filename", "test.dsp");
        m.declare("name", "test");
        m.declare("noises.lib/name", "Faust Noise Generator Library");
        m.declare("noises.lib/version", "0.4");
    }

    fn get_sample_rate(&self) -> u32 {
        return self.fSampleRate;
    }

    fn get_num_outputs(&self) -> usize {
        2
    }

    fn class_init(sample_rate: u32) {}
    fn instance_reset_params(&mut self) {}
    fn instance_clear(&mut self) {
        self.iRec0.fill(0);
    }
    fn instance_constants(&mut self, sample_rate: u32) {
        self.fSampleRate = sample_rate;
    }
    fn instance_init(&mut self, sample_rate: u32) {
        self.instance_constants(sample_rate);
        self.instance_reset_params();
        self.instance_clear();
    }
    fn init(&mut self, sample_rate: u32) {
        mydsp::class_init(sample_rate);
        self.instance_init(sample_rate);
    }

    fn build_user_interface(&self, ui_interface: &mut impl UI<Self::SampleType>) {
        Self::build_user_interface_static(ui_interface);
    }

    fn build_user_interface_static(ui_interface: &mut impl UI<Self::SampleType>) {
        ui_interface.open_vertical_box("test");
        ui_interface.close_box();
    }

    fn get_param(&self, param: ParamIndex) -> Option<Self::SampleType> {
        match param.0 {
            _ => None,
        }
    }

    fn set_param(&mut self, param: ParamIndex, value: Self::SampleType) {
        match param.0 {
            _ => {}
        }
    }

    fn compute(&mut self, inputs: &[&[Self::SampleType]], outputs: &mut [&mut [Self::SampleType]]) {
        let (outputs0, outputs1) = if let [outputs0, outputs1, ..] = outputs {
            let outputs0 = outputs0[..2 as usize].iter_mut();
            let outputs1 = outputs1[..2 as usize].iter_mut();
            (outputs0, outputs1)
        } else {
            panic!("wrong number of outputs");
        };
        let zipped_iterators = outputs0.zip(outputs1);
        for (output0, output1) in zipped_iterators {
            self.iRec0[0] = i32::wrapping_add(i32::wrapping_mul(1103515245, self.iRec0[1]), 12345);
            let mut fTemp0: f32 = 4.6566128e-12 * ((self.iRec0[0]) as f32);
            *output0 = fTemp0;
            *output1 = fTemp0;
            self.iRec0[1] = self.iRec0[0];
        }
    }
}

impl mydsp {
    fn compute_slice(&mut self, outputs: [&mut [f32]; 2]) {
        let [output0_, output1_] = outputs;
        for (output0, output1) in output0_.iter_mut().zip(output1_) {
            self.iRec0[0] = i32::wrapping_add(i32::wrapping_mul(1103515245, self.iRec0[1]), 12345);
            let mut fTemp0: f32 = 4.6566128e-12 * ((self.iRec0[0]) as f32);
            *output0 = fTemp0;
            *output1 = fTemp0;
            self.iRec0[1] = self.iRec0[0];
        }
    }

    fn compute_iter_static<'a, I: Iterator<Item = &'a mut f32>>(&mut self, outputs: [&mut I; 2]) {
        // let (outputs0, outputs1) = if let [outputs0, outputs1, ..] = outputs {
        //     (outputs0, outputs1)
        // } else {
        //     panic!("wrong number of outputs");
        // };
        let [output0_, output1_] = outputs;
        for (output0, output1) in output0_.zip(output1_) {
            self.iRec0[0] = i32::wrapping_add(i32::wrapping_mul(1103515245, self.iRec0[1]), 12345);
            let mut fTemp0: f32 = 4.6566128e-12 * ((self.iRec0[0]) as f32);
            *output0 = fTemp0;
            *output1 = fTemp0;
            self.iRec0[1] = self.iRec0[0];
        }
    }

    fn compute_static<const N: usize>(&mut self, outputs: [&mut [f32; N]; 2]) {
        // let (outputs0, outputs1) = if let [outputs0, outputs1, ..] = outputs {
        //     (outputs0, outputs1)
        // } else {
        //     panic!("wrong number of outputs");
        // };
        let [output0_, output1_] = outputs;
        for (output0, output1) in output0_.iter_mut().zip(output1_) {
            self.iRec0[0] = i32::wrapping_add(i32::wrapping_mul(1103515245, self.iRec0[1]), 12345);
            let mut fTemp0: f32 = 4.6566128e-12 * ((self.iRec0[0]) as f32);
            *output0 = fTemp0;
            *output1 = fTemp0;
            self.iRec0[1] = self.iRec0[0];
        }
    }
}

fn main() -> Result<(), anyhow::Error> {
    let mut dsp = mydsp::new();

    println!("get_num_inputs: {}", dsp.get_num_inputs());
    println!("get_num_outputs: {}", dsp.get_num_outputs());

    //println!("Faust Rust code running with CPAL: sample-rate = {} buffer-size = {}", client.sample_rate(), client.buffer_size());

    println!("Supported hosts:\n  {:?}", cpal::ALL_HOSTS);
    let available_hosts = cpal::available_hosts();
    println!("Available hosts:\n  {:?}", available_hosts);

    let device = cpal::default_host().default_output_device().unwrap();
    let default_config = device.default_output_config()?;
    println!("default output config : {default_config:?}");

    dsp.init(default_config.sample_rate().0);

    let buffer_size = 1024;

    let stream = device.build_output_stream(
        &StreamConfig {
            channels: dsp.get_num_outputs() as u16,
            sample_rate: SampleRate(dsp.get_sample_rate() as u32),
            buffer_size: BufferSize::Fixed(buffer_size),
        },
        move |buf: &mut [f32], info| {
            //let mut iters = iter_stride_static(buf);
            //let mut refs = iters.iter_mut().collect::<Vec<_>>();
            //dsp.compute_iter_static(iters.map(|mut i| &mut i));

            println!("{}", buf.len());

            let mut buf0 = [0.0; 4096];
            let mut buf1 = [0.0; 4096];

            dsp.compute_slice([&mut buf0, &mut buf1]);

            interleave(buf, &buf0, &buf1);
        },
        |err| {
            println!("err: {err}");
        },
        None,
    )?;

    stream.play()?;

    thread::sleep(Duration::from_secs(5));

    stream.pause()?;

    Ok(())
}

fn interleave(buf: &mut [f32], mut buf0: &[f32], mut buf1: &[f32]) {
    for ((out, a), b) in buf.chunks_exact_mut(2).zip(buf0).zip(buf1) {
        out[0] = *a;
        out[1] = *b;
    }
}

fn iter_stride<'a, T>(slice: &'a mut [T], stride: usize) -> Vec<StrideIterator<'a, T>> {
    let mut iters = Vec::new();
    for i in 0..stride {
        iters.push(StrideIterator {
            ptr: slice.as_mut_ptr(),
            idx: i,
            len: slice.len(),
            stride,
            _marker: PhantomData,
        });
    }
    iters
}

struct StrideIterator<'a, T> {
    ptr: *mut T,
    idx: usize,
    len: usize,
    stride: usize,
    _marker: PhantomData<&'a mut T>,
}

impl<'a, T> Iterator for StrideIterator<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < self.len {
            let ret = Some(unsafe { &mut *self.ptr.add(self.idx) });
            self.idx += self.stride;
            ret
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let num = (self.len - self.idx) / self.stride;
        (num, Some(num))
    }
}

fn iter_stride_static<T: Copy, const S: usize>(slice: &mut [T]) -> [StrideIteratorStatic<T, S>; S] {
    let mut iters = [StrideIteratorStatic {
        ptr: slice.as_mut_ptr(),
        idx: 0,
        len: slice.len(),
        _marker: PhantomData,
    }; S];
    for i in 0..S {
        iters[i].idx = i;
    }
    iters
}

#[derive(Clone, Copy)]
struct StrideIteratorStatic<'a, T, const S: usize> {
    ptr: *mut T,
    idx: usize,
    len: usize,
    _marker: PhantomData<&'a mut T>,
}

impl<'a, T, const S: usize> Iterator for StrideIteratorStatic<'a, T, S> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < self.len {
            let ret = Some(unsafe { &mut *self.ptr.add(self.idx) });
            self.idx += S;
            ret
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let num = (self.len - self.idx) / S;
        (num, Some(num))
    }
}
