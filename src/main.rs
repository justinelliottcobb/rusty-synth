use anyhow::Result;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::f32::consts::PI;

trait Oscillator {
    fn next_sample(&mut self) -> f32;
}

struct SineOscillator {
    frequency: f32,
    phase: f32,
    sample_rate: f32,
}

struct SquareOscillator {
    frequency: f32,
    phase: f32,
    sample_rate: f32,
}

impl Oscillator for SineOscillator {
    fn next_sample(&mut self) -> f32 {
        let sample = (self.phase * 2.0 * PI).sin();
        self.phase = (self.phase + self.frequency / self.sample_rate) % 1.0;
        sample
    }
}

impl Oscillator for SquareOscillator {
    fn next_sample(&mut self) -> f32 {
        let sample = if self.phase < 0.5 { 1.0 } else { -1.0 };
        self.phase = (self.phase + self.frequency / self.sample_rate) % 1.0;
        sample
    }
}

// Factory functions
impl SineOscillator {
    fn new(frequency: f32, sample_rate: f32) -> Self {
        Self {
            frequency,
            phase: 0.0,
            sample_rate,
        }
    }
}

impl SquareOscillator {
    fn new(frequency: f32, sample_rate: f32) -> Self {
        Self {
            frequency,
            phase: 0.0,
            sample_rate,
        }
    }
}

struct MixedOscillator {
    sine: SineOscillator,
    square: SquareOscillator,
    mix_ratio: f32, // 0.0 = all sine, 1.0 = all square
}

impl MixedOscillator {
    fn new(frequency: f32, sample_rate: f32, mix_ratio: f32) -> Self {
        Self {
            sine: SineOscillator::new(frequency, sample_rate),
            square: SquareOscillator::new(frequency, sample_rate),
            mix_ratio: mix_ratio.clamp(0.0, 0.4),
        }
    }
}

impl Oscillator for MixedOscillator {
    fn next_sample(&mut self) -> f32 {
        let sine_sample = self.sine.next_sample();
        let square_sample = self.square.next_sample();
        (1.0 - self.mix_ratio) * sine_sample + self.mix_ratio * square_sample
    }
}

// First, let's create a trait for any module that can be modulated
trait Modulatable {
    fn set_modulation(&mut self, value: f32);
    fn get_modulation(&self) -> f32;
}

// Our VCA will control amplitude based on modulation
struct VCA {
    input_gain: f32,
    modulation_amount: f32,
    current_modulation: f32,
}

impl VCA {
    fn new(input_gain: f32, modulation_amount: f32) -> Self {
        Self {
            input_gain,
            modulation_amount,
            current_modulation: 1.0,
        }
    }

    fn process(&mut self, input: f32) -> f32 {
        // The VCA multiplies the input by both the base gain and modulation
        input * self.input_gain * self.current_modulation
    }
}

impl Modulatable for VCA {
    fn set_modulation(&mut self, value: f32) {
        // Convert the -1.0 to 1.0 range to 0.0 to 1.0 for amplitude
        let normalized = (value + 1.0) * 0.5;
        // Apply modulation amount
        self.current_modulation = 1.0 + (normalized - 0.5) * self.modulation_amount;
    }

    fn get_modulation(&self) -> f32 {
        self.current_modulation
    }
}

fn main() -> Result<()> {
    let host = cpal::default_host();
    let device = host.default_output_device().unwrap();
    let config = device.default_output_config()?;
    let sample_rate = config.sample_rate().0 as f32;

    // Create our oscillators and VCA
    let mut audio_osc = MixedOscillator::new(440.0, sample_rate, 0.3);
    let mut lfo = SineOscillator::new(2.0, sample_rate); // 2 Hz modulation
    let mut vca = VCA::new(0.5, 0.8); // 0.5 base gain, 0.8 modulation amount

    let stream = device.build_output_stream(
        &config.into(),
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            for sample in data.iter_mut() {
                // Get the LFO value and use it to modulate the VCA
                let modulation = lfo.next_sample();
                vca.set_modulation(modulation);

                // Process audio through the VCA
                let audio_sample = audio_osc.next_sample();
                *sample = vca.process(audio_sample);
            }
        },
        |err| eprintln!("Error: {}", err),
        None,
    )?;

    stream.play()?;
    std::thread::sleep(std::time::Duration::from_secs(4));
    Ok(())
}
