use crate::traits::Modulatable;

pub struct BiquadFilter {
    sample_rate: f32,
    cutoff: f32,
    resonance: f32,
    a0: f32,
    a1: f32,
    a2: f32,
    b1: f32,
    b2: f32,
    x1: f32,
    x2: f32,
    y1: f32,
    y2: f32,
}

impl Modulatable for BiquadFilter {
    fn set_modulation(&mut self, value: f32) {
        // Map to a more conservative range
        let normalized = (value + 1.0) * 0.5;
        // Scale from 400Hz to 4000Hz
        self.cutoff = 400.0 + (normalized * 3600.0);
        self.update_coefficients();
    }

    fn get_modulation(&self) -> f32 {
        self.cutoff
    }
}

impl BiquadFilter {
    pub fn new(sample_rate: f32, cutoff: f32, resonance: f32) -> Self {
        let mut filter = Self {
            sample_rate,
            cutoff: cutoff.clamp(20.0, 20000.0), // Clamp to audible range
            resonance: resonance.clamp(0.1, 20.0), // Prevent unstable resonance
            a0: 0.0,
            a1: 0.0,
            a2: 0.0,
            b1: 0.0,
            b2: 0.0,
            x1: 0.0,
            x2: 0.0,
            y1: 0.0,
            y2: 0.0,
        };
        filter.update_coefficients();
        filter
    }

    pub fn process(&mut self, input: f32) -> f32 {
        let output = self.a0 * input + self.a1 * self.x1 + self.a2 * self.x2
            - self.b1 * self.y1
            - self.b2 * self.y2;

        // Check for NaN or infinite values
        if !output.is_finite() {
            // Reset filter state if we get bad values
            self.x1 = 0.0;
            self.x2 = 0.0;
            self.y1 = 0.0;
            self.y2 = 0.0;
            return input; // Bypass filter
        }

        self.x2 = self.x1;
        self.x1 = input;
        self.y2 = self.y1;
        self.y1 = output;

        output
    }

    fn update_coefficients(&mut self) {
        let omega = 2.0 * std::f32::consts::PI * self.cutoff.clamp(20.0, self.sample_rate / 2.0)
            / self.sample_rate;
        let cos_omega = omega.cos();
        let alpha = omega.sin() / (2.0 * self.resonance);

        let b0 = (1.0 - cos_omega) / 2.0;
        let b1 = 1.0 - cos_omega;
        let b2 = (1.0 - cos_omega) / 2.0;
        let a0 = 1.0 + alpha;
        let a1 = -2.0 * cos_omega;
        let a2 = 1.0 - alpha;

        // Normalize coefficients
        self.a0 = b0 / a0;
        self.a1 = b1 / a0;
        self.a2 = b2 / a0;
        self.b1 = -a1 / a0;
        self.b2 = -a2 / a0;
    }

    pub fn get_cutoff(&self) -> f32 {
        self.cutoff
    }

    pub fn get_resonance(&self) -> f32 {
        self.resonance
    }
}
