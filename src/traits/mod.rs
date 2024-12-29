pub trait Oscillator {
    fn next_sample(&mut self) -> f32;
}

pub trait Modulatable {
    fn set_modulation(&mut self, value: f32);
    fn get_modulation(&self) -> f32;
}
