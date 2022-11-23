use gloo::console;
use web_sys::AudioContext;

use crate::{
    audio::{
        note::{note_to_frequency, Note, CMAJOR_SCALE},
        play,
    },
    utils::random,
};

use super::assets;

#[derive(Clone)]
pub struct AudioEngine {
    context: AudioContext,
}

impl AudioEngine {
    pub fn new(context: AudioContext) -> Self {
        assets::load();
        Self { context }
    }

    pub fn trigger(&self, event: &str, val: Option<f32>) {
        console::log!("[audio] event", event, val);
        match event {
            "eat" => play::play_oscillator(&self.context, 440.0, -18.0),
            "start" => {
                console::log!("start");
                self.play_sound()
            }
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
                        let note =
                            CMAJOR_SCALE[val as usize % CMAJOR_SCALE.len()] % Note::C4 + Note::C3;
                        let frequency = note_to_frequency(note);
                        frequency
                    }
                    None => 440.0,
                },
                -96.0,
            ),
            _ => {}
        }
    }

    pub fn play_sound(&self) {
        if let Some(buffer) = assets::get(0) {
            let source = self.context.create_buffer_source();
            if source.is_err() {
                return;
            }
            let source = source.unwrap();
            source.set_buffer(Some(&buffer));

            let destination = source.connect_with_audio_node(&self.context.destination());
            if destination.is_err() {
                return;
            }
            destination.unwrap();

            match source.start() {
                Ok(..) => return,
                Err(..) => return,
            }
        }
    }
}
