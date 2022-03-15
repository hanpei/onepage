use anyhow::Result;
use chrono::{DateTime, Local, NaiveDateTime};
use gray_matter::{engine::YAML, Matter};
use serde::{Deserialize, Serialize};

use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Post {
    path: PathBuf,
    title: String,
    content: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FrontMatter {
    title: String,
    tags: Vec<String>,
    date: String, //yyyy-mm-dd hh:mm:ss
}

impl Post {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Post> {
        let content = std::fs::read_to_string(&path)?;
        // let title = content.lines().next().unwrap().to_string();
        Ok(Post {
            path: path.as_ref().to_owned(),
            title: "fake_title".to_string(),
            content,
        })
    }

    pub fn read_front_matter(content: &str) {
        let matter = Matter::<YAML>::new();
        let fm = matter.parse_with_struct::<FrontMatter>(content).unwrap();

        println!("front matter {:#?}", fm.data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_front_matter() {
        let post = Post::load("posts/abc.md").unwrap();
        Post::read_front_matter(&post.content);
    }
}
