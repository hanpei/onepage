use anyhow::Result;
use axum::{http::StatusCode, routing::get_service, Router};
use notify::{RecommendedWatcher, Watcher};
use std::{net::SocketAddr, sync::mpsc, thread, time::Duration};
use tower_http::services::ServeDir;

use crate::{SiteBuilder, OUTPUT_PATH, PAGE_DIR};

pub struct SiteServer {
    host: String,
    port: u16,
}

impl Default for SiteServer {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8080,
        }
    }
}

impl SiteServer {
    pub fn new(address: &str) -> Self {
        let pairs = address.split(":").collect::<Vec<&str>>();
        let host = pairs[0].to_string();
        let port = pairs[1].parse::<u16>().unwrap_or_default();
        SiteServer { host, port }
    }

    pub fn run(self) -> Result<()> {
        let mut site = SiteBuilder::new(PAGE_DIR);
        site.build()?;

        let addr = format!("{}:{}", self.host, self.port).parse::<SocketAddr>()?;
        thread::spawn(move || {
            serve(addr).unwrap();
        });

        watch(&mut site);

        Ok(())
    }
}

#[tokio::main]
async fn serve(address: SocketAddr) -> Result<()> {
    let app = Router::new().nest(
        "/",
        get_service(ServeDir::new(OUTPUT_PATH)).handle_error(|error: std::io::Error| async move {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Unhandled internal error: {}", error),
            )
        }),
    );

    println!("Serving site on {}\n\n", address);
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

pub fn watch(site: &mut SiteBuilder) {
    let (tx, rx) = mpsc::channel();

    let mut watcher = RecommendedWatcher::new(tx, Duration::from_millis(50))
        .expect("watcher failed to initialize!");

    watcher
        .watch(PAGE_DIR, notify::RecursiveMode::Recursive)
        .expect("failed to watch content folder!");

    loop {
        match rx.recv() {
            Ok(event) => match event {
                hotwatch::Event::Create(path)
                | hotwatch::Event::Write(path)
                | hotwatch::Event::Remove(path)
                | hotwatch::Event::Rename(path, _) => {
                    path.display()
                        .to_string()
                        .split("pages")
                        .nth(1)
                        .map(|path| {
                            println!("File changed, {:?}", path);
                            println!("Rebuilding site...");
                        });
                    site.rebuild().expect("Site rebuild failed");
                }
                _ => {}
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}
