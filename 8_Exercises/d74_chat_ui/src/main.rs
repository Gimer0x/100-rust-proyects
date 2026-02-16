// Day 74: Project: Chat Frontend UI with Yew
// Build a responsive chat frontend in Yew that connects to a WebSocket backend (like the one from Day 54).
// This project focuses on bi-directional real-time communication, message lists, and input handling.
//
// Key Concepts:
// + WebSocket API in web_sys
// + use_mut_ref to persist connection
// + Closure to handle JavaScript events in WASM
// + Reactive updates with use_state
// You now have a real-time chat interface entirely in Rust—powerful for live dashboards, 
// multiplayer games, or collaborative tools.

// Import Yew's component and HTML macros (html!, use_state, Callback, etc.).
use yew::prelude::*;
// Import wasm_bindgen for JS interop (e.g. Closure).
use wasm_bindgen::prelude::*;
// Import WebSocket and MessageEvent from the browser's WebSocket API.
use web_sys::{MessageEvent, WebSocket};

// Declare the root component as a function component.
#[function_component(App)]
fn app() -> Html {
    // State: list of chat messages to display (each is a String).
    let messages = use_state(|| vec![]);
    // State: current value of the text input (what the user is typing).
    let input = use_state(|| String::new());
    // Mutable ref that holds the WebSocket instance so we can send messages and keep the connection alive.
    let ws_ref = use_mut_ref(|| None::<WebSocket>);

    // Clone the messages handle so we can use it inside the callback.
    let append_msg = {
        let messages = messages.clone();
        // Callback that appends a new message to the list and updates state.
        Callback::from(move |msg: String| {
            // Clone current messages, push the new one, then set state.
            let mut new = (*messages).clone();
            new.push(msg);
            messages.set(new);
        })
    };

    // Run a side effect once when the component mounts (connect WebSocket and set up listener).
    {
        let append_msg = append_msg.clone();
        let ws_ref = ws_ref.clone();
        // use_effect with no deps: runs once on mount; closure returns cleanup (here: no-op).
        use_effect(move || {
            // Create a new WebSocket connection to the backend (panics if URL invalid or connection fails).
            let ws = WebSocket::new("ws://localhost:9001").expect("Failed to connect to WebSocket");
            // Create a Rust closure that will be called by the browser when a message is received.
            let onmessage = Closure::<dyn FnMut(_)>::wrap(Box::new(move |e: MessageEvent| {
                // e.data() can be string or blob; as_string() returns Option<String>.
                if let Some(txt) = e.data().as_string() {
                    // Push the received message into our messages state so the UI updates.
                    append_msg.emit(txt);
                }
            }) as _);

            // Register the closure as the WebSocket's onmessage handler (unchecked_ref for JS compatibility).
            ws.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
            // Prevent the closure from being dropped so the JS handler stays valid.
            onmessage.forget();
            // Store the WebSocket in our ref so we can call send later (e.g. when user clicks Send).
            *ws_ref.borrow_mut() = Some(ws);
            // No cleanup: return a no-op. (In a real app you might close the socket here.)
            || ()
        });
    }

    // Clone input state for use inside the callback.
    let oninput = {
        let input = input.clone();
        // Callback fired on every keystroke in the input field.
        Callback::from(move |e: InputEvent| {
            // Read the current value from the DOM input element.
            let val = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
            // Keep our state in sync with the input.
            input.set(val);
        })
    };

    // Clone input and ws_ref so we can send the message and clear the input.
    let onclick = {
        let input = input.clone();
        let ws_ref = ws_ref.clone();
        // Callback fired when the user clicks the Send button.
        Callback::from(move |_| {
            // If we have a WebSocket, send the current input text to the server.
            if let Some(ws) = &*ws_ref.borrow() {
                let _ = ws.send_with_str(&input);
            }
            // Clear the input field after sending.
            input.set(String::new());
        })
    };

    // Return the component's HTML.
    html! {
        // Outer container: centered, max width, some margin.
        <div style="font-family: sans-serif; max-width: 500px; margin: 3em auto;">
            // Page title.
            <h1>{ "💬 Yew Chat Client" }</h1>
            // Scrollable area that displays the list of messages.
            <div style="border: 1px solid #ccc; padding: 1em; height: 300px; overflow-y: scroll;">
                // Iterate over messages and render each as a paragraph.
                { for messages.iter().map(|m| html! {
                    <p>{ m }</p>
                })}
            </div>
            // Text input: value bound to state, updates via oninput.
            <input style="width: 70%;" value={(*input).clone()} oninput={oninput} />
            // Send button: clicking runs onclick, which sends the message and clears the input.
            <button onclick={onclick} style="width: 28%; margin-left: 2%;">{ "Send" }</button>
        </div>
    }
}

// Entry point: runs when the Wasm module loads in the browser.
fn main() {
    // Create a renderer for the App component and mount it to the document body.
    yew::Renderer::<App>::new().render();
}
