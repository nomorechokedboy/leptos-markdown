#[cfg(test)]
mod tests {
    use crate::{
        helper::{clean_leptos_markup, TestCase},
        render_node,
    };

    const EMPTY_LINK: &str = "![]()";
    const NINJA_IMG: &str = "![an image](/ninJA.png).";
    const EXPECTED_NINJA_IMG: &str =
        concat!("<p><img src=\"/ninJA.png\" alt=\"an image\"", "/>.</p>");
    const EMPTY_IMAGE: &str = concat!("<p><img src alt", "/></p>");
    #[test]
    fn test_image_handling() {
        let test_cases = vec![
            TestCase {
                desc: "should support images without alt, url, or title",
                expected: EMPTY_IMAGE,
                source: EMPTY_LINK,
                ..Default::default()
            },
            TestCase {
                desc: "should support images without title attribute",
                expected: EXPECTED_NINJA_IMG,
                source: NINJA_IMG,
                ..Default::default()
            },
            TestCase {
                desc: "should handle images with title attribute",
                expected: "<p>This is <img src=\" /ninJA.png\" alt=\"an
        image\" title=\"foo bar\">.</p>",
                source: "This is ![an image](/ninJA.png \"foo bar\").",
                ..Default::default()
            },
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
