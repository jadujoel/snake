use gloo::console;
use js_sys::ArrayBuffer;
use js_sys::Uint8Array;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen_futures::JsFuture;
use web_sys::window;
use web_sys::AudioBuffer;
use web_sys::AudioContext;
use web_sys::{Request, RequestInit, RequestMode, Response};
// use seek_bufread::BufReader;


const NUM_ASSETS: usize = 1;
// const NAMES: [&str; NUM_ASSETS] = ["music"];
const URLS: [&str; NUM_ASSETS] = ["audio/music-a-115bpm.webm"];
static mut AUDIO_BUFFERS: [Option<AudioBuffer>; NUM_ASSETS] = [None];

// allows us to fetch buffers before audio context could be started
static mut ARRAY_BUFFERS: [Option<ArrayBuffer>; NUM_ASSETS] = [None];

#[allow(dead_code)]
pub async fn request() {
    let origin = window().unwrap().location().origin().unwrap();
    let full_url = format!("{}/{}", origin, URLS[0]);
    console::log!("full_url:", full_url.to_owned());
    let result = reqwest::get(full_url).await;
    let response = match result {
        Ok(response) => {
            console::log!("response: OK");
            response
        }
        Err(_) => {
            console::log!("response: Err");
            return;
        }
    };
    // response.

    let body = response.text().await.unwrap();
    let buffer = Uint8Array::from(body.as_bytes()).buffer();
    unsafe {
        ARRAY_BUFFERS[0] = Some(buffer);
    };
}

pub async fn get_array_buffer(asset_index: usize) -> ArrayBuffer {
    unsafe {
        if ARRAY_BUFFERS[asset_index].is_some() {
            return ARRAY_BUFFERS[asset_index].to_owned().unwrap();
        }
    }

    let url = URLS[asset_index];

    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);
    let request = Request::new_with_str_and_init(&url, &opts).unwrap();
    request.headers().set("Accept", "application").unwrap();

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .unwrap();

    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.to_owned().dyn_into().unwrap();
    console::log!(resp_value.clone());

    let array_buffer_promise = resp.array_buffer().unwrap();
    let array_buffer = JsFuture::from(array_buffer_promise).await.unwrap();
    let array_buffer: ArrayBuffer = array_buffer.dyn_into().unwrap();

    unsafe {
        ARRAY_BUFFERS[asset_index] = Some(array_buffer.to_owned());
    }
    return array_buffer;
}

#[allow(unused)]
pub fn spawn_load_array_buffers() {
    spawn_local(async {
        for asset_index in 0..NUM_ASSETS {
            get_array_buffer(asset_index).await;
        }
    })
}

pub async fn get_audio_buffer(asset_index: usize) -> AudioBuffer {
    unsafe {
        if AUDIO_BUFFERS[asset_index].is_some() {
            return AUDIO_BUFFERS[asset_index].to_owned().unwrap();
        }
    }
    let context = AudioContext::new().unwrap();
    let array_buffer = get_array_buffer(asset_index).await;
    let buffer = context.decode_audio_data(&array_buffer).unwrap();
    let buffer = JsFuture::from(buffer);
    let buffer = buffer.await;
    let buffer = buffer.unwrap();
    let buffer: AudioBuffer = buffer.dyn_into().unwrap();
    unsafe {
        AUDIO_BUFFERS[asset_index] = Some(buffer.to_owned());
    }
    return buffer;
}

#[allow(unused)]
pub fn get_audio_buffer_sync(asset_index: usize) -> Option<AudioBuffer> {
    unsafe { AUDIO_BUFFERS[asset_index].to_owned() }
}

pub async fn load_audio_buffers() {
    for asset_index in 0..NUM_ASSETS {
        get_audio_buffer(asset_index).await;
    }
}

pub fn spawn_load_audio_buffers() {
    spawn_local(async {
        load_audio_buffers().await;
    })
}

pub fn get_buffer(index: usize) -> Option<AudioBuffer> {
    unsafe {
        return AUDIO_BUFFERS[index].to_owned();
    }
}
