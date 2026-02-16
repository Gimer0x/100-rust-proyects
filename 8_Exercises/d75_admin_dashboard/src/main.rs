// Day 75: Project: Admin Dashboard UI with Yew
// Build a clean and reactive Admin Dashboard UI using Yew, displaying mock data
// like total users, sales, and orders. You'll learn about components, layouts,
// and charts integration (optional), ideal for internal tools or management panels.
// Key Concepts:
// + Dashboard layout using flexbox (inline CSS)
// + Simulated async data with Timeout
// + children props for flexible components
// You've created a dashboard UI in Yew — perfect for internal tools, admin panels,
// and monitoring systems. You now know:
// + State-driven design
// + Component reuse
// + WASM-ready dashboard interfaces
use yew::prelude::*;
use gloo_timers::callback::Timeout;

#[derive(Clone, PartialEq)]
struct Metrics {
    users: u32,
    orders: u32,
    revenue: f64,
}

// Props for MetricCard: accepts children (label and value as two child nodes).
#[derive(Properties, Clone, PartialEq)]
struct MetricCardProps {
    pub children: Children,
}

#[function_component(MetricCard)]
fn metric_card(props: &MetricCardProps) -> Html {
    let label = props.children.iter().next().unwrap();
    let value = props.children.iter().nth(1).unwrap();

    html! {
        <div style="padding: 1em; border-radius: 8px; background: #f4f4f4; width: 30%; margin: 1%;">
            <h3>{ label }</h3>
            <p style="font-size: 1.4em; font-weight: bold;">{ value }</p>
        </div>
    }
}

#[function_component(App)]
fn app() -> Html {
    let data = use_state(|| Metrics {
        users: 0,
        orders: 0,
        revenue: 0.0,
    });

    {
        let data = data.clone();
        use_effect(move || {
            Timeout::new(1000, move || {
                data.set(Metrics {
                    users: 1324,
                    orders: 768,
                    revenue: 125430.75,
                });
            })
            .forget();
            || ()
        });
    }

    html! {
        <div style="font-family: sans-serif; padding: 2em;">
            <h1>{ "📊 Admin Dashboard" }</h1>
            <div style="display: flex; justify-content: space-between;">
                <MetricCard>
                    { "Users" } { format!("{}", data.users) }
                </MetricCard>
                <MetricCard>
                    { "Orders" } { format!("{}", data.orders) }
                </MetricCard>
                <MetricCard>
                    { "Revenue" } { format!("${:.2}", data.revenue) }
                </MetricCard>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
