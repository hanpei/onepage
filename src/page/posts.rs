use anyhow::Result;
use std::{
    ops::{Deref, DerefMut},
    path::Path,
};

use crate::builder::LoadPage;

use super::{Post, PostIndex};

#[derive(Debug)]
pub struct Posts {
    inner: Vec<Post>,
}

impl Posts {
    pub fn new(inner: Vec<Post>) -> Self {
        Self { inner }
    }
    pub fn into_inner(&self) -> &Vec<Post> {
        &self.inner
    }

    pub fn get_post_index(&mut self) -> Vec<PostIndex> {
        let mut post_index = self
            .inner
            .iter()
            .map(|post| post.into())
            .collect::<Vec<PostIndex>>();
        post_index.sort_by(|a, b| b.date.cmp(&a.date));
        post_index
    }
}

impl LoadPage for Posts {
    type Item = Self;
    /**
     * Load posts from a dictionary.
     */
    fn load<P: AsRef<Path>>(path: P) -> Result<Self::Item> {
        let mut posts = Vec::new();
        let files = walkdir::WalkDir::new(path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .filter(|e| e.path().display().to_string().ends_with(".md"))
            .map(|e| e.path().to_path_buf())
            .collect::<Vec<_>>();

        for file in files {
            let post = Post::load(file)?;
            posts.push(post);
        }

        Ok(Posts::new(posts))
    }
}

impl Deref for Posts {
    type Target = Vec<Post>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Posts {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_load_posts() {
        let posts = Posts::load("pages/posts").unwrap();
        assert_eq!(posts.len(), 4);
        let paths = posts
            .iter()
            .map(|p| p.path.display().to_string())
            .collect::<Vec<_>>();
        assert_eq!(
            paths,
            vec![
                "posts/markdown.md",
                "posts/hello-world.md",
                "posts/syntax-demo.md",
                "posts/test.md"
            ]
        );
    }
}
