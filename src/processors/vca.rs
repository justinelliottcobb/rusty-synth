use crate::traits::Modulatable;

pub struct VCA {
    input_gain: f32,
    modulation_amount: f32,
    current_modulation: f32,
}

impl VCA {
    pub fn new(input_gain: f32, modulation_amount: f32) -> Self {
        Self {
            input_gain,
            modulation_amount,
            current_modulation: 1.0,
        }
    }

    pub fn process(&mut self, input: f32) -> f32 {
        input * self.input_gain * self.current_modulation
    }
}

impl Modulatable for VCA {
    fn set_modulation(&mut self, value: f32) {
        let normalized = (value + 1.0) * 0.5;
        self.current_modulation = 1.0 + (normalized - 0.5) * self.modulation_amount;
    }

    fn get_modulation(&self) -> f32 {
        self.current_modulation
    }
}
