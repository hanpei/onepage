use anyhow::Result;
use std::{
    fs,
    path::{Path, PathBuf},
};

/**
 * get all file paths in a directory
 */
pub fn get_files_by_walkdir<P: AsRef<Path>>(path: P) -> Vec<PathBuf> {
    let files = walkdir::WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| e.path().to_path_buf())
        .collect::<Vec<_>>();
    files
}

/**
 * copy files from src directory to dst directory
 * @param src: dictionary of source files
 * @param dst: dictionary of destination files
 */
pub fn copy_files(src: &Path, dst: &Path) -> Result<()> {
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

    #[test]
    fn copy_files_test() {
        if fs::metadata("tmp").is_ok() {
            fs::remove_dir_all("tmp").unwrap();
        }
        let src = PathBuf::from("tmp/src/");
        let files = [
            src.join("test.md"),
            src.join("1/1.md"),
            src.join("1/2/2.md"),
        ];
        files.iter().for_each(|f| {
            fs::create_dir_all(f.parent().unwrap()).unwrap();
            fs::write(f, f.display().to_string()).unwrap();
        });

        let dst = PathBuf::from("tmp/dst/");
        copy_files(&src.as_path(), &dst.as_path()).unwrap();

        let outputs = get_files_by_walkdir(&dst)
            .into_iter()
            .map(|p| p.strip_prefix("tmp/dst").unwrap().display().to_string())
            .collect::<Vec<_>>();
        let inputs = files
            .iter()
            .map(|p| p.strip_prefix("tmp/src").unwrap().display().to_string())
            .collect::<Vec<_>>();

        assert!(inputs.iter().all(|item| outputs.contains(item)));
    }

    #[test]
    fn test_get_md_files_by_walkdir() {
        let result = get_files_by_walkdir("pages")
            .into_iter()
            .filter(|e| e.display().to_string().ends_with(".md"))
            .map(|p| p.display().to_string())
            .collect::<Vec<_>>();
        assert_eq!(
            result,
            vec![
                "pages/posts/markdown.md",
                "pages/posts/hello-world.md",
                "pages/posts/syntax-demo.md",
                "pages/posts/test.md",
                "pages/index.md"
            ]
        )
    }
}
