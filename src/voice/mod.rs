use crate::{
    modulators::{ModulationOscillator, ModulationShape},
    oscillators::MixedOscillator,
    processors::{BiquadFilter, VCA},
    traits::{Modulatable, Oscillator},
};

pub struct Voice {
    oscillator: MixedOscillator,
    filter: BiquadFilter,
    filter_lfo: ModulationOscillator,
    vca: VCA,
    sample_rate: f32,
}

impl Voice {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            oscillator: MixedOscillator::new(440.0, sample_rate, 0.3),
            filter: BiquadFilter::new(sample_rate, 1000.0, 0.1),
            filter_lfo: ModulationOscillator::new(0.5, sample_rate, ModulationShape::Triangle),
            vca: VCA::new(0.5, 0.8),
            sample_rate,
        }
    }

    pub fn process_sample(&mut self) -> f32 {
        let audio_sample = self.oscillator.next_sample();

        let filter_mod = self.filter_lfo.next_sample() * 0.25;
        self.filter.set_modulation(filter_mod);

        let filtered = self.filter.process(audio_sample);
        self.vca.process(filtered * 0.5)
    }

    // Parameter control methods
    pub fn set_frequency(&mut self, freq: f32) {
        // Clamp frequency to audible range (20Hz to 20kHz)
        let safe_freq = freq.clamp(20.0, 20000.0);
        // We'll need to add this method to MixedOscillator
        self.oscillator.set_frequency(safe_freq);
    }

    pub fn set_filter_cutoff(&mut self, cutoff: f32) {
        // Cutoff in Hz, clamped to reasonable range
        let safe_cutoff = cutoff.clamp(20.0, self.sample_rate / 2.0);
        self.filter = BiquadFilter::new(
            self.sample_rate,
            safe_cutoff,
            self.filter.get_resonance(), // We'll need to add this getter
        );
    }

    pub fn set_filter_resonance(&mut self, resonance: f32) {
        // Resonance typically ranges from 0.1 (minimal) to ~10.0 (high)
        let safe_resonance = resonance.clamp(0.1, 10.0);
        self.filter = BiquadFilter::new(
            self.sample_rate,
            self.filter.get_cutoff(), // We'll need to add this getter
            safe_resonance,
        );
    }

    pub fn set_filter_lfo_rate(&mut self, rate: f32) {
        // LFO rate from 0.01 Hz to 20 Hz
        let safe_rate = rate.clamp(0.01, 20.0);
        self.filter_lfo =
            ModulationOscillator::new(safe_rate, self.sample_rate, ModulationShape::Triangle);
    }

    pub fn set_mix_ratio(&mut self, ratio: f32) {
        // Ratio from 0.0 (all sine) to 1.0 (all square)
        let safe_ratio = ratio.clamp(0.0, 1.0);
        // We'll need to add this method to MixedOscillator
        self.oscillator.set_mix_ratio(safe_ratio);
    }
}
