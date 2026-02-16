// Day 72: Project: Frontend Todo List (Yew + REST API)
// Create a Yew-based frontend that connects to your Actix-Web backend to fetch and display todos
// via HTTP. You'll learn to:
// + Make async REST API calls
// + Render dynamic lists
// + Handle side effects with use_effect_with

// Import gloo-net for HTTP requests from the browser (fetch API wrapper).
use gloo_net::http::Request;
// Import Serde for serializing and deserializing JSON (API request/response).
use serde::{Deserialize, Serialize};
// Import Yew prelude: components, hooks (use_state, use_effect_with), html!, etc.
use yew::prelude::*;

// Derive common traits so Todo can be cloned, (de)serialized, compared, and debug-printed.
#[derive(Clone, Deserialize, Serialize, PartialEq, Debug)]
// Struct that matches the JSON shape returned by GET /todos.
struct Todo {
    // Unique id from the backend.
    id: i32,
    // Todo title text.
    title: String,
    // Whether the todo is done.
    completed: bool,
}

// Register this function as a Yew function component named App (used in the root render).
#[function_component(App)]
// Root component: fetches todos on mount and renders a list or loading state.
fn app() -> Html {
    // State: list of todos; initial value is empty vec; set with todos.set(...).
    let todos = use_state(|| vec![]);
    // State: true while the fetch is in progress; false when done (success or error).
    let loading = use_state(|| true);

    // Block to scope the clones and run the effect once on mount.
    {
        // Clone handles so we can move them into the async block and the effect closure.
        let todos = todos.clone();
        let loading = loading.clone();
        // Run a side effect when the component mounts; deps = () so it runs only once.
        use_effect_with((), move |_| {
            // Spawn an async task on the browser executor (so we can .await inside the effect).
            yew::platform::spawn_local(async move {
                // Send GET request to the backend; await the response.
                let res = match Request::get("http://localhost:8080/todos").send().await {
                    // Success: we have the HTTP response.
                    Ok(r) => r,
                    // Network or server error: log and stop loading.
                    Err(_) => {
                        gloo_console::error!("Failed to fetch todos");
                        loading.set(false);
                        return;
                    }
                };
                // Parse the response body as JSON into Vec<Todo>; .json() returns a Future, so we await.
                match res.json::<Vec<Todo>>().await {
                    // Success: update state with the list of todos.
                    Ok(data) => {
                        todos.set(data);
                    }
                    // Invalid JSON or wrong shape: log error (loading still set false below).
                    Err(_) => {
                        gloo_console::error!("Failed to parse todos");
                    }
                }
                // Mark loading as complete so the UI shows the list (or empty) instead of "Loading...".
                loading.set(false);
            });
            // Cleanup function: run when the component unmounts; we have nothing to cancel, so return no-op.
            || ()
        });
    }

    // Return the HTML for this component (Yew's html! macro).
    html! {
        // Outer container with basic styling.
        <div style="font-family: sans-serif; padding: 2em;">
            // Page title.
            <h1>{ "Yew Todo App" }</h1>
            // If still loading, show a message.
            if *loading {
                <p>{ "Loading..." }</p>
            } else {
                // Otherwise show an unordered list of todos.
                <ul>
                    // Iterate over todos and render each as a list item; key helps Yew reconcile the list.
                    { for todos.iter().map(|todo| html! {
                        <li key={todo.id}>
                            // Show title and a check or cross depending on completed.
                            { format!("{} - {}", todo.title, if todo.completed { "✅" } else { "❌" }) }
                        </li>
                    })}
                </ul>
            }
        </div>
    }
}

// Entry point: runs when the Wasm module loads in the browser.
fn main() {
    // Create a renderer for the App component and mount it to the document body.
    yew::Renderer::<App>::new().render();
}
