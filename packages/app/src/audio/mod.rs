use std::{cell::RefCell, rc::Rc};

use gloo::{console, events::EventListener};
use wasm_bindgen::JsCast;
use web_sys::{AudioContext, KeyboardEvent};

use crate::utils::{clamp, random};

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

#[derive(Clone, PartialEq)]
struct AudioEngine {
    context: AudioContext,
}

impl AudioEngine {
    pub fn new(context: AudioContext) -> Self {
        Self { context }
    }

    pub fn trigger(&self, event: &str) {
        console::log!("[audio] event", event);

        match event {
            "eat" => self.play_sound(440.0, -18.0),
            "die" => self.play_sound(220.0, -18.0),
            "step" => self.play_sound(40.0, -80.0),
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
        // let me = Rc::new(Self {
        //     audio_engine: None,
        //     is_enabled: false,
        // });

        let me = Rc::new(RefCell::new(Self {
            audio_engine: None,
            is_enabled: false,
        }));

        let window = web_sys::window().expect("global window does not exists");

        // {
        //     let me = me.clone();
        //     let on_keydown = EventListener::new(&window, "keydown", move |event| {
        //         // let keyboard_event = event.clone().dyn_into::<KeyboardEvent>().unwrap();
        //         if !me.borrow().is_enabled {
        //             me.borrow_mut().start();
        //             me.borrow_mut().is_enabled = true;
        //         }
        //     });
        //     on_keydown.forget();
        // }
        return (*me).borrow_mut().clone();
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

    pub fn trigger(&self, event: &str) {
        if self.is_enabled {
            match &self.audio_engine {
                Some(audio_engine) => audio_engine.trigger(event),
                None => {}
            }
        }
    }
}
