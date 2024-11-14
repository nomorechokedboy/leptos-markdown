use leptos::*;
use markdown::mdast::{
    self, AttributeValue, Code, Emphasis, Heading, Image, Link, MdxJsxTextElement, Paragraph,
    Strong, Table, TableCell, TableRow,
};
use once_cell::sync::Lazy;
use syntect::highlighting::ThemeSet;
use syntect::html::{css_for_theme_with_class_style, ClassStyle, ClassedHTMLGenerator};
use syntect::parsing::SyntaxSet;

static SYNTAX_SET: Lazy<SyntaxSet> = Lazy::new(|| SyntaxSet::load_defaults_newlines());
static THEME_SET: Lazy<ThemeSet> = Lazy::new(ThemeSet::load_defaults);
static SYNTAX_CSS: Lazy<String> = Lazy::new(|| {
    let theme = &THEME_SET.themes["InspiredGitHub"];
    css_for_theme_with_class_style(theme, ClassStyle::Spaced).unwrap()
});

// Component to inject syntax highlighting CSS
#[component]
pub fn SyntaxHighlightingStyle() -> impl IntoView {
    view! { <style>{SYNTAX_CSS.clone()}</style> }
}

pub fn render_node(node: &mdast::Node) -> View {
    match node {
        mdast::Node::Root(root) => root
            .children
            .iter()
            .map(render_node)
            .collect::<Vec<_>>()
            .into_view(),
        mdast::Node::Blockquote(block_quote) => view! {
            <blockquote>
                {block_quote.children.iter().map(render_node).collect::<Vec<_>>()}
            </blockquote>
        }
        .into_view(),
        /* mdast::Node::FootnoteDefinition(_) => todo!(),
        mdast::Node::MdxJsxFlowElement(_) => todo!(),
        mdast::Node::List(_) => todo!(),
        mdast::Node::MdxjsEsm(_) => todo!(),
        mdast::Node::Toml(_) => todo!(),
        mdast::Node::Yaml(_) => todo!(), */
        mdast::Node::Break(_) => view! { <br /> }.into_view(),
        mdast::Node::InlineCode(code) => view! { <code>{&code.value}</code> }.into_view(),
        // mdast::Node::InlineMath(_) => todo!(),
        mdast::Node::Delete(_) => todo!(),
        mdast::Node::Emphasis(Emphasis { children, .. }) => {
            view! { <em>{children.iter().map(render_node).collect::<Vec<_>>()}</em> }.into_view()
        }
        /* mdast::Node::MdxTextExpression(_) => todo!(),
        mdast::Node::FootnoteReference(_) => todo!(),
        mdast::Node::Html(_) => todo!(), */
        mdast::Node::Image(Image {
            title, url, alt, ..
        }) => {
            let url_owned = url.clone();
            let alt_owned = alt.clone();
            let title_owned = title.clone();

            let img_view = move || {
                if title_owned.is_some() {
                    view! {
                        <img
                            src=url_owned.clone()
                            alt=alt_owned.clone()
                            title=title_owned.clone().unwrap_or_default()
                        />
                    }
                    .into_view()
                } else {
                    view! { <img src=url_owned.clone() alt=alt_owned.clone() /> }.into_view()
                }
            };

            img_view().into_view()
        }
        mdast::Node::ImageReference(_) => todo!(),
        mdast::Node::MdxJsxTextElement(MdxJsxTextElement {
            name,
            children,
            attributes,
            ..
        }) => {
            let mut attrs: Vec<mdast::MdxJsxAttribute> = vec![];
            for attribute in attributes {
                match attribute {
                    mdast::AttributeContent::Expression(_) => todo!(),
                    mdast::AttributeContent::Property(prop) => attrs.push(prop.clone()),
                }
            }

            let el = create_element(
                &attrs,
                children,
                &name.clone().unwrap_or(String::from("span")),
            );
            el.into_view()
        }
        mdast::Node::Link(Link {
            children,
            url,
            title,
            ..
        }) => {
            let url_owned = url.clone();
            let title_owned = title.clone();
            let link_view = move || {
                if title_owned.is_some() {
                    view! {
                        <a href=url_owned.clone() title=title_owned.clone()>
                            {children.iter().map(render_node).collect::<Vec<_>>()}
                        </a>
                    }
                } else {
                    view! {
                        <a href=url_owned.clone() title=title_owned.clone()>
                            {children.iter().map(render_node).collect::<Vec<_>>()}
                        </a>
                    }
                }
            };

            link_view().into_view()
        }
        // mdast::Node::LinkReference(_) => todo!(),
        mdast::Node::Strong(Strong { children, .. }) => {
            view! { <strong>{children.iter().map(render_node).collect::<Vec<_>>()}</strong> }
                .into_view()
        }
        mdast::Node::Text(text) => text.value.clone().into_view(),
        mdast::Node::Code(Code {
            value, lang, meta, ..
        }) => {
            // Get syntax reference based on language
            let syntax_ref = lang
                .as_ref()
                .and_then(|lang| SYNTAX_SET.find_syntax_by_extension(lang))
                .or_else(|| Some(SYNTAX_SET.find_syntax_plain_text()))
                .unwrap();

            let mut html_generator = ClassedHTMLGenerator::new_with_class_style(
                syntax_ref,
                &SYNTAX_SET,
                ClassStyle::Spaced,
            );

            for line in value.lines() {
                html_generator
                    .parse_html_for_line_which_includes_newline(&format!("{line}\n"))
                    .unwrap();
            }

            let highlighted_html = html_generator.finalize();

            view! {
                        <div class="relative">
                            // Language badge if available
                            {lang
                                .clone()
                                .map(|lang| {
                                    view! {
                                        <div class="absolute top-2 right-2 px-2 py-1 text-sm rounded bg-gray-700 text-gray-300">
                                            {lang}
                                        </div>
                                    }
                                })}
                            <pre class="p-4 rounded-lg bg-gray-900 text-gray-100 overflow-x-auto">
                                <code
                                    class="block"
                                    data-language=lang.clone()
                                    data-meta=meta.clone()
                                    inner_html=highlighted_html
                                />
                            </pre>
                        </div>
                    }.into_view()
        }
        /* mdast::Node::Math(_) => todo!(),
        mdast::Node::MdxFlowExpression(_) => todo!(), */
        mdast::Node::Heading(Heading {
            depth, children, ..
        }) => match depth {
            1 => view! { <h1>{children.iter().map(render_node).collect::<Vec<_>>()}</h1> }
                .into_view(),
            2 => view! { <h2>{children.iter().map(render_node).collect::<Vec<_>>()}</h2> }
                .into_view(),
            3 => view! { <h3>{children.iter().map(render_node).collect::<Vec<_>>()}</h3> }
                .into_view(),
            4 => view! { <h4>{children.iter().map(render_node).collect::<Vec<_>>()}</h4> }
                .into_view(),
            5 => view! { <h5>{children.iter().map(render_node).collect::<Vec<_>>()}</h5> }
                .into_view(),
            _ => view! { <h6>{children.iter().map(render_node).collect::<Vec<_>>()}</h6> }
                .into_view(),
        },
        mdast::Node::Table(Table {
            children, align: _, ..
        }) => view! {
            <table style="text-align: initial;">
                {children.iter().map(render_node).collect::<Vec<_>>()}
            </table>
        }
        .into_view(),
        mdast::Node::ThematicBreak(_) => todo!(),
        mdast::Node::TableRow(TableRow { children, .. }) => {
            view! { <tr>{children.iter().map(render_node).collect::<Vec<_>>()}</tr> }.into_view()
        }
        mdast::Node::TableCell(TableCell { children, .. }) => {
            view! { <td>{children.iter().map(render_node).collect::<Vec<_>>()}</td> }.into_view()
        }
        mdast::Node::ListItem(_) => todo!(),
        // mdast::Node::Definition(_) => todo!(),
        mdast::Node::Paragraph(Paragraph { children, .. }) => {
            view! { <p>{children.iter().map(render_node).collect::<Vec<_>>()}</p> }.into_view()
        }
        _ => view! { <span>Unsupported node type</span> }.into_view(),
    }
}

