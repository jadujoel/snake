#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod app;
mod render;
mod game;
mod audio;
mod utils;

use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

/// # Errors
///
/// Will return `Err` if anything goes wrong in the app
#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    yew::start_app::<app::App>();
    Ok(())
}
