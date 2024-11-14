#[cfg(test)]
mod tests {
    use crate::{
        helper::{clean_leptos_markup, TestCase},
        render_node,
    };

    #[test]
    fn test_markdown_rendering() {
        let test_cases = vec![
            TestCase {
                desc: "should render a single paragraph",
                expected: "<p>Test</p>",
                source: "Test",
                ..Default::default()
            },
            TestCase {
                desc: "should handle multiple paragraphs properly",
                expected: concat!(
                    "<p>Vue is awesome\nAnd so is markdown</p>",
                    "<p>Combining = epic</p>"
                ),
                source: "Vue is awesome\nAnd so is markdown\n\nCombining = epic",
                ..Default::default()
            },
            TestCase {
                desc: "should handle multiline paragraphs properly (softbreak, paragraphs)",
                expected: concat!(
                    "<p>Vue is awesome\nAnd so is markdown",
                    concat!("<br", "/>"),
                    "Combining=epic</p>"
                ),
                source: concat!(
                    "Vue is awesome\nAnd so is markdown ",
                    " ",
                    "\nCombining=epic"
                ),
                ..Default::default()
            },
            TestCase {
                desc: "should handle emphasis",
                expected: "<p>Vue is <em>totally</em> <em>awesome</em></p>",
                source: "Vue is _totally_ *awesome*",
                ..Default::default()
            },
            TestCase {
                desc: "should handle bold/strong texts",
                expected: "<p>Vue is <strong>totally</strong> <strong>awesome</strong></p>",
                source: "Vue is __totally__ **awesome**",
                ..Default::default()
            },
            TestCase {
                desc: "should handle headers",
                expected: "<h1>Awesome</h1>",
                source: "# Awesome",
                ..Default::default()
            },
            TestCase {
                desc: "should handle blockquotes",
                expected: "<blockquote><p>Moo\nTools\nFTW</p></blockquote>",
                source: "> Moo\n> Tools\n> FTW\n",
                ..Default::default()
            },
            TestCase {
                desc: "should handle code blocks with language",
                expected: "<pre><code class=\"language-js\">var foo = require(&#x27;bar&#x27;);\nfoo();</code></pre>",
                source: "```js\nvar foo = require(\'bar\');\nfoo();\n```",
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
