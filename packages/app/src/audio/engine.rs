use gloo::console;
use web_sys::AudioContext;

use crate::{
    audio::{play, player::Player, scales},
    utils::random_usize,
};

#[derive(Clone)]
pub struct Engine {
    context: AudioContext,
    player: Player,
}

impl Engine {
    pub fn new(context: AudioContext) -> Self {
        // assets::load_audio_buffers();
        let player = Player::new(context.clone());
        Self { context, player }
    }

    pub fn start(&mut self) {
        self.player.play(&self.context);
    }

    pub fn trigger(&mut self, event: &str, val: Option<f64>) {
        if event != "step" {
            console::log!("[engine] event", event, val);
        }
        #[allow(clippy::match_same_arms)]
        match event {
            "eat" => {
                play::oscillator(&self.context, 440.0, -18.0);
            }
            "start" => {
                self.start();
            }
            "pause" => {
                play::oscillator(&self.context, 1600.0, -18.0);
            }
            "resume" => {
                self.player.resume();
                play::oscillator(&self.context, 880.0, -18.0);
            }
            "restart" => {
                play::oscillator(&self.context, 880.0, -18.0);
            }
            "win" => {
                play::oscillator(&self.context, 1600.0, -18.0);
            }
            "lose" => {
                self.player.slow_down(2.0);
                play::oscillator(&self.context, 220.0, -9.0);
            }
            "direction" => {
                let random = random_usize(12, 24);
                let note = scales::CMAJOR[random % scales::CMAJOR.len()];
                play::direction(&self.context, note.to_frequency(), -18.0);
            }
            "step" => {}
            _ => {}
        }
    }
}
