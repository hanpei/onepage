use static_site_generator::build;

fn main() {
    match build() {
        Ok(_) => {}
        Err(e) => {
            println!("❌ Building ERROR: {:?}", e);
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_walkdir() {
        walkdir::WalkDir::new("pages/posts")
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .filter(|e| e.path().display().to_string().ends_with(".md"))
            .for_each(|e| {
                println!("====> {}", e.path().display());
            });
    }
}
