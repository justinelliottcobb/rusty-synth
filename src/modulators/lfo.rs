use std::f32::consts::PI;

pub enum ModulationShape {
    Sine,
    Triangle,
    Ramp,
    SmoothSquare,
}

pub struct ModulationOscillator {
    frequency: f32,
    phase: f32,
    sample_rate: f32,
    shape: ModulationShape,
}

impl ModulationOscillator {
    pub fn new(frequency: f32, sample_rate: f32, shape: ModulationShape) -> Self {
        Self {
            frequency,
            phase: 0.0,
            sample_rate,
            shape,
        }
    }

    pub fn next_sample(&mut self) -> f32 {
        let output = match self.shape {
            ModulationShape::Sine => (self.phase * 2.0 * PI).sin(),
            ModulationShape::Triangle => {
                if self.phase < 0.5 {
                    -1.0 + (4.0 * self.phase)
                } else {
                    3.0 - (4.0 * self.phase)
                }
            }
            ModulationShape::Ramp => 2.0 * self.phase - 1.0,
            ModulationShape::SmoothSquare => (self.phase * 2.0 * PI).sin().tanh() * 2.0,
        };

        self.phase = (self.phase + self.frequency / self.sample_rate) % 1.0;
        output
    }
}
