use pulldown_cmark::{html, Event, Options, Parser, Tag};

/**
 * https://docs.rs/pulldown-cmark/latest/pulldown_cmark/#example
 */
pub fn parse_md_to_html(markdown_input: &str) -> String {
    // Set up options and parser. Strikethroughs are not part of the CommonMark standard
    // and we therefore must enable it explicitly.
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_SMART_PUNCTUATION);
    let parser = Parser::new_ext(markdown_input, options);
    let parser = parser.map(|event| match event {
        Event::Start(tag) => {
            // convert image src to absolute path
            if let Tag::Image(a, url, b) = tag {
                let u = url.replace("../image", "/image");
                Event::Start(Tag::Image(a, u.into(), b))
            } else {
                Event::Start(tag)
            }
        }
        _ => event,
    });
    // Write to String buffer.
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    // Check that the output is what we expected.
    html_output
}
