mod index;
mod markdown;
mod posts;
mod templates;
use std::{
    fs::{self},
    path::PathBuf,
};

use anyhow::Result;

use posts::*;

use crate::index::load_index;

const BASE_DIR: &str = "pages";
const POSTS_DIR: &str = "posts";
const POST_TEMPLATE: &str = "post.html";
const INDEX_TEMPLATE: &str = "index.html";
const OUTPUT_PATH: &str = "dist";

pub fn build() -> Result<()> {
    fs::remove_dir_all(OUTPUT_PATH)?;
    fs::create_dir_all(OUTPUT_PATH)?;

    let posts = build_posts()?;
    build_index(posts)?;
    Ok(())
}

fn build_posts() -> Result<Vec<PostIndex>> {
    println!(" ▶️ Building posts...");
    let input = PathBuf::from(BASE_DIR).join(POSTS_DIR);
    let output = PathBuf::from(OUTPUT_PATH).join(POSTS_DIR);

    fs::create_dir_all(output)?;
    let posts = load_posts(input)?;
    let mut posts_index = Vec::new();

    for post in posts {
        let rendered = templates::render_template(POST_TEMPLATE, &post)?;
        let path = post.path.with_extension("html");
        let output = PathBuf::from(OUTPUT_PATH).join(path);
        posts_index.push(post.get_index());
        std::fs::write(output, rendered)?;
    }
    Ok(posts_index)
}

fn build_index(posts: Vec<PostIndex>) -> Result<()> {
    println!(" ▶️ Building index...");
    let input = PathBuf::from(BASE_DIR).join("index.md");
    let index = load_index(input, posts)?;
    let rendered = templates::render_template(INDEX_TEMPLATE, &index)?;
    let output = PathBuf::from(OUTPUT_PATH).join("index.html");

    std::fs::write(output, rendered)?;

    Ok(())
}
