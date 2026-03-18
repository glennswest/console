mod client;
mod ui;

use axum::{routing::get, Router};
use clap::Parser;
use std::sync::Arc;

#[derive(Parser)]
#[command(name = "console", about = "Unified dashboard for mkube + stormd")]
struct Args {
    /// mkube API base URL
    #[arg(long, default_value = "http://192.168.200.2:8082")]
    mkube_url: String,

    /// Listen address
    #[arg(long, default_value = "0.0.0.0:8090")]
    bind: String,
}

pub struct AppState {
    pub mkube_url: String,
    pub mkube: client::mkube::MkubeClient,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let state = Arc::new(AppState {
        mkube: client::mkube::MkubeClient::new(&args.mkube_url),
        mkube_url: args.mkube_url,
    });

    let app = Router::new()
        // Dashboard
        .route("/", get(|| async { axum::response::Redirect::to("/ui/") }))
        .route("/ui/", get(ui::dashboard::page))
        .route("/ui", get(|| async { axum::response::Redirect::to("/ui/") }))
        // Nodes
        .route("/ui/nodes", get(ui::nodes::list_page))
        .route("/ui/nodes/{name}", get(ui::nodes::detail_page))
        // Pods
        .route("/ui/pods", get(ui::pods::list_page))
        .route("/ui/pods/{ns}/{name}", get(ui::pods::detail_page))
        // Deployments
        .route("/ui/deployments", get(ui::deployments::list_page))
        .route("/ui/deployments/{ns}/{name}", get(ui::deployments::detail_page))
        // Networks
        .route("/ui/networks", get(ui::networks::list_page))
        .route("/ui/networks/{name}", get(ui::networks::detail_page))
        // BMH
        .route("/ui/bmh", get(ui::bmh::list_page))
        .route("/ui/bmh/{ns}/{name}", get(ui::bmh::detail_page))
        // Boot Configs
        .route("/ui/bootconfigs", get(ui::bootconfigs::list_page))
        .route("/ui/bootconfigs/{ns}/{name}", get(ui::bootconfigs::detail_page))
        // Registries
        .route("/ui/registries", get(ui::registries::list_page))
        .route("/ui/registries/{ns}/{name}", get(ui::registries::detail_page))
        // Storage
        .route("/ui/storage", get(ui::storage::page))
        // Jobs
        .route("/ui/jobs", get(ui::jobs::page))
        // Logs
        .route("/ui/logs", get(ui::logs::page))
        // Terminal
        .route("/ui/terminal", get(ui::terminal::page))
        // Health
        .route("/healthz", get(healthz))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&args.bind).await.unwrap();
    eprintln!("console listening on {}", args.bind);
    axum::serve(listener, app).await.unwrap();
}

async fn healthz() -> &'static str {
    r#"{"status":"ok","service":"console"}"#
}
