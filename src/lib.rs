mod index;
mod markdown;
mod posts;
mod server;
mod templates;

use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::Result;

use crate::index::IndexPage;
use posts::*;

const PAGE_DIR: &str = "pages";
const POSTS_DIR: &str = "posts";
const POST_TEMPLATE: &str = "post.html";
const INDEX_TEMPLATE: &str = "index.html";
const OUTPUT_PATH: &str = "dist";

pub trait LoadSourceFile {
    type Item;
    fn load<P: AsRef<Path>>(path: P) -> Result<Self::Item>;
}

pub struct SiteBuilder {
    pub base_path: PathBuf,
    pub index: IndexPage,
    pub posts: Vec<Post>,
}

impl SiteBuilder {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let posts = Posts::load(path.as_ref().join(POSTS_DIR)).expect("loading posts error");
        let index = IndexPage::load(path.as_ref()).expect("loading index page error");
        Self {
            base_path: path.as_ref().to_path_buf(),
            index,
            posts,
        }
    }

    pub fn reload(&mut self) -> Result<()> {
        self.posts = Posts::load(self.base_path.join(POSTS_DIR)).expect("loading posts error");
        self.index = IndexPage::load(self.base_path.as_path()).expect("loading index page error");
        self.build()
    }

    pub fn build(&mut self) -> Result<()> {
        fs::remove_dir_all(OUTPUT_PATH)?;
        fs::create_dir_all(OUTPUT_PATH)?;
        self.build_posts()?;
        self.build_index()?;
        self.build_assets()?;
        println!("✅ Build success.");
        Ok(())
    }

    fn build_posts(&mut self) -> Result<()> {
        println!("🏃🏻 Building post pages...");
        let output = PathBuf::from(OUTPUT_PATH).join(POSTS_DIR);
        fs::create_dir_all(output)?;

        for post in &self.posts {
            let rendered = templates::render_template(POST_TEMPLATE, post)?;
            let path = post.path.with_extension("html");
            let output = PathBuf::from(OUTPUT_PATH).join(path);
            std::fs::write(output, rendered)?;
        }
        println!("\t- {} post pages built.", self.posts.len());

        Ok(())
    }

    fn crate_post_index(&mut self) -> Vec<PostIndex> {
        println!("🏃🏻 Create post index ...");

        let mut post_index = self
            .posts
            .iter()
            .map(|post| post.into())
            .collect::<Vec<PostIndex>>();
        post_index.sort_by(|a, b| b.date.cmp(&a.date));
        println!("\t- {} post index created.", post_index.len());
        post_index
    }

    fn build_index(&mut self) -> Result<()> {
        println!("🏃🏻 Building index page...");
        let post_index = self.crate_post_index();
        self.index.set_post_index(post_index);
        let rendered = templates::render_template(INDEX_TEMPLATE, &self.index)?;
        let output = PathBuf::from(OUTPUT_PATH).join("index.html");
        std::fs::write(output, rendered)?;
        Ok(())
    }

    fn build_assets(&mut self) -> Result<()> {
        println!("🏃🏻 Copying assets...");
        let input = self.base_path.join("assets");
        let output = PathBuf::from(OUTPUT_PATH);

        walkdir::WalkDir::new(input)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .for_each(|e| {
                let input = e.path();
                let output = output.join(input.strip_prefix(PAGE_DIR).unwrap());
                // println!("\tinput\t - {}", input.display());
                // println!("\t- {}", output.display());

                fs::create_dir_all(output.parent().unwrap()).unwrap();
                fs::copy(input, output).unwrap();
            });

        //copy ico and favicon
        walkdir::WalkDir::new(self.base_path.clone())
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .filter(|e| {
                let filename = e.file_name().to_str().unwrap();
                filename != "index.md" && !filename.starts_with(".")
            })
            .for_each(|e| {
                let input = e.path();
                let output = output.join(input.strip_prefix(PAGE_DIR).unwrap());
                // println!("\tinput\t - {}", input.display());
                // println!("\t- {}", output.display());

                fs::copy(input, output).unwrap();
            });
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn walk_assets_dir() {
        let mut page = SiteBuilder::new("pages");
        page.build_assets().unwrap();
    }
}
