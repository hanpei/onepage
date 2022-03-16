use anyhow::Result;
use chrono::Local;
use gray_matter::{engine::YAML, Matter};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::parse_md_to_html;

#[derive(Debug, Deserialize, Serialize)]
pub struct Post {
    pub front_matter: FrontMatter,
    pub path: PathBuf,
    pub title: String,
    pub content: String,
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

impl Post {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Post> {
        let raw_content = std::fs::read_to_string(&path)?;
        let (fm, md) = Self::read_front_matter(&raw_content);
        let title = fm.title.clone();
        let content = parse_md_to_html(&md);
        Ok(Post {
            front_matter: fm,
            path: path.as_ref().to_owned(),
            title,
            content,
        })
    }

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
        let post = Post::load("posts/test.md").unwrap();
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
