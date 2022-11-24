use std::borrow::Borrow;

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
        match &self.source {
            Some(source) => match source.playback_rate().cancel_scheduled_values(0.0) {
                Ok(param) => param.set_value(self.playback_rate),
                Err(_) => {}
            },
            None => {}
        }
        self.is_playing = true;
    }

    pub fn slow_down(&mut self, duration: f64) {
        match &self.source {
            Some(source) => match source.playback_rate().cancel_scheduled_values(0.0) {
                Ok(param) => {
                    let current = source.playback_rate().value();
                    match param.set_value_curve_at_time(
                        &mut [current, 0.0 as f32],
                        self.context.current_time(),
                        duration,
                    ) {
                        Ok(_) => {}
                        Err(_) => {}
                    }
                }
                Err(_) => {}
            },
            None => {}
        }
    }

    pub fn speed_up(&mut self, duration: f64) {
        match &self.source {
            Some(source) => {
                let current = source.playback_rate().value();
                let result = source.playback_rate().set_value_curve_at_time(
                    &mut [current, 1.0 as f32],
                    self.context.current_time(),
                    duration,
                );
                match result {
                    Ok(_) => {
                        self.is_playing = true;
                    }
                    Err(_) => {}
                }
            }
            None => {}
        }
    }

    pub fn stop(&mut self) {
        match &self.source {
            Some(source) => match source.stop() {
                Ok(_) => {}
                Err(_) => {}
            },
            None => {}
        }
        self.is_playing = false;
    }
}
