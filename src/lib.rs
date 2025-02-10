use leptos::*;
use wasm_bindgen::prelude::*;

mod helper;
mod mdast_to_leptos;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

/// Main component for rendering Markdown content
#[component]
pub fn Markdown(
    /// The markdown content as a string signal
    #[prop(into)]
    children: Signal<String>,
    /// Optional class name for the wrapper div
    #[prop(optional)]
    class: Option<String>,
) -> impl IntoView {
    let node = move || {
        markdown::to_mdast(&children(), &markdown::ParseOptions::mdx())
            .expect("Failed to parse markdown")
    };
    let md_content = move || mdast_to_leptos::render_node(&node());

    view! {
        <div class=move || {
            format!("markdown-content {}", class.clone().unwrap_or_default()).trim().to_string()
        }>{md_content}</div>
    }
}
