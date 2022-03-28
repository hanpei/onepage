use anyhow::Result;
use std::{
    ops::{Deref, DerefMut},
    path::Path,
};

use crate::builder::LoadPage;

use super::Post;

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
}

impl LoadPage for Posts {
    type Item = Self;
    /**
     * Load posts from a dictionary.
     */
    fn load<P: AsRef<Path>>(path: P) -> Result<Self::Item> {
        let mut posts = Vec::new();
        walkdir::WalkDir::new(path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .filter(|e| e.path().display().to_string().ends_with(".md"))
            .for_each(|e| match Post::load(e.path()) {
                Ok(p) => posts.push(p),
                Err(e) => {
                    println!("load posts error:  {}", e);
                }
            });

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
    }
}
