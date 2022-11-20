use gloo::console;
use web_sys::AudioContext;
mod note;
mod play;
mod math;
use note::{note_to_frequency, Note};

use crate::{
    audio::note::CMAJOR_SCALE,
    utils::{random},
};

// export function decibelFromLinear(linear: number) {
//     return 20 * Math.log10(linear)
// }

pub fn create_audio_context() -> AudioContext {
    web_sys::AudioContext::new().unwrap()
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
            "eat" => play::play_oscillator(&self.context, 440.0, -18.0),
            "start" => play::play_oscillator(&self.context, 880.0, -18.0),
            "pause" => play::play_oscillator(&self.context, 1600.0, -18.0),
            "resume" => play::play_oscillator(&self.context, 880.0, -18.0),
            "restart" => play::play_oscillator(&self.context, 880.0, -18.0),
            "win" => play::play_oscillator(&self.context, 1600.0, -18.0),
            "lose" => play::play_oscillator(&self.context, 220.0, -9.0),
            "direction" => {
                let random = random(12, 24);
                let note = CMAJOR_SCALE[random % CMAJOR_SCALE.len()];
                play::play_direction(&self.context, note_to_frequency(note), -18.0);
            }
            "step" => play::play_step(
                &self.context,
                match val {
                    Some(val) => {
                        let note = CMAJOR_SCALE[val as usize % CMAJOR_SCALE.len()] % Note::C4 + Note::C3;
                        let frequency = note_to_frequency(note);
                        // console::log!("note", note.to_string());
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
        play::play_oscillator(&self.context, frequency, gain);
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
