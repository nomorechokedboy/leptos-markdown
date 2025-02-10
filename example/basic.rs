use leptos::*;
use leptos_markdown::Markdown;

fn main() {
// Initialize logging for development
#[cfg(debug_assertions)]
{
_ = console_log::init_with_level(log::Level::Debug);
console_error_panic_hook::set_once();
}

// Example usage
let (markdown_content, set_markdown_content) = create_signal(
"# Hello, Markdown!\n\nThis is a **bold** text.".to_string(),
);

mount_to_body(move || {
view! {
<div>
    <Markdown children=markdown_content />
    <textarea on:input=move |ev| { set_markdown_content(event_target_value(&ev)); } rows="4" cols="50"
        prop:value=markdown_content>
    </textarea>
</div>
}
})
}
