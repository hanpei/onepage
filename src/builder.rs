use anyhow::{bail, Result};
use chrono::Local;
use std::{fs, io::Write, path::Path};

use crate::{
    page::{IndexPage, Posts},
    templates, utils, Config, INDEX_TEMPLATE, POST_TEMPLATE,
};

pub trait LoadPage {
    type Item;
    fn load<P: AsRef<Path>>(path: P) -> Result<Self::Item>;
}

#[derive(Debug, Default)]
pub struct SiteBuilder {
    pub config: Config,
    pub index: IndexPage,
    pub posts: Posts,
}

impl SiteBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load(&mut self) {
        println!("🏃🏻 Loading posts ...");
        let posts = Posts::load(self.config.get_page_posts_path()).unwrap_or_else(|e| {
            println!("\n\n💥 Failed to load \"pages/posts/...\": \n{}\n\n", e);
            std::process::exit(1);
        });
        println!("🏃🏻 Loading index page ...");
        let index = IndexPage::load(self.config.get_page_index_path()).unwrap_or_else(|e| {
            println!("\n💥 Failed to load \"pages/index.md\" page: \n{}\n\n", e);
            std::process::exit(1);
        });
        self.posts = posts;
        self.index = index;
    }

    pub fn create_page(&mut self, name: &str) -> Result<()> {
        let path = self.config.get_page_posts_path().join(name);
        if path.exists() {
            bail!("Page already exists.");
        }
        let mut file = fs::File::create(&path)?;
        let title = name.to_string().replace(".md", "");
        let date = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let content = format!(
            r#"---
title: {title}
date: {date}
tags: 
    - draft
---

# Write your post here.

"#
        );
        file.write_all(content.as_bytes())?;
        println!("✅ Create \"{}\" success.", &path.display().to_string());
        Ok(())
    }

    pub fn rebuild(&mut self) -> Result<()> {
        self.load();

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
        self.load();
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

        for post in self.posts.as_ref() {
            let rendered = templates::render_template(POST_TEMPLATE, post)?;
            let path = post.path.with_extension("html");
            let output = self.config.output_dir.join(path);

            std::fs::write(output, rendered)?;
        }

        self.copy_pages_image()?;

        Ok(())
    }

    fn copy_pages_image(&mut self) -> Result<()> {
        let src = self.config.get_page_image_path();
        let dst = &self.config.get_output_image_path(&src);
        utils::copy_files(src.as_path(), dst.as_path())?;
        Ok(())
    }

    fn build_index(&mut self) -> Result<()> {
        let post_index = self.posts.get_post_index();
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
        let src = self.config.static_dir.join("assets");
        let dst = self.config.get_output_assets_path(&src);
        utils::copy_files(src.as_path(), dst.as_path()).unwrap();

        Ok(())
    }

    fn build_favicon(&mut self) -> Result<()> {
        // copy ico and favicon
        let src = self.config.static_dir.join("favicon");
        let dst = self.config.get_output_favicon_path(&src);
        utils::copy_files(src.as_path(), dst.as_path())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::page::Post;

    use super::*;

    #[test]
    fn default_site() {
        let site = SiteBuilder::new();
        println!("{:#?}", site);
    }

    #[test]
    fn test_build() {
        let mut site = SiteBuilder::new();
        assert!(site.build().is_ok());
    }

    #[test]
    fn test_create_page() {
        let mut site = SiteBuilder::new();
        assert!(site.create_page("new.md").is_ok());
        let path = Path::new("pages/posts/new.md");
        let page = Post::load(&path);
        assert!(page.is_ok());
        let page = page.unwrap();
        assert_eq!(page.title, "new");
        fs::remove_file(&path).unwrap();
    }
}
