mod builder;
mod index;
mod init;
mod markdown;
mod posts;
mod server;
mod templates;
pub use builder::*;
pub use init::*;
pub use server::*;

pub const BASE_PATH: &str = ".";
pub const STATIC_PATH: &str = "static";
pub const PAGE_DIR: &str = "pages";
pub const POSTS_DIR: &str = "posts";
pub const POST_TEMPLATE: &str = "post.html";
pub const INDEX_TEMPLATE: &str = "index.html";
pub const OUTPUT_PATH: &str = "dist";
pub const STARTER_TEMPLATE_URL: &str =
    "https://github.com/hanpei/onepage-starter/archive/refs/heads/main.zip";
