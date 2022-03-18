use static_site_generator::SiteBuilder;

fn main() {
    let result = SiteBuilder::new("pages").build();
    match result {
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
