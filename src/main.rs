use static_site_generator::load_posts;

fn main() {
    load_posts("/posts").unwrap();
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_walkdir() {
        walkdir::WalkDir::new("posts")
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .filter(|e| e.path().display().to_string().ends_with(".md"))
            .for_each(|e| {
                println!("====> {}", e.path().display());
            });
    }
}
