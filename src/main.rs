#![feature(int_roundings)]

use leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
    let (quant, set_quant) = signal("".to_string());
    let (value, set_value) = signal(10u64);

    let ten_percent = move || {
        quant
            .get()
            .parse::<u64>()
            // TODO this panics when q is too high
            .map(|q: u64| q + (value.get() * q).div_ceil(100) + 1)
    };

    view! {
        <p>
            <input
                id="quant"
                type="number"
                on:input:target=move |ev| { set_quant.set(ev.target().value()) }
                prop:value=quant
            />
            " + "
            <PercentSelector setter=set_value getter=value />
            " + 1 = "
            <ErrorBoundary fallback=|errors| view! { <span class="error">"???"</span> }>
                <strong>{ten_percent}</strong>
            </ErrorBoundary>
        </p>
        <p>
            <button on:click=move |_| { set_quant.set("".to_string()) }>"clear"</button>
        </p>
    }
}

#[component]
fn PercentSelector(setter: WriteSignal<u64>, getter: ReadSignal<u64>) -> impl IntoView {
    view! {
        <select
            on:change:target=move |ev| {
                setter.set(ev.target().value().parse().unwrap());
            }
            prop:value=move || getter.get().to_string()
        >
            <option value="10">"10%"</option>
            <option value="15">"15%"</option>
            <option value="20">"20%"</option>
        </select>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App)
}
