use anyhow::Result;
use axum::{
    extract::{ws, WebSocketUpgrade},
    http::StatusCode,
    routing::{get, get_service},
    Router,
};
use notify::{RecommendedWatcher, Watcher};
use std::{net::SocketAddr, sync::mpsc, thread, time::Duration};

use tokio::sync::broadcast;
use tower_http::services::ServeDir;

use crate::{SiteBuilder, OUTPUT_DIR, PAGE_DIR};

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
        let pairs = address.split(':').collect::<Vec<&str>>();
        let host = pairs[0].to_string();
        let port = pairs[1].parse::<u16>().unwrap_or_default();
        SiteServer { host, port }
    }

    pub fn run(self) -> Result<()> {
        let mut site = SiteBuilder::new();
        site.build()?;
        let (reload_channel, _) = broadcast::channel(10);
        let tx = reload_channel.clone();
        let addr = format!("{}:{}", self.host, self.port).parse::<SocketAddr>()?;
        thread::spawn(move || {
            serve(addr, reload_channel).unwrap();
        });

        watch(&mut site, tx);

        Ok(())
    }
}

#[tokio::main]
async fn serve(address: SocketAddr, reload_channel: broadcast::Sender<()>) -> Result<()> {
    let app = Router::new()
        .fallback(get_service(ServeDir::new(OUTPUT_DIR)).handle_error(
            |error: std::io::Error| async move {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Unhandled internal error: {}", error),
                )
            },
        ))
        .route(
            "/__ws",
            get(|ws: WebSocketUpgrade| async move {
                ws.on_upgrade(|socket| async move { handle_socket(socket, reload_channel).await })
            }),
        );

    println!("Serving site on {}\n\n", address);
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

pub fn watch(site: &mut SiteBuilder, reload_channel: broadcast::Sender<()>) {
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
                    if let Some(path) = path.display().to_string().split("pages").nth(1) {
                        println!("File changed: {:?}", path);
                        println!("Rebuilding site...");
                    }
                    site.rebuild().expect("Site rebuild failed");
                    reload_channel
                        .send(())
                        .expect("livereloading message send failed");
                }
                _ => {}
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}

async fn handle_socket(
    mut socket: ws::WebSocket,
    reload_channel: broadcast::Sender<()>,
) -> Result<()> {
    let mut rx = reload_channel.subscribe();
    while rx.recv().await.is_ok() {
        println!("reload_channel recv ...");

        let ws_send = socket.send(ws::Message::Text("reload".to_owned()));
        if ws_send.await.is_err() {
            break;
        }
    }
    Ok(())
}
