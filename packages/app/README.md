web-worker: https://github.com/yewstack/yew/blob/yew-v0.19.3/examples/web_worker_fib/src/agent.rs


```rust
use crate::{JsValue, ui};
use crate::{game::World, utils::random};
use gloo::console;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;
use yew::prelude::*;



#[function_component(App)]
pub fn app() -> Html {
    console::log!("Hello from the console!");
    let fps = 2;
    let ms = 1000 / fps;
    let width = 4;
    let cellSize = 800 / width;
    let height = width;

    let startIndex = random(0, width * height - 1);
    let world = World::new(width, height, startIndex);

    let document =

    let canvas = document
        .create_element("canvas").unwrap()
        .dyn_into::<HtmlCanvasElement>().unwrap();

    // let canvas = document.get_element_by_id("snake-canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    canvas.set_width(width as u32 * cellSize as u32);
    canvas.set_height(height as u32 * cellSize as u32);

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    fn draw_cell(context: &web_sys::CanvasRenderingContext2d, x: u32, y: u32, cellSize: u32) {
        context.begin_path();
        context.rect(x as f64, y as f64, cellSize as f64, cellSize as f64);
        context.fill();
    }

    fn drawBoard(ctx: &web_sys::CanvasRenderingContext2d, width: usize, height: usize, cellSize: usize) {
        ctx.begin_path();
        ctx.set_fill_style(&JsValue::from_str("white"));
        ctx.set_line_width(1.0);
        for x in 0..width + 1{
            ctx.move_to((cellSize * x) as f64, 0.0);
            ctx.line_to((cellSize * x) as f64, (cellSize * width) as f64);
        }
        for y in 0..height + 1 {
            ctx.move_to(0.0, (cellSize * y) as f64);
            ctx.line_to((cellSize * height) as f64, (cellSize * y) as f64);
        }
        ctx.stroke()
    }
    drawBoard(&context, width, height, cellSize);

    // let snake_canvas = ui::SnakeUI::create();


    html! (
        <div class="inactive" id="snake-game">
            <div id="snake-info" class="show">
                <div></div>
                <p id="snake-win" class="hide">{"You won, well played!"}</p>
                <div class="snake-score">
                    <label>{"Score:"}</label>
                </div>
                <p>{"space to play"}</p>

            </div>
        </div>
    )
}




```




web-worker
https://github.com/yewstack/yew/blob/yew-v0.19.3/examples/web_worker_fib/src/agent.rs

## Web-sys
https://rustwasm.github.io/wasm-bindgen/api/web_sys/

## Trunk

```bash
cargo install --locked trunk
cargo install wasm-bindgen-cli

cargo add yew

trunk serve
trunk build --release
```


```rust
// main.rs
use yew::prelude::*;

enum Msg {
    AddOne,
}

struct Model {
    value: i64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value: 0,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        html! {
            <div>
                <button onclick={link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
                <p>{ self.value }</p>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}

```

## wasm-pack
```toml
<!-- Cargo.toml -->
[lib]
crate-type = ["rlib", "cdylib"]
```
```bash
cargo install wasm-pack
wasm-pack build --target web
rollup ./main.js --format iife --file ./pkg/bundle.js
http-server # or your other preferred server
```
