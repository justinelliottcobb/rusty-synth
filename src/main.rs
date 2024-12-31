use anyhow::Result;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use rust_synth::voice::Voice;

fn main() -> Result<()> {
    let host = cpal::default_host();
    let device = host.default_output_device().unwrap();
    let config = device.default_output_config()?;
    let sample_rate = config.sample_rate().0 as f32;

    let mut voice = Voice::new(sample_rate);

    // Set initial parameters
    voice.set_frequency(440.0);
    voice.set_filter_cutoff(1000.0);
    voice.set_filter_resonance(0.0);
    voice.set_filter_lfo_rate(0.5);
    voice.set_mix_ratio(0.3);

    let stream = device.build_output_stream(
        &config.into(),
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            for sample in data.iter_mut() {
                *sample = voice.process_sample();
            }
        },
        |err| eprintln!("Error: {}", err),
        None,
    )?;

    stream.play()?;
    std::thread::sleep(std::time::Duration::from_secs(4));
    Ok(())
}
