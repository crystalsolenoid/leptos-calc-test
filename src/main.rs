#![feature(int_roundings)]

use std::num::ParseIntError;

use leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
    let (quant, set_quant) = signal("".to_string());
    let (percent, set_percent) = signal(10u64);
    let (custom_percent, set_custom_percent) = signal(false);

    let quant_parsed = move || {
        quant.get().parse::<u64>()
    };

    let over_thresh = move || {
        quant_parsed()
            .map(|q: u64| q > 500)
    };

    let ten_percent = move || {
        quant_parsed()
            .map(|q: u64| q + (percent.get() * q).div_ceil(100) + 1)
    };

    view! {
        <p>
            <input
                id="quant"
                type="number"
                on:input:target=move |ev| {
                    set_quant.set(ev.target().value());
                    if !custom_percent.get() {
                        if over_thresh().unwrap_or(false) {
                            set_percent.set(20)
                        } else {
                            set_percent.set(10)
                        }
                    }
                }
                prop:value=quant
            />
            " + "
            <PercentSelector
                setter=set_percent
                getter=percent
                custom=set_custom_percent
                thresh=Signal::derive(over_thresh)
            />
            " + 1 = "
            <ErrorBoundary fallback=|errors| view! { <span class="error">"???"</span> }>
                <strong>{ten_percent}</strong>
            </ErrorBoundary>
        </p>
        <p>
            <button on:click=move |_| {
                set_quant.set("".to_string());
                set_custom_percent.set(false);
                set_percent.set(10);
            }>"clear"</button>
        </p>
    }
}

#[component]
fn PercentSelector(setter: WriteSignal<u64>, getter: ReadSignal<u64>,
    custom: WriteSignal<bool>,
    #[prop(into)]
    thresh: Signal<Result<bool, ParseIntError>>) -> impl IntoView {
    view! {
        <select
            on:change:target=move |ev| {
                setter.set(ev.target().value().parse().unwrap());
                custom.set(true);
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
