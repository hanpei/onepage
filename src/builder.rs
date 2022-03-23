use anyhow::Result;
use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{
    index::IndexPage,
    posts::{PostIndex, Posts},
    templates, INDEX_TEMPLATE, OUTPUT_PATH, PAGE_DIR, POSTS_DIR, POST_TEMPLATE, STATIC_PATH,
};

pub trait LoadSourceFile {
    type Item;
    fn load<P: AsRef<Path>>(path: P) -> Result<Self::Item>;
}

pub struct SiteBuilder {
    pub page_path: PathBuf,
    pub static_path: PathBuf,
    pub index: IndexPage,
    pub posts: Posts,
}

impl Default for SiteBuilder {
    fn default() -> Self {
        let path = Path::new(PAGE_DIR);
        Self {
            page_path: PAGE_DIR.into(),
            static_path: STATIC_PATH.into(),
            index: IndexPage::load(path).expect("loading index page error"),
            posts: Posts::load(path.join(POSTS_DIR)).expect("loading posts error"),
        }
    }
}

impl SiteBuilder {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        println!("🏃🏻 Loading posts ...");
        let posts = Posts::load(path.as_ref().join(POSTS_DIR)).expect("loading posts error");
        println!("🏃🏻 Loading index page ...");
        let index = IndexPage::load(path.as_ref()).expect("loading index page error");
        Self {
            page_path: path.as_ref().to_path_buf(),
            static_path: STATIC_PATH.into(),
            index,
            posts,
        }
    }

    pub fn rebuild(&mut self) -> Result<()> {
        self.posts = Posts::load(self.page_path.join(POSTS_DIR)).expect("loading posts error");
        self.index = IndexPage::load(self.page_path.as_path()).expect("loading index page error");
        fs::remove_dir_all(OUTPUT_PATH)?;
        fs::create_dir_all(OUTPUT_PATH)?;
        self.build_posts()?;
        self.build_index()?;
        self.build_assets()?;
        self.build_favicon()?;
        println!("✅ Build success.");
        println!();

        Ok(())
    }

    pub fn build(&mut self) -> Result<()> {
        fs::remove_dir_all(OUTPUT_PATH)?;
        fs::create_dir_all(OUTPUT_PATH)?;

        println!("🏃🏻 Building post pages...");
        self.build_posts()?;
        println!("\t- {} post pages built.", self.posts.len());

        println!("🏃🏻 Building index page...");
        self.build_index()?;

        println!("🏃🏻 Copying assets...");
        self.build_assets()?;
        self.build_favicon()?;
        println!("✅ Build success.");
        println!();
        Ok(())
    }

    fn build_posts(&mut self) -> Result<()> {
        let output = PathBuf::from(OUTPUT_PATH).join(POSTS_DIR);
        fs::create_dir_all(output)?;

        for post in self.posts.into_inner() {
            let rendered = templates::render_template(POST_TEMPLATE, post)?;
            let path = post.path.with_extension("html");
            let output = PathBuf::from(OUTPUT_PATH).join(path);
            std::fs::write(output, rendered)?;
        }

        walkdir::WalkDir::new(self.page_path.join("image"))
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .for_each(|e| {
                let input = e.path();
                let output = PathBuf::from(OUTPUT_PATH).join(input.strip_prefix(PAGE_DIR).unwrap());

                fs::create_dir_all(output.parent().unwrap()).unwrap();
                fs::copy(input, output).unwrap();
            });

        Ok(())
    }

    fn crate_post_index(&mut self) -> Vec<PostIndex> {
        // println!("🏃🏻 Create post index ...");

        let mut post_index = self
            .posts
            .iter()
            .map(|post| post.into())
            .collect::<Vec<PostIndex>>();
        post_index.sort_by(|a, b| b.date.cmp(&a.date));
        // println!("\t- {} post index created.", post_index.len());
        post_index
    }

    fn build_index(&mut self) -> Result<()> {
        let post_index = self.crate_post_index();
        self.index.set_post_index(post_index);
        let rendered = templates::render_template(INDEX_TEMPLATE, &self.index)?;
        let output = PathBuf::from(OUTPUT_PATH).join("index.html");
        std::fs::write(output, rendered)?;

        Ok(())
    }

    fn build_assets(&mut self) -> Result<()> {
        let input = self.static_path.join("assets");

        walkdir::WalkDir::new(input)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .for_each(|e| {
                let input = e.path();
                let output =
                    PathBuf::from(OUTPUT_PATH).join(input.strip_prefix(STATIC_PATH).unwrap());
                // println!("\tbuild_assetsinput\t - {}", input.display());
                // println!("\tbuild_assets- {}", output.display());

                fs::create_dir_all(output.parent().unwrap()).unwrap();
                fs::copy(input, output).unwrap();
            });

        Ok(())
    }

    fn build_favicon(&mut self) -> Result<()> {
        // copy ico and favicon
        let input = self.static_path.join("favicon");

        let output = PathBuf::from(OUTPUT_PATH);

        walkdir::WalkDir::new(input)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .for_each(|e| {
                let input = e.path();
                let output = output.join(input.file_name().unwrap());
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
