use crate::traits::Oscillator;
use std::f32::consts::PI;

pub struct SineOscillator {
    frequency: f32,
    phase: f32,
    sample_rate: f32,
}

impl SineOscillator {
    pub fn new(frequency: f32, sample_rate: f32) -> Self {
        Self {
            frequency,
            phase: 0.0,
            sample_rate,
        }
    }
}

impl Oscillator for SineOscillator {
    fn next_sample(&mut self) -> f32 {
        let sample = (self.phase * 2.0 * PI).sin();
        self.phase = (self.phase + self.frequency / self.sample_rate) % 1.0;
        sample
    }
}
