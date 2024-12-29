use anyhow::Result;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use rust_synth::{
    modulators::{ModulationOscillator, ModulationShape},
    oscillators::MixedOscillator,
    processors::{BiquadFilter, VCA},
    traits::{Modulatable, Oscillator},
};

fn main() -> Result<()> {
    let host = cpal::default_host();
    let device = host.default_output_device().unwrap();
    let config = device.default_output_config()?;
    let sample_rate = config.sample_rate().0 as f32;

    let mut audio_osc = MixedOscillator::new(440.0, sample_rate, 0.3);

    let mut vca = VCA::new(0.5, 0.8);

    let mut filter = BiquadFilter::new(sample_rate, 1000.0, 0.1); // Much lower resonance
    let mut filter_lfo = ModulationOscillator::new(0.5, sample_rate, ModulationShape::Triangle); // Faster, triangle wave

    let stream = device.build_output_stream(
        &config.into(),
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            for sample in data.iter_mut() {
                let audio_sample = audio_osc.next_sample();

                // Reduce modulation depth significantly
                let filter_mod = filter_lfo.next_sample() * 0.25; // Only use 25% modulation depth
                filter.set_modulation(filter_mod);

                let filtered = filter.process(audio_sample);
                *sample = vca.process(filtered * 0.5); // Reduce output volume
            }
        },
        |err| eprintln!("Error: {}", err),
        None,
    )?;

    stream.play()?;
    std::thread::sleep(std::time::Duration::from_secs(4));
    Ok(())
}
