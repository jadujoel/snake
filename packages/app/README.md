'tis a simple snake game built in rust using yew

## Todo:
* Use reqwest to fetch data instead of web_sys bindings
* Rendering
    * render using WebGL instead of canvas
* Audio System
    * move into separate package
    * run in separate thread
    * embedd assets into one file
* Controls
    * add touch controls for mobile device
## Notes:
web-worker: https://github.com/yewstack/yew/blob/yew-v0.19.3/examples/web_worker_fib/src/agent.rs
build gui using webgl: https://www.egui.rs/
embedd raw data: https://nickb.dev/blog/a-quick-tour-of-trade-offs-embedding-data-in-rust/
openAL audio:

go from typescript to rust:
* https://github.com/dlunch/typescript-wasm-bindgen
* https://rustwasm.github.io/wasm-bindgen/reference/attributes/on-rust-exports/typescript_type.html

```bash
# show the file size of app.js in kilobytes
SIZE=$(cat pkg/app_bg.wasm | base64 | wc -c)
echo "size: $(cat pkg/app_bg.wasm | base64 | wc -c) bytes"

```
