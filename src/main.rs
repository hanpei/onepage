use std::{net::SocketAddr, thread, time::Duration};

use axum::{http::StatusCode, routing::get_service, Router};
use static_site_generator::SiteBuilder;
use tower_http::services::ServeDir;

const CONTENT_DIR: &str = "pages";
const PUBLIC_DIR: &str = "dist";

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let mut site = SiteBuilder::new(CONTENT_DIR);
    site.build()?;

    tokio::task::spawn_blocking(move || {
        println!("\nListenning for changes ...");
        let mut hotwatch = hotwatch::Hotwatch::new().expect("hotwatch failed to initialize!");
        hotwatch
            .watch(CONTENT_DIR, move |_| {
                println!("Rebuilding site");
                match site.reload() {
                    Ok(_) => println!("✅ Site build success."),
                    Err(e) => println!("❌ Site build failed: {}", e),
                }
            })
            .expect("failed to watch content folder!");
        loop {
            thread::sleep(Duration::from_secs(1));
        }
    });
    let app = Router::new().nest(
        "/",
        get_service(ServeDir::new(PUBLIC_DIR)).handle_error(|error: std::io::Error| async move {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Unhandled internal error: {}", error),
            )
        }),
    );

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("serving site on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
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
