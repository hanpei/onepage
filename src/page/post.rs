use anyhow::{bail, Result};
use chrono::Local;
use gray_matter::{engine::YAML, Matter};
use std::path::{Path, PathBuf};

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

        let (fm, md) = Self::read_front_matter(&raw_content, &path)?;

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
    pub fn read_front_matter<P: AsRef<Path>>(
        content: &str,
        path: P,
    ) -> Result<(FrontMatter, String)> {
        let matter = Matter::<YAML>::new();
        match matter.parse_with_struct::<FrontMatter>(content) {
            Some(parsed) => Ok((parsed.data, parsed.content)),
            None => bail!(
                r#"
Invalid front matter found in {}", 
font matter details:[ https://github.com/hanpei/onepage#new-post ]"#,
                path.as_ref().display(),
            ),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_load_post() {
        let post = Post::load("pages/posts/test.md").unwrap();
        assert_eq!(post.front_matter.title, "Page for test");
        assert_eq!(post.front_matter.tags.unwrap(), ["draft"]);
        assert_eq!(post.front_matter.date, "2022-03-29 12:00");

        assert_eq!(post.title, "Page for test");
        assert_eq!(post.content, "<h1>Test</h1>\n<p>this is content</p>\n");
        assert_eq!(post.url, "/posts/test.html");
        assert_eq!(post.path, PathBuf::from("posts/test.md"));
    }

    #[test]
    fn valid_front_matter() {
        let content = r#"---
title: "test"
tags: 
        - "test"
        - "test2"  
date: "2020-01-01 00:00:00"
---
# test
is test
"#;
        let (matter, _) = Post::read_front_matter(&content, "path/demo.md").unwrap();
        assert_eq!(matter.title, "test");
        assert_eq!(matter.tags.unwrap(), ["test", "test2"]);
        assert_eq!(matter.date, "2020-01-01 00:00:00");
    }

    #[test]
    fn empty_tag_front_matter() {
        let content = r#"---
title: "test"
date: "2020-01-01 00:00:00"
---
"#;
        let (matter, _) = Post::read_front_matter(&content, "path/demo.md").unwrap();
        assert_eq!(matter.title, "test");
        assert_eq!(matter.date, "2020-01-01 00:00:00");
        assert!(matter.tags.is_none());
    }

    #[test]
    #[should_panic]
    fn invalid_front_matter() {
        let content = r#"---
        name: "name"
        ---
        this is content
        "#;
        let post = Post::read_front_matter(&content, "path/demo.md");
        assert!(post.is_err());
        post.unwrap();
    }

    #[test]
    fn empty_front_matter() {
        let content = r#"---
        ---
        this is content
        "#;
        let post = Post::read_front_matter(&content, "path/demo.md");
        assert!(post.is_err());
    }
}
