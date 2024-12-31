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

    // Set initial parameters
    voice.set_frequency(440.0);
    voice.set_filter_cutoff(1000.0);
    voice.set_filter_resonance(0.05);
    voice.set_filter_lfo_rate(0.5);
    voice.set_mix_ratio(0.3);

    let stream = device.build_output_stream(
        &config.into(),
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            let elapsed = start_time.elapsed().as_secs_f32();

            // Frequency sweep between 220Hz and 880Hz
            let freq = 220.0 + (330.0 * (elapsed * 0.5).sin() + 330.0);
            voice.set_frequency(freq);

            // Filter cutoff sweep
            let cutoff = 500.0 + (2000.0 * (elapsed * 0.25).sin() + 2000.0);
            voice.set_filter_cutoff(cutoff);

            // Mix ratio sweep
            let mix = (elapsed * 0.1).sin() * 0.5 + 0.5;
            voice.set_mix_ratio(mix);

            for sample in data.iter_mut() {
                *sample = voice.process_sample();
            }
        },
        |err| eprintln!("Error: {}", err),
        None,
    )?;

    stream.play()?;
    std::thread::sleep(std::time::Duration::from_secs(10)); // Run for 10 seconds
    Ok(())
}
