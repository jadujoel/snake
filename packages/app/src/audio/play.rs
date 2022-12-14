use super::math;
use crate::utils::{clamp, random_f32};
use web_sys::AudioContext;

#[allow(unused)]
pub fn oscillator(context: &AudioContext, frequency: f32, gain: f32) {
    let now = context.current_time();
    let oscillator = context.create_oscillator().unwrap();
    oscillator.set_type(web_sys::OscillatorType::Square);
    oscillator.frequency().set_value(clamp(
        10.0,
        frequency + random_f32(0.0, 800.0) - 400.0,
        20000.0,
    ));
    oscillator.start().unwrap();
    oscillator.stop_with_when(now + 0.4);
    oscillator
        .frequency()
        .set_value_curve_at_time(&mut [frequency, random_f32(10.0, 800.0)], now, 0.3);

    let gain_node = context.create_gain().unwrap();
    gain_node.gain().set_value(math::linear_from_decibel(gain));
    gain_node
        .gain()
        .exponential_ramp_to_value_at_time(0.01, now + 0.3);
    oscillator
        .connect_with_audio_node(&gain_node)
        .unwrap()
        .connect_with_audio_node(&context.destination());
}

#[allow(unused)]
pub fn step(context: &AudioContext, frequency: f32, gain: f32) {
    let now = context.current_time();
    let oscillator = context.create_oscillator().unwrap();
    oscillator.set_type(web_sys::OscillatorType::Sawtooth);
    oscillator
        .frequency()
        .set_value(clamp(10.0, frequency, 20000.0));
    oscillator.start().unwrap();
    oscillator.stop_with_when(now + 0.4);
    oscillator.frequency().set_value_curve_at_time(
        &mut [frequency + random_f32(0.0, 20.0), frequency / 2.0],
        now,
        0.1,
    );

    let gain_node = context.create_gain().unwrap();
    gain_node.gain().set_value(math::linear_from_decibel(gain));
    gain_node
        .gain()
        .exponential_ramp_to_value_at_time(0.01, now + 0.1);
    oscillator
        .connect_with_audio_node(&gain_node)
        .unwrap()
        .connect_with_audio_node(&context.destination());
}

#[allow(unused)]
pub fn direction(context: &AudioContext, frequency: f32, gain: f32) {
    let now = context.current_time();
    let oscillator = context.create_oscillator().unwrap();
    oscillator.set_type(web_sys::OscillatorType::Sine);
    oscillator.frequency().set_value(clamp(
        10.0,
        frequency + random_f32(0.0, 12.0) - 6.0,
        20000.0,
    ));
    oscillator.start().unwrap();
    oscillator.stop_with_when(now + 0.4);
    oscillator
        .frequency()
        .set_value_curve_at_time(&mut [frequency, frequency * 0.5], now, 0.3);

    let gain_node = context.create_gain().unwrap();
    gain_node.gain().set_value(math::linear_from_decibel(gain));
    gain_node
        .gain()
        .exponential_ramp_to_value_at_time(0.01, now + 0.3);
    oscillator
        .connect_with_audio_node(&gain_node)
        .unwrap()
        .connect_with_audio_node(&context.destination());
}
