use pulldown_cmark::{html, Options, Parser};

/**
 * https://docs.rs/pulldown-cmark/latest/pulldown_cmark/#example
 */
pub fn parse_md_to_html(markdown_input: &str) -> String {
    // Set up options and parser. Strikethroughs are not part of the CommonMark standard
    // and we therefore must enable it explicitly.
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(markdown_input, options);

    // Write to String buffer.
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    // Check that the output is what we expected.
    html_output
}
