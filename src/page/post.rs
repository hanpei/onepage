use anyhow::Result;
use gray_matter::{engine::YAML, Matter};
use std::path::{Path, PathBuf};

use chrono::Local;
use serde::{Deserialize, Serialize};

use crate::{markdown::parse_md_to_html, LoadPage, PAGE_DIR};

#[derive(Debug, Deserialize, Serialize)]
pub struct Post {
    pub front_matter: FrontMatter,
    pub path: PathBuf,
    pub url: String,
    pub title: String,
    pub content: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PostIndex {
    pub title: String,
    pub url: String,
    pub date: String,
}

impl From<&Post> for PostIndex {
    fn from(post: &Post) -> Self {
        Self {
            title: post.title.clone(),
            url: post.url.clone(),
            date: post.front_matter.date.clone(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FrontMatter {
    pub title: String,
    pub tags: Option<Vec<String>>,
    pub date: String, //yyyy-mm-dd hh:mm:ss
}

impl Default for FrontMatter {
    fn default() -> Self {
        FrontMatter {
            title: "Undefined Title".to_string(),
            tags: None,
            date: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        }
    }
}

impl LoadPage for Post {
    type Item = Post;

    fn load<P: AsRef<Path>>(path: P) -> Result<Self::Item> {
        let raw_content = std::fs::read_to_string(&path)?;
        let (fm, md) = Self::read_front_matter(&raw_content);
        let title = fm.title.clone();
        let content = parse_md_to_html(&md);
        let path = path.as_ref().strip_prefix(PAGE_DIR).unwrap().to_path_buf();
        Ok(Post {
            front_matter: fm,
            path: path.clone(),
            url: Path::new("/")
                .join(path)
                .with_extension("html")
                .display()
                .to_string(),
            title,
            content,
        })
    }
}

impl Post {
    pub fn read_front_matter(content: &str) -> (FrontMatter, String) {
        let matter = Matter::<YAML>::new();
        let parsed = matter
            .parse_with_struct::<FrontMatter>(content)
            .expect("invalid front matter");

        (parsed.data, parsed.content)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_load_post() {
        let post = Post::load("pages/posts/markdown.md").unwrap();
        println!("{:#?}", post);
    }

    #[test]
    fn front_matter() {
        let content = r#"---
title: "test"
tags: ["test"]
date: "2020-01-01 00:00:00"
---
# test
is test
"#;
        let (matter, _) = Post::read_front_matter(&content);
        println!("{:#?}", matter);
        assert_eq!(matter.title, "test");
        assert_eq!(matter.tags.unwrap(), ["test"]);
        assert_eq!(matter.date, "2020-01-01 00:00:00");
    }

    #[test]
    #[should_panic(expected = "invalid front matter")]
    fn empty_front_matter() {
        let content = "";
        let (matter, _) = Post::read_front_matter(&content);
        assert_eq!(matter.title, FrontMatter::default().title);
    }

    #[test]
    #[should_panic(expected = "invalid front matter")]
    fn invalid_front_matter() {
        let content = r#"---
        name: "name"
        ---
        this is content
        "#;
        Post::read_front_matter(&content);
    }
}
