use super::{assets::spawn_load_audio_buffers, engine::Engine};
use web_sys::AudioContext;

pub fn create_audio_context() -> Option<AudioContext> {
    let context = web_sys::AudioContext::new();
    match context {
        Ok(context) => Some(context),
        Err(_) => None,
    }
}

#[derive(Clone)]
pub struct System {
    pub audio_engine: Option<Engine>,
    pub is_started: bool,
}

impl System {
    pub fn new() -> Self {
        spawn_load_audio_buffers();
        Self {
            audio_engine: None,
            is_started: false,
        }
    }

    /// use this on user action to create the audio context and initialize engine
    pub fn start(&mut self) {
        if self.is_started {
            return;
        }
        if let Some(context) = create_audio_context() {
            self.audio_engine = Some(Engine::new(context));
            self.is_started = true;
        }
    }

    /// use to trigger events that the audio engine can react to
    pub fn trigger(&mut self, event: &str, val: Option<f64>) {
        if self.is_started {
            match &mut self.audio_engine {
                Some(audio_engine) => audio_engine.trigger(event, val),
                None => {}
            }
        }
    }
}
