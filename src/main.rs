use anyhow::Result;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use rust_synth::voice::Voice;
use std::time::Instant;

fn main() -> Result<()> {
    let host = cpal::default_host();
    let device = host.default_output_device().unwrap();
    let config = device.default_output_config()?;
    let sample_rate = config.sample_rate().0 as f32;

    let mut voice = Voice::new(sample_rate);
    let start_time = Instant::now();

    // Initial settings
    voice.set_frequency(440.0);
    voice.set_filter_cutoff(1000.0);
    voice.set_filter_resonance(0.05);
    voice.set_filter_lfo_rate(0.5);
    voice.set_mix_ratio(0.3);

    let stream = device.build_output_stream(
        &config.into(),
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            let elapsed = start_time.elapsed().as_secs_f32();

            // Compound frequency modulation using two sine waves
            let freq_mod1 = (elapsed * 0.5).sin(); // Slow wave
            let freq_mod2 = (elapsed * 2.0).sin() * 0.3; // Faster wave with less depth
            let freq = 440.0 + ((freq_mod1 + freq_mod2) * 200.0);
            voice.set_frequency(freq);

            // Exponential filter cutoff sweep
            let filter_mod = (elapsed * 0.15).sin();
            let cutoff = 500.0 * (1.0 + filter_mod * 4.0).exp2();
            voice.set_filter_cutoff(cutoff);

            // "Random-like" mix ratio using multiple waves
            let mix = ((elapsed * 0.1).sin() + (elapsed * 0.23).sin() * 0.5) * 0.25 + 0.5;
            voice.set_mix_ratio(mix.clamp(0.0, 1.0));

            for sample in data.iter_mut() {
                *sample = voice.process_sample();
            }
        },
        |err| eprintln!("Error: {}", err),
        None,
    )?;

    stream.play()?;
    std::thread::sleep(std::time::Duration::from_secs(10));
    Ok(())
}
