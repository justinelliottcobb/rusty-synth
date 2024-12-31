use super::{SineOscillator, SquareOscillator};
use crate::traits::Oscillator;

pub struct MixedOscillator {
    sine: SineOscillator,
    square: SquareOscillator,
    mix_ratio: f32,
}

impl MixedOscillator {
    pub fn new(frequency: f32, sample_rate: f32, mix_ratio: f32) -> Self {
        Self {
            sine: SineOscillator::new(frequency, sample_rate),
            square: SquareOscillator::new(frequency, sample_rate),
            mix_ratio: mix_ratio.clamp(0.0, 1.0),
        }
    }

    pub fn set_frequency(&mut self, freq: f32) {
        self.sine = SineOscillator::new(freq, self.sine.get_sample_rate());
        self.square = SquareOscillator::new(freq, self.square.get_sample_rate());
    }

    pub fn set_mix_ratio(&mut self, ratio: f32) {
        self.mix_ratio = ratio.clamp(0.0, 1.0);
    }
}

impl Oscillator for MixedOscillator {
    fn next_sample(&mut self) -> f32 {
        let sine_sample = self.sine.next_sample();
        let square_sample = self.square.next_sample();
        (1.0 - self.mix_ratio) * sine_sample + self.mix_ratio * square_sample
    }
}
