use gloo::console;
use web_sys::{AudioBufferSourceNode, AudioContext};

use super::assets;

// derive clone, to owned
#[derive(Clone)]
pub struct Player {
    // sources: Vec<AudioBufferSourceNode>,
    source: Option<AudioBufferSourceNode>,
    is_playing: bool,
    context: AudioContext,
    playback_rate: f32,
}

impl Player {
    pub fn new(context: AudioContext) -> Self {
        Self {
            source: None,
            is_playing: false,
            playback_rate: 1.0,
            context,
        }
    }

    pub fn play(&mut self, context: &AudioContext) {
        console::log!("[player] play");
        if self.is_playing {
            return;
        }
        if let Some(buffer) = assets::get_buffer(0) {
            let source = context.create_buffer_source();
            if source.is_err() {
                return;
            }
            let source = source.unwrap();
            source.set_buffer(Some(&buffer));
            source.set_loop(true);

            let destination = source.connect_with_audio_node(&context.destination());
            if destination.is_err() {
                return;
            }
            destination.unwrap();

            match source.start() {
                Ok(_) => {
                    console::log!("[player] start ok");
                    self.source = Some(source);
                    self.is_playing = true;
                }
                Err(_) => {
                    console::log!("[player] start err");
                }
            }
        }
    }

    #[allow(unused)]
    pub fn pause(&mut self) {
        match &self.source {
            Some(source) => {
                self.playback_rate = source.playback_rate().value();
                source.playback_rate().set_value(0.0);
            }
            None => {}
        }
    }

    pub fn resume(&mut self) {
        if let Some(ref source) = &self.source {
            if let Ok(ref param) = source.playback_rate().cancel_scheduled_values(0.0) {
                param.set_value(self.playback_rate)
            }
        }
        self.is_playing = true;
    }

    pub fn slow_down(&mut self, duration: f64) {
        if let Some(ref source) = &self.source {
            let start_time = 0.0;
            if let Ok(ref param) = source.playback_rate().cancel_scheduled_values(start_time) {
                let current = source.playback_rate().value();
                let mut values = [current, 0.0];
                let start_time = self.context.current_time();
                if let Err(ref _param) =
                    param.set_value_curve_at_time(&mut values, start_time, duration)
                {
                    console::log!("[player] Error setting value curve during slow down.")
                }
            }
        }
    }

    #[allow(unused)]
    pub fn speed_up(&mut self, duration: f64) {
        match &self.source {
            Some(source) => {
                let current = source.playback_rate().value();
                let mut values = [current, 1.0];
                let start_time = self.context.current_time();
                let result = source.playback_rate().set_value_curve_at_time(
                    &mut values,
                    start_time,
                    duration,
                );
                match result {
                    Ok(_) => {
                        self.is_playing = true;
                    }
                    Err(_) => {
                        console::error!("[player] Error setting value curve for speed up")
                    }
                }
            }
            None => {}
        }
    }

    #[allow(unused)]
    pub fn stop(&mut self) {
        if let Some(ref source) = &self.source {}
        self.is_playing = false;
    }
}
