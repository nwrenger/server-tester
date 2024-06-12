use std::{net::SocketAddr, path::PathBuf};

use axum::{
    body::Body,
    extract::{Path, State},
    http::Request,
    response::IntoResponse,
    routing::get,
    Router,
};
use clap::{arg, command, Parser};
use maud::{html, Markup, DOCTYPE};
use tower::util::ServiceExt;
use tower_http::services::ServeFile;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{
    prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt, EnvFilter,
};

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// Ip and port for the webserver
    host: SocketAddr,
    /// Directory for the static assets
    #[arg(short, long, default_value = "./assets")]
    assets: PathBuf,
}

#[tokio::main]
async fn main() {
    logging();

    let Args { host, assets } = Args::parse();

    let app = Router::new()
        .route("/", get(root))
        .route("/*file", get(static_assets).with_state(assets))
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind(host).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

/// Initialize tracing
fn logging() {
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "debug".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();
}

/// Header of the Page
fn header(page_title: &str) -> Markup {
    html! {
        meta charset="UTF-8";
        meta name="viewport" content="width=device-width, initial-scale=1.0";
        title { (page_title) }
        script src="/static/content/node_modules/htmx.org/dist/htmx.min.js" {}
        link href="/static/content/dist/output.css" rel="stylesheet";
    }
}

/// The Page, useful when wanting to use multiple Pages
fn page(title: &str, html: Markup) -> Markup {
    html! {
        (DOCTYPE)
        head {
            (header(title))
        }
        body {
            div class="h-full overflow-scroll" {
                div class="container space-y-8 flex flex-col items-center !max-w-6xl mx-auto p-4" {
                    (html)
                }
            }
        }
    }
}

async fn root() -> Markup {
    page(
        "Server Tester",
        html! {
            div class="grid grid-flow-col gap-5 text-center auto-cols-max" {
                div class="flex flex-col p-2 bg-neutral rounded-box text-neutral-content" {
                    span class="countdown font-mono text-5xl" {
                        span id="hours" style="--value:00;"{}
                    }
                    "hours"
                }
                div class="flex flex-col p-2 bg-neutral rounded-box text-neutral-content" {
                    span class="countdown font-mono text-5xl" {
                        span  id="minutes" style="--value:00;"{}
                    }
                    "min"
                }
                div class="flex flex-col p-2 bg-neutral rounded-box text-neutral-content" {
                    span class="countdown font-mono text-5xl" {
                        span id="seconds" style="--value:00;"{}
                    }
                    "sec"
                }
            }
            p {
                "Loaded Successfully! Look into logs for more details!"
            }
            script {
                "function updateCountdown() {
                    const now = new Date();
                    const hours = now.getHours();
                    const minutes = now.getMinutes();
                    const seconds = now.getSeconds();

                    document.getElementById('hours').style.setProperty('--value', hours);
                    document.getElementById('minutes').style.setProperty('--value', minutes);
                    document.getElementById('seconds').style.setProperty('--value', seconds);
                }

                updateCountdown();
                setInterval(updateCountdown, 1000);"
            }
        },
    )
}

/// Mounting Static Files
async fn static_assets(
    State(dir): State<PathBuf>,
    Path(file): Path<String>,
    req: Request<Body>,
) -> impl IntoResponse {
    let path = dir.join(&file);
    ServeFile::new(path)
        .oneshot(req)
        .await
        .unwrap()
        .into_response()
}
