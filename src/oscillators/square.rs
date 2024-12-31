use crate::traits::Oscillator;

pub struct SquareOscillator {
    frequency: f32,
    phase: f32,
    sample_rate: f32,
}

impl SquareOscillator {
    pub fn new(frequency: f32, sample_rate: f32) -> Self {
        Self {
            frequency,
            phase: 0.0,
            sample_rate,
        }
    }

    pub fn get_sample_rate(&self) -> f32 {
        self.sample_rate
    }
}

impl Oscillator for SquareOscillator {
    fn next_sample(&mut self) -> f32 {
        let sample = if self.phase < 0.5 { 1.0 } else { -1.0 };
        self.phase = (self.phase + self.frequency / self.sample_rate) % 1.0;
        sample
    }
}
