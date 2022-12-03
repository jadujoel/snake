use web_sys::AudioContext;
use super::{engine::AudioEngine, assets::{spawn_load_audio_buffers}};

pub fn create_audio_context() -> Option<AudioContext> {
    let context = web_sys::AudioContext::new();
    match context {
        Ok(context) => { Some(context) },
        Err(_) => { None}
    }
}

#[derive(Clone)]
pub struct AudioEngineProvider {
    pub audio_engine: Option<AudioEngine>,
    pub is_started: bool,
}

impl AudioEngineProvider {
    pub fn new() -> Self {
        spawn_load_audio_buffers();
        Self {
            audio_engine: None,
            is_started: false,
        }
    }

    // use this on user action to create the audio context and initialize engine
    // pub fn start<'a>(&'a mut self) {
    pub fn start(&mut self) {
        if self.is_started {
            return;
        }
        let context = create_audio_context();
        match context {
            Some(context) => {
                self.audio_engine = Some(AudioEngine::new(context));
                self.is_started = true;
            },
            None => {}
        }
    }

    pub fn trigger(&mut self, event: &str, val: Option<f32>) {
        if self.is_started {
            match & mut self.audio_engine {
                Some(audio_engine) => audio_engine.trigger(event, val),
                None => {}
            }
        }
    }
}
