use gloo::console;
use js_sys::ArrayBuffer;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen_futures::JsFuture;
use web_sys::AudioBuffer;
use web_sys::AudioContext;
use web_sys::{Request, RequestInit, RequestMode, Response};


const NUM_ASSETS: usize = 1;
// const NAMES: [&str; NUM_ASSETS] = ["music"];
const URLS: [&str; NUM_ASSETS] = ["audio/music-a-115bpm.webm"];
static mut BUFFERS: [Option<AudioBuffer>; NUM_ASSETS] = [None];

pub async fn fetch_array_buffer(url: &str) -> ArrayBuffer {
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
    return array_buffer.to_owned();
}

pub fn load() {
    spawn_local(async {
        let context = AudioContext::new().unwrap();

        for i in 0..NUM_ASSETS {
            let array_buffer = fetch_array_buffer(URLS[i]).await;
            let buffer = context.decode_audio_data(&array_buffer).unwrap();
            let buffer = JsFuture::from(buffer);
            let buffer = buffer.await;
            let buffer = buffer.unwrap();
            let buffer: AudioBuffer = buffer.dyn_into().unwrap();
            unsafe {
                BUFFERS[i] = Some(buffer);
            }
        }
    })
}

pub fn get_buffer(index: usize) -> Option<AudioBuffer> {
    unsafe {
        return BUFFERS[index].to_owned();
    }
}
