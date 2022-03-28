use anyhow::Result;
use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{
    page::{IndexPage, PostIndex, Posts},
    templates, INDEX_TEMPLATE, OUTPUT_DIR, PAGE_DIR, POSTS_DIR, POST_TEMPLATE, STATIC_DIR,
};

pub trait LoadPage {
    type Item;
    fn load<P: AsRef<Path>>(path: P) -> Result<Self::Item>;
}

#[derive(Debug)]
pub struct Config {
    page_dir: PathBuf,
    static_dir: PathBuf,
    output_dir: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            page_dir: PathBuf::from(PAGE_DIR),
            static_dir: PathBuf::from(STATIC_DIR),
            output_dir: PathBuf::from(OUTPUT_DIR),
        }
    }
}

impl Config {
    pub fn get_page_posts_path(&self) -> PathBuf {
        self.page_dir.join(POSTS_DIR)
    }

    pub fn get_output_posts_path(&self) -> PathBuf {
        self.output_dir.join(POSTS_DIR)
    }

    pub fn get_page_index_path(&self) -> PathBuf {
        self.page_dir.join("index.md")
    }

    pub fn get_page_image_path(&self) -> PathBuf {
        self.page_dir.join("image")
    }

    /** image path:
     * input:  /pages/images/xxx.png
     * output: /dist/images/xxx.png
     */
    pub fn get_output_image_path(&self, input: &Path) -> PathBuf {
        let path = input.strip_prefix(&self.page_dir).unwrap();
        self.output_dir.join(path)
    }

    /** assets path:
     *  input: /static/assets/xxx.css
     *  output: /dist/assets/xxx.css
     */
    pub fn get_output_assets_path(&self, input: &Path) -> PathBuf {
        let path = input.strip_prefix(&self.static_dir).unwrap();
        self.output_dir.join(path)
    }

    /** favicon path:
     *  input: /static/favicon/favicon.ico
     *  output: /dist/favicon.ico
     */
    pub fn get_output_favicon_path(&self, input: &Path) -> PathBuf {
        let path = input
            .strip_prefix(&self.static_dir.join("favicon"))
            .unwrap();
        self.output_dir.join(path)
    }
}

#[derive(Debug)]
pub struct SiteBuilder {
    pub config: Config,
    pub index: IndexPage,
    pub posts: Posts,
}

impl SiteBuilder {
    pub fn new() -> Self {
        let config = Config::default();
        println!("🏃🏻 Loading posts ...");
        let posts = Posts::load(config.get_page_posts_path()).expect("loading posts error");
        println!("🏃🏻 Loading index page ...");
        let index =
            IndexPage::load(config.get_page_index_path()).expect("loading index page error");

        Self {
            config,
            index,
            posts,
        }
    }

    pub fn rebuild(&mut self) -> Result<()> {
        self.posts = Posts::load(self.config.get_page_posts_path()).expect("loading posts error");
        self.index =
            IndexPage::load(self.config.get_page_index_path()).expect("loading index page error");
        fs::remove_dir_all(&self.config.output_dir)?;
        fs::create_dir_all(&self.config.output_dir)?;
        self.build_posts()?;
        self.build_index()?;
        self.build_statics()?;
        println!("✅ Build success.");
        println!();

        Ok(())
    }

    pub fn build(&mut self) -> Result<()> {
        // if exists output dir, remove it
        if fs::metadata(&self.config.output_dir).is_ok() {
            fs::remove_dir_all(&self.config.output_dir)?;
        }
        fs::create_dir_all(&self.config.output_dir)?;

        println!("🏃🏻 Building post pages...");
        self.build_posts()?;
        println!("\t- {} post pages built.", self.posts.len());

        println!("🏃🏻 Building index page...");
        self.build_index()?;

        println!("🏃🏻 Copying static files...");
        self.build_statics()?;
        println!("✅ Build success.");
        println!();
        Ok(())
    }

    fn build_posts(&mut self) -> Result<()> {
        let output = self.config.get_output_posts_path();
        fs::create_dir_all(output)?;

        for post in self.posts.into_inner() {
            let rendered = templates::render_template(POST_TEMPLATE, post)?;
            let path = post.path.with_extension("html");
            let output = self.config.output_dir.join(path);

            std::fs::write(output, rendered)?;
        }

        self.copy_pages_image()?;

        Ok(())
    }

    fn copy_pages_image(&mut self) -> Result<()> {
        walkdir::WalkDir::new(self.config.get_page_image_path())
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .for_each(|e| {
                let input = e.path();
                let output = &self.config.get_output_image_path(input);

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
        let output = self.config.output_dir.join("index.html");
        std::fs::write(output, rendered)?;

        Ok(())
    }

    fn build_statics(&mut self) -> Result<()> {
        self.build_assets()?;
        self.build_favicon()?;
        Ok(())
    }

    fn build_assets(&mut self) -> Result<()> {
        let input = self.config.static_dir.join("assets");

        walkdir::WalkDir::new(input)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .for_each(|e| {
                let input = e.path();
                let output = &self.config.get_output_assets_path(input);
                fs::create_dir_all(output.parent().unwrap()).unwrap();
                fs::copy(input, output).unwrap();
            });

        Ok(())
    }

    fn build_favicon(&mut self) -> Result<()> {
        // copy ico and favicon
        let input = &self.config.static_dir.join("favicon");

        walkdir::WalkDir::new(input)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .for_each(|e| {
                let input = e.path();
                let output = &self.config.get_output_favicon_path(input);
                fs::copy(input, output).unwrap();
            });
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn site_builder_test() {
        let mut page = SiteBuilder::new();
        println!("{:?}", page);
    }
}
