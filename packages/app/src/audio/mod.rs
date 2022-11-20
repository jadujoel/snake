use gloo::console;
use web_sys::AudioContext;
mod note;
use note::{note_to_frequency, Note};

use crate::{
    audio::note::CMAJOR_SCALE,
    utils::{clamp, random},
};

fn linear_from_decibel(decibel: f32) -> f32 {
    10.0_f32.powf(decibel / 20.0)
}

fn decibel_from_linear(linear: f32) -> f32 {
    20.0 * linear.log10()
}

// export function decibelFromLinear(linear: number) {
//     return 20 * Math.log10(linear)
// }

pub fn create_audio_context() -> AudioContext {
    web_sys::AudioContext::new().unwrap()
}

pub fn play_oscillator(context: &AudioContext, frequency: f32, gain: f32) {
    let now = context.current_time();
    let oscillator = context.create_oscillator().unwrap();
    oscillator.set_type(web_sys::OscillatorType::Square);
    oscillator.frequency().set_value(clamp(
        10.0,
        frequency + random(0, 800) as f32 - 400.0,
        20000.0,
    ));
    oscillator.start().unwrap();
    oscillator.stop_with_when(now + 0.4);
    oscillator.frequency().set_value_curve_at_time(
        &mut [frequency, random(10, 800) as f32],
        now,
        0.3,
    );

    let gainNode = context.create_gain().unwrap();
    gainNode.gain().set_value(linear_from_decibel(gain));
    gainNode
        .gain()
        .exponential_ramp_to_value_at_time(0.01, now + 0.3);
    oscillator
        .connect_with_audio_node(&gainNode)
        .unwrap()
        .connect_with_audio_node(&context.destination());
}

pub fn play_step(context: &AudioContext, frequency: f32, gain: f32) {
    let now = context.current_time();
    let oscillator = context.create_oscillator().unwrap();
    oscillator.set_type(web_sys::OscillatorType::Sawtooth);
    oscillator
        .frequency()
        .set_value(clamp(10.0, frequency, 20000.0));
    oscillator.start().unwrap();
    oscillator.stop_with_when(now + 0.4);
    oscillator.frequency().set_value_curve_at_time(
        &mut [frequency + random(0, 20) as f32, frequency / 2.0 as f32],
        now,
        0.1,
    );

    let gainNode = context.create_gain().unwrap();
    gainNode.gain().set_value(linear_from_decibel(gain));
    gainNode
        .gain()
        .exponential_ramp_to_value_at_time(0.01, now + 0.1);
    oscillator
        .connect_with_audio_node(&gainNode)
        .unwrap()
        .connect_with_audio_node(&context.destination());
}

#[derive(Clone, PartialEq)]
struct AudioEngine {
    context: AudioContext,
}

impl AudioEngine {
    pub fn new(context: AudioContext) -> Self {
        Self { context }
    }

    pub fn trigger(&self, event: &str, val: Option<f32>) {
        console::log!("[audio] event", event, val);

        match event {
            "eat" => self.play_sound(440.0, -18.0),
            "die" => self.play_sound(220.0, -18.0),
            "step" => play_step(
                &self.context,
                match val {
                    Some(val) => {
                        let min = 24;
                        let max = 56;
                        let note = CMAJOR_SCALE[val as usize % CMAJOR_SCALE.len()] % Note::C4 + Note::C3;
                        let frequency = note_to_frequency(note);
                        console::log!("note", note.to_string());
                        frequency
                    }
                    None => 440.0,
                },
                -96.0,
            ),
            _ => {}
        }
    }

    pub fn play_sound(&self, frequency: f32, gain: f32) {
        play_oscillator(&self.context, frequency, gain);
    }
}

#[derive(Clone, PartialEq)]
pub struct AudioEngineProvider {
    audio_engine: Option<AudioEngine>,
    is_enabled: bool,
}

impl AudioEngineProvider {
    pub fn new() -> Self {
        Self {
            audio_engine: None,
            is_enabled: false,
        }
    }

    // use this on user action to create the audio context
    pub fn start(&mut self) {
        console::log!("[audio] start");
        let context = create_audio_context();
        context.resume();
        let audio_engine = AudioEngine::new(context);
        self.audio_engine = Some(audio_engine);
        self.is_enabled = true;
    }

    pub fn trigger(&self, event: &str, val: Option<f32>) {
        if self.is_enabled {
            match &self.audio_engine {
                Some(audio_engine) => audio_engine.trigger(event, val),
                None => {}
            }
        }
    }
}
