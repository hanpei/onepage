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

const PAGE_DIR: &str = "pages";
const POSTS_DIR: &str = "posts";
const POST_TEMPLATE: &str = "post.html";
const INDEX_TEMPLATE: &str = "index.html";
const OUTPUT_PATH: &str = "dist";

pub fn build() -> Result<()> {
    fs::remove_dir_all(OUTPUT_PATH)?;
    fs::create_dir_all(OUTPUT_PATH)?;

    let posts = build_posts()?;
    build_index(posts)?;
    build_assets()?;
    Ok(())
}

fn build_posts() -> Result<Vec<PostIndex>> {
    println!(" ▶️ Building posts...");
    let input = PathBuf::from(PAGE_DIR).join(POSTS_DIR);
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
    let input = PathBuf::from(PAGE_DIR).join("index.md");
    let index = load_index(input, posts)?;
    let rendered = templates::render_template(INDEX_TEMPLATE, &index)?;
    let output = PathBuf::from(OUTPUT_PATH).join("index.html");

    std::fs::write(output, rendered)?;

    Ok(())
}

fn build_assets() -> Result<()> {
    println!(" ▶️ Copying assets...");
    let input = PathBuf::from(PAGE_DIR).join("assets");
    let output = PathBuf::from(OUTPUT_PATH);

    walkdir::WalkDir::new(input)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .for_each(|e| {
            let input = e.path();
            let output = output.join(input.strip_prefix(PAGE_DIR).unwrap());
            println!("\tinput\t - {}", input.display());
            println!("\toutput\t - {}", output.display());

            fs::create_dir_all(output.parent().unwrap()).unwrap();
            fs::copy(input, output).unwrap();
        });

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn walk_assets_dir() {
        build_assets().unwrap();
    }
}
