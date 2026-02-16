// Day 73: Project: Weather Dashboard (Yew + External API)
// Build a simple weather dashboard in Yew that fetches real-time data from a public
// weather API (e.g., OpenWeatherMap). This project teaches API integration, conditional
// rendering, and handling user input in Yew.

// Key Concepts:
// + API key management
// + use_state for dynamic input
// + use_effect_with_deps for reactive HTTP calls
// + Handling Option<T> in the UI
// You now have a real-time weather app—a powerful example of external API integration 
// and dynamic frontend interactivity in Rust.

// Import Yew's component and HTML macros (html!, use_state, Callback, etc.).
use yew::prelude::*;
// Import gloo's HTTP client for making fetch requests from the browser.
use gloo_net::http::Request;
// Import serde's Deserialize so we can parse JSON API responses into Rust structs.
use serde::Deserialize;

// Struct that matches the JSON shape returned by OpenWeatherMap's current weather API.
#[derive(Deserialize, Debug, Clone, PartialEq)]
struct WeatherData {
    // City name returned by the API.
    name: String,
    // Nested object with temp, humidity, etc.
    main: Main,
    // Array of weather condition objects (description, icon, etc.).
    weather: Vec<Weather>,
}

// Nested struct for the "main" object in the API response.
#[derive(Deserialize, Debug, Clone, PartialEq)]
struct Main {
    // Temperature in the units we requested (e.g. metric = Celsius).
    temp: f64,
    // Humidity percentage (0–100).
    humidity: u8,
}

// One weather condition entry (e.g. "clear sky", "light rain").
#[derive(Deserialize, Debug, Clone, PartialEq)]
struct Weather {
    // Human-readable description of the weather.
    description: String,
}

// Declare the root component as a function component (no struct + impl).
#[function_component(App)]
fn app() -> Html {
    // State: which city we're showing weather for (used for the API request).
    let city = use_state(|| "London".to_string());
    // State: the fetched weather data, or None while loading / before first fetch.
    let weather = use_state(|| None::<WeatherData>);
    // State: the current value in the text input (may not match `city` until user clicks the button).
    let input = use_state(|| city.to_string());

    // Clone the input state handle so we can use it inside the callback.
    let on_input = {
        let input = input.clone();
        // Callback fired on every keystroke in the input.
        Callback::from(move |e: InputEvent| {
            // Get the DOM input element and read its current value.
            let val = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
            // Update our input state so the field stays in sync.
            input.set(val);
        })
    };

    // Clone state handles for use inside the click callback.
    let on_click = {
        let city = city.clone();
        let input = input.clone();
        // Callback fired when the user clicks "Get Weather".
        Callback::from(move |_| city.set((*input).clone()))
    };

    // Run a side effect whenever `city` changes (including initial mount).
    {
        let weather = weather.clone();
        let city = city.clone();
        // use_effect_with(deps, closure): closure runs when deps change; closure returns a cleanup (here: || ()).
        use_effect_with(
            // Dependency: when this value changes, we re-run the effect.
            (*city).clone(),
            move |city_name| {
                // Copy the city name into the async block.
                let city = city_name.clone();
                // Spawn a local async task (runs on the same thread; no separate runtime).
                yew::platform::spawn_local(async move {
                    // Build the OpenWeatherMap URL with city, metric units, and API key.
                    let url = format!(
                        "https://api.openweathermap.org/data/2.5/weather?q={}&units=metric&appid=0907cae62757d6018399b160055e212e",
                        city
                    );
                    // Send GET request; if it succeeds, try to parse JSON.
                    if let Ok(res) = Request::get(&url).send().await {
                        if let Ok(data) = res.json::<WeatherData>().await {
                            // Store the parsed weather data so the UI can render it.
                            weather.set(Some(data));
                        }
                    }
                });
                // No cleanup needed; return a no-op.
                || ()
            },
        );
    }

    // Return the component's HTML (JSX-like syntax in Rust).
    html! {
        // Outer container: centered, with some top padding.
        <div style="font-family: sans-serif; text-align: center; padding-top: 2em;">
            // Page title.
            <h1>{ "🌦️ Weather Dashboard" }</h1>
            // Text input: value bound to `input` state, updates via on_input.
            <input value={(*input).clone()} oninput={on_input} placeholder="Enter city..." />
            // Button: clicking triggers on_click, which sets `city` from `input`.
            <button onclick={on_click}>{ "Get Weather" }</button>

            // Conditional block: render different content based on whether we have weather data.
            {
                if let Some(data) = &*weather {
                    // We have data: show city name, temp, humidity, and description.
                    html! {
                        <div style="margin-top: 2em;">
                            <h2>{ format!("Weather in {}", data.name) }</h2>
                            <p>{ format!("🌡️ Temp: {:.1}°C", data.main.temp) }</p>
                            <p>{ format!("💧 Humidity: {}%", data.main.humidity) }</p>
                            <p>{ format!("🌤️ Description: {}", data.weather[0].description) }</p>
                        </div>
                    }
                } else {
                    // No data yet: show a loading / placeholder message.
                    html! { <p>{ "🔄 Loading or no data yet..." }</p> }
                }
            }
        </div>
    }
}

// Entry point: runs when the Wasm module loads in the browser.
fn main() {
    // Create a renderer for the App component and mount it to the document body.
    yew::Renderer::<App>::new().render();
}