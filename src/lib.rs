mod posts;
mod templates;
use std::{fs::create_dir_all, path::PathBuf};

use anyhow::Result;

use posts::*;

const POSTS_DIR: &str = "posts";
const POST_TEMPLATE: &str = "post.html";
const OUTPUT_PATH: &str = "dist";

pub fn build() -> Result<()> {
    build_posts()?;
    build_index()?;
    Ok(())
}

fn build_posts() -> Result<()> {
    println!(" ▶️ Building posts...");
    create_dir_all(PathBuf::from(OUTPUT_PATH).join(POSTS_DIR))?;
    let posts = load_posts(POSTS_DIR)?;

    for post in posts {
        let rendered = templates::render_template(POST_TEMPLATE, &post)?;
        let path = post.path.with_extension("html");
        let output = PathBuf::from(OUTPUT_PATH).join(path);
        std::fs::write(output, rendered)?;
    }
    Ok(())
}

fn build_index() -> Result<()> {
    println!(" ▶️ Building index...");
    Ok(())
}
