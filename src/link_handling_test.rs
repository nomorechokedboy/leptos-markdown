#[cfg(test)]
mod tests {
    use crate::{
        helper::{clean_leptos_markup, TestCase},
        render_node,
    };

    #[test]
    fn test_markdown_rendering() {
        let test_cases=vec![
TestCase {
desc: "should handle links without title attribute" ,
expected: "<p>This is <a href=\" https://rust-lang.org/\">
        a link</a> to Rust Lang</p>",
source: "This is [a link](https://rust-lang.org/) to Rust Lang",
},
TestCase {
desc: "should handle links with title attribute",
expected: "<p>This is <a href=\"https://rust-lang.org/\" title=\"some title\">a link</a> to Rust Lang</p>",
source: "This is [a link](https://rust-lang.org/ \"some title\") to Rust Lang",
}
];

        for test_case in test_cases {
            let node = markdown::to_mdast(&test_case.source, &markdown::ParseOptions::mdx())
                .expect("Lmao err");
            let result = render_node(&node);
            let html_string = clean_leptos_markup(&result.clone().render_to_string());
            assert_eq!(
                test_case.expected, html_string,
                "\nTest '{}' failed.\nExpected: {}\nGot: {}\n",
                test_case.desc, test_case.expected, html_string,
            );
        }
    }
}
