use std::path::Path;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{markdown::parse_md_to_html, LoadPage};

use super::PostIndex;

#[derive(Debug, Deserialize, Serialize)]
pub struct IndexPage {
    title: String,
    content: String,
    post_index: Option<Vec<PostIndex>>,
}

impl LoadPage for IndexPage {
    type Item = IndexPage;

    fn load<P: AsRef<Path>>(path: P) -> Result<Self::Item> {
        let raw_content = std::fs::read_to_string(path)?;
        let md_title = raw_content.lines().next().unwrap().to_string();
        let title = strip_hash_from_title(&md_title);
        let raw_content_without_title = raw_content.lines().skip(1).collect::<Vec<_>>().join("\n");
        let content = parse_md_to_html(&raw_content_without_title);

        Ok(IndexPage::new(title, content, None))
    }
}

fn strip_hash_from_title(title: &str) -> String {
    title.split("#").last().unwrap().trim().to_string()
}

impl IndexPage {
    pub fn new(title: String, content: String, posts: Option<Vec<PostIndex>>) -> Self {
        Self {
            title,
            content,
            post_index: posts,
        }
    }

    pub fn set_post_index(&mut self, posts: Vec<PostIndex>) {
        self.post_index = Some(posts)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_hash_from_title() {
        assert_eq!(strip_hash_from_title("# title"), "title");
        assert_eq!(strip_hash_from_title("## title"), "title");
        assert_eq!(strip_hash_from_title("### title"), "title");
        assert_eq!(strip_hash_from_title("#### title"), "title");
        assert_eq!(strip_hash_from_title("##### title"), "title");
        assert_eq!(strip_hash_from_title("###### title"), "title");
        assert_eq!(strip_hash_from_title("####### title"), "title");
        assert_eq!(strip_hash_from_title("title"), "title");
    }

    #[test]
    fn test_load_index_page() {
        let index_page = IndexPage::load("pages/index.md").unwrap();
        assert_eq!(index_page.title, "ONEPAGE");

        let h3 = index_page.content.contains("<h3>Index Page</h3>");
        assert!(h3);
        let section = index_page.content.contains("<p>Sequi voluptates est voluptas architecto. Dolor fuga veniam velit molestiae consectetur. Ut adipisci illum non aliquam voluptas eum. Velit nostrum voluptatem. Aspernatur non saepe asperiores. Veritatis rerum magnam animi ea velit.</p>");
        assert!(section);

        assert!(index_page.post_index.is_none());
    }
}
