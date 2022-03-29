use anyhow::Result;
use std::{fs, path::Path};

use crate::{
    page::{IndexPage, PostIndex, Posts},
    templates, Config, INDEX_TEMPLATE, POST_TEMPLATE,
};

pub trait LoadPage {
    type Item;
    fn load<P: AsRef<Path>>(path: P) -> Result<Self::Item>;
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
        let posts = Posts::load(config.get_page_posts_path()).unwrap_or_else(|e| {
            println!("\n\n💥 Failed to load posts: \n\t{}\n\n", e);
            std::process::exit(1);
        });
        println!("🏃🏻 Loading index page ...");
        let index = IndexPage::load(config.get_page_index_path()).unwrap_or_else(|e| {
            println!("\n💥 Failed to load index page: {}\n\n", e);
            std::process::exit(1);
        });

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
        let src = self.config.static_dir.join("assets");
        let dst = self.config.get_output_assets_path(&src);
        copy_files(src.as_path(), dst.as_path()).unwrap();

        Ok(())
    }

    fn build_favicon(&mut self) -> Result<()> {
        // copy ico and favicon
        let src = self.config.static_dir.join("favicon");
        let dst = self.config.get_output_favicon_path(&src);
        copy_files(src.as_path(), dst.as_path())?;
        Ok(())
    }
}

fn copy_files(src: &Path, dst: &Path) -> Result<()> {
    if src.is_dir() {
        fs::create_dir_all(dst)?;
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let src = entry.path();
            let dst = dst.join(src.file_name().unwrap());
            copy_files(&src, &dst)?;
        }
    } else {
        fs::copy(src, dst)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn clear_output_dir() {
        let output_dir = Config::default().output_dir;
        if fs::metadata(&output_dir).is_ok() {
            fs::remove_dir_all(&output_dir).unwrap();
        }
    }

    #[test]
    fn test_build() {
        let mut site = SiteBuilder::new();
        assert!(site.build().is_ok());
    }

    #[test]
    fn copy_files_test() {
        clear_output_dir();
        let config = Config::default();
        let src = &config.static_dir.join("assets");
        let dst = &config.get_output_assets_path(&src);
        copy_files(src.as_path(), dst.as_path()).unwrap();
        let input_files = fs::read_dir(src).unwrap().count();
        let output_files = fs::read_dir(dst).unwrap().count();
        assert_eq!(input_files, output_files);
    }
}
