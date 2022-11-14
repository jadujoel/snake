
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
