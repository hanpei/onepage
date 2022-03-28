use crate::constants::*;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Config {
    // markdown file path
    pub page_dir: PathBuf,
    // static file path, include css, js, img..
    pub static_dir: PathBuf,
    // output file path
    pub output_dir: PathBuf,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config() {
        let config = Config::default();
        assert_eq!(config.page_dir, PathBuf::from("pages"));
        assert_eq!(config.static_dir, PathBuf::from("static"));
        assert_eq!(config.output_dir, PathBuf::from("dist"));
        println!("{:?}", config);
    }

    #[test]
    fn get_path() {
        let config = Config::default();
        assert_eq!(config.get_page_posts_path(), PathBuf::from("pages/posts"));
        assert_eq!(config.get_output_posts_path(), PathBuf::from("dist/posts"));
        assert_eq!(
            config.get_page_index_path(),
            PathBuf::from("pages/index.md")
        );

        let image_path = config.get_page_image_path();
        assert_eq!(&image_path, &PathBuf::from("pages/image"));

        let image_file = image_path.join("xxx.png");
        assert_eq!(
            config.get_output_image_path(&image_file),
            PathBuf::from("dist/image/xxx.png")
        );

        assert_eq!(
            config.get_output_assets_path(&PathBuf::from("static/assets/123.css")),
            PathBuf::from("dist/assets/123.css")
        );
        assert_eq!(
            config.get_output_favicon_path(&PathBuf::from("static/favicon/favicon.ico")),
            PathBuf::from("dist/favicon.ico")
        );
    }
}
