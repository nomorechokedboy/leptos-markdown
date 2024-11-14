mod helper;
mod img_handling_test;
mod link_handling_test;
mod mdast_to_leptos;
mod mdast_to_leptos_test;

use helper::clean_leptos_markup;
use leptos::*;
use mdast_to_leptos::render_node;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    let (s, set_s) = create_signal(
        "# Hello, Markdown!\n\nThis is a **bold** text. <h1 style=\"text-align: center\">Lmao</h1>"
            .to_string(),
    );
    /* println!(
    "{:?}",
    markdown::to_mdast(
    "#
    <HelloMessage />, {username}!\n\n<h1 style=\"text-align: center\">Lmao</h1>",
    &markdown::ParseOptions::mdx()
    )
    .expect("Lmao err")
    ); */
    let node = markdown::to_mdast(
        // "# Hello, Markdown!\n\nThis is a **bold** text. <h1>Lmao</h1>",
        r#"Vue is awesome
And so is markdown

Combining = epic"#,
        &markdown::ParseOptions::default(),
    )
    .expect("Lmao err");

    mount_to_body(move || {
        view! {
            <Markdown children=s />
            <textarea
                on:change=move |ev| {
                    set_s(event_target_value(&ev));
                }
                rows="4"
                cols="50"
            >
                {s}
            </textarea>
        }
    })
}

#[component]
pub fn Markdown(#[prop(into)] children: Signal<String>) -> impl IntoView {
    let node =
        move || markdown::to_mdast(&children(), &markdown::ParseOptions::mdx()).expect("Lmao err");
    let md_content = move || render_node(&node());
    view! { <div class="markdown-content">{md_content}</div> }
}
