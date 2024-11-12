use regex::Regex;

pub fn clean_leptos_markup(input: &str) -> String {
    // First remove Leptos comments
    let leptos_comment_pattern =
        r"<!--(?:leptos-view\|src-[^>]*|hk=[^>]*\|leptos-(<>|dyn-child)-(?:start|end))-->";
    let comment_re = Regex::new(leptos_comment_pattern).unwrap();
    let without_comments = comment_re.replace_all(input, "");

    // Then remove data-hk attributes
    let data_hk_pattern = r#"\s+data-hk="[^"]*""#;
    let attr_re = Regex::new(data_hk_pattern).unwrap();
    attr_re.replace_all(&without_comments, "").to_string()
}

#[derive(Debug, Default)]
pub struct TestCase {
    pub desc: &'static str,
    pub expected: &'static str,
    pub source: &'static str,
}
