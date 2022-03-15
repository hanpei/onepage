mod posts;
use anyhow::Result;

pub use posts::*;

pub fn load_posts(posts_dir: &str) -> Result<Vec<Post>> {
    let mut posts = Vec::new();
    walkdir::WalkDir::new("posts")
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.path().display().to_string().ends_with(".md"))
        .for_each(|e| match Post::load(e.path()) {
            Ok(p) => posts.push(p),
            Err(e) => {
                println!("load posts error:  {}", e);
            }
        });

    Ok(posts)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_load_posts() {
        let posts = load_posts("posts").unwrap();
        println!("{:#?}", posts);
        assert_eq!(posts.len(), 2);
    }
}
