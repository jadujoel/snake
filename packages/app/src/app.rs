use gloo::console;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let name = "Brookzerker";
    console::log!("Hello from the console!");
    console::log!(name, "is a cool name!");
    html! {
        <h1>{ "Hello World!" }</h1>
    }
}
