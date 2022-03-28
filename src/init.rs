use anyhow::Result;
use std::{fs, path::PathBuf};

pub fn init(root: &str, starter_template_url: &str) -> Result<()> {
    let mut file = tempfile::tempfile()?;
    println!("Downloading starter template from {}", starter_template_url);
    reqwest::blocking::get(starter_template_url)
        .unwrap()
        .copy_to(&mut file)?;
    let mut zip = zip::ZipArchive::new(file)?;

    zip.extract("./")?;
    fs::rename(PathBuf::from("onepage-starter-main"), PathBuf::from(root))?;
    println!("Site initialized at {}", root);
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    #[ignore]
    fn it_workd() {
        init(
            "temp",
            "https://github.com/hanpei/onepage-starter/archive/refs/heads/main.zip",
        )
        .unwrap();
    }
}
