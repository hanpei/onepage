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
        let title = raw_content.lines().next().unwrap().to_string();
        let raw_content_without_title = raw_content.lines().skip(1).collect::<Vec<_>>().join("\n");
        let content = parse_md_to_html(&raw_content_without_title);

        Ok(IndexPage::new(title, content, None))
    }
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