fn create_element(
    attributes: &[mdast::MdxJsxAttribute],
    children: &[mdast::Node],
    name: &str,
) -> View {
    match name.to_lowercase().as_str() {
        // Handle common HTML elements
        "div" => view! {
            <div class=get_class_from_attributes(
                attributes,
            )>{children.iter().map(render_node).collect::<Vec<_>>()}</div>
        }
        .into_view(),

        "span" => view! {
            <span class=get_class_from_attributes(
                attributes,
            )>{children.iter().map(render_node).collect::<Vec<_>>()}</span>
        }
        .into_view(),

        "p" => view! {
            <p class=get_class_from_attributes(
                attributes,
            )>{children.iter().map(render_node).collect::<Vec<_>>()}</p>
        }
        .into_view(),

        // Add support for custom components or special elements
        "custom-alert" => view! {
            <div class="custom-alert">
                // <div class="alert-icon">⚠️</div>
                <div class="alert-content">
                    {children.iter().map(render_node).collect::<Vec<_>>()}
                </div>
            </div>
        }
        .into_view(),

        // Add support for semantic elements
        "article" => view! {
            <article class=get_class_from_attributes(
                attributes,
            )>{children.iter().map(render_node).collect::<Vec<_>>()}</article>
        }
        .into_view(),

        "section" => view! {
            <section class=get_class_from_attributes(
                attributes,
            )>{children.iter().map(render_node).collect::<Vec<_>>()}</section>
        }
        .into_view(),

        "h1" => view! {
            <h1 class=get_class_from_attributes(
                attributes,
            )>{children.iter().map(render_node).collect::<Vec<_>>()}</h1>
        }
        .into_view(),

        _ => view! {
            <div class="unknown-element" data-element-type=name.to_string()>
                {children.iter().map(render_node).collect::<Vec<_>>()}
            </div>
        }
        .into_view(),
    }
}

// Helper function to extract class names from attributes
fn get_class_from_attributes(attributes: &[mdast::MdxJsxAttribute]) -> Option<String> {
    attributes.iter().find_map(|attr| match attr.name.as_str() {
        "class" | "className" => match &attr.value {
            Some(value) => match value {
                AttributeValue::Expression(exp) => Some(exp.value.clone()),
                AttributeValue::Literal(lit) => Some(lit.to_owned()),
            },
            None => None,
        },
        _ => None,
    })
}
