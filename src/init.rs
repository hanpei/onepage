use anyhow::Result;
use std::path::PathBuf;

use fs_extra::dir::CopyOptions;

pub fn init(root: &str) -> Result<()> {
    let dirs = vec!["pages", "static", "templates"];
    let mut options = CopyOptions::new();
    options.buffer_size = 1;
    fs_extra::dir::create_all(&root, true)?;
    fs_extra::copy_items(&dirs, PathBuf::from(root), &options)?;
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_workd() {
        init("temp").unwrap();
    }
}
