#[derive(Copy, Clone)]
pub struct ParamIndex(pub usize);

pub struct Soundfile<'a, T> {
    buffers: &'a &'a T,
    length: &'a i32,
    sr: &'a i32,
    offset: &'a i32,
    channels: i32,
}

pub trait FaustDsp {
    type SampleType;

    fn new() -> Self;
    fn metadata(&self, m: &mut dyn Meta);
    fn get_sample_rate(&self) -> u32;
    fn get_num_inputs(&self) -> usize { 0 }
    fn get_num_outputs(&self) -> usize { 0 }
    fn class_init(sample_rate: u32) where Self: Sized;
    fn instance_reset_params(&mut self);
    fn instance_clear(&mut self);
    fn instance_constants(&mut self, sample_rate: u32);
    fn instance_init(&mut self, sample_rate: u32);
    fn init(&mut self, sample_rate: u32);
    fn build_user_interface(&self, ui_interface: &mut impl UI<Self::SampleType>);
    fn build_user_interface_static(ui_interface: &mut impl UI<Self::SampleType>);
    fn get_param(&self, param: ParamIndex) -> Option<Self::SampleType>;
    fn set_param(&mut self, param: ParamIndex, value: Self::SampleType);
    fn compute(&mut self, inputs: &[&[Self::SampleType]], outputs: &mut [&mut [Self::SampleType]]);
}

pub trait Meta {
    // -- metadata declarations
    fn declare(&mut self, key: &str, value: &str);
}

pub trait UI<T> {
    // -- widget's layouts
    fn open_tab_box(&mut self, label: &str);
    fn open_horizontal_box(&mut self, label: &str);
    fn open_vertical_box(&mut self, label: &str);
    fn close_box(&mut self);

    // -- active widgets
    fn add_button(&mut self, label: &str, param: ParamIndex);
    fn add_check_button(&mut self, label: &str, param: ParamIndex);
    fn add_vertical_slider(&mut self, label: &str, param: ParamIndex, init: T, min: T, max: T, step: T);
    fn add_horizontal_slider(&mut self, label: &str, param: ParamIndex, init: T, min: T, max: T, step: T);
    fn add_num_entry(&mut self, label: &str, param: ParamIndex, init: T, min: T, max: T, step: T);

    // -- passive widgets
    fn add_horizontal_bargraph(&mut self, label: &str, param: ParamIndex, min: T, max: T);
    fn add_vertical_bargraph(&mut self, label: &str, param: ParamIndex, min: T, max: T);

    // -- metadata declarations
    fn declare(&mut self, param: Option<ParamIndex>, key: &str, value: &str);
}
