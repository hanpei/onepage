use std::path::Path;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{markdown::parse_md_to_html, posts::PostIndex};

pub fn load_index<P: AsRef<Path>>(path: P, posts: Vec<PostIndex>) -> Result<IndexPage> {
    let raw_content = std::fs::read_to_string(path)?;
    let title = raw_content.lines().next().unwrap().to_string();
    let content = parse_md_to_html(&raw_content);
    Ok(IndexPage::new(title, content, posts))
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IndexPage {
    title: String,
    content: String,
    posts: Vec<PostIndex>,
}

impl IndexPage {
    pub fn new(title: String, content: String, posts: Vec<PostIndex>) -> Self {
        Self {
            title,
            content,
            posts,
        }
    }
}
