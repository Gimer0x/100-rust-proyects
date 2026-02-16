use wasm_bindgen::prelude::*;
use yew::prelude::*;
 
#[function_component(App)]
fn app() -> Html {
    let counter = use_state(|| 0);
 
    let increment = {
        let counter = counter.clone();
        Callback::from(move |_| counter.set(*counter + 1))
    };
 
    html! {
        <div style="font-family: sans-serif; text-align: center; margin-top: 50px;">
            <h1>{ "👋 Hello from Yew!" }</h1>
            <p>{ format!("You clicked {} times.", *counter) }</p>
            <button onclick={increment}>{ "Click me!" }</button>
        </div>
    }
}

/// Entry point: run when the Wasm module loads (called by Trunk).
#[wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<App>::new().render();
}