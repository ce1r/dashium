#![forbid(unsafe_code)]

use dashium::Database;
use dashium::Result;
use dotenvy::dotenv;
use tokio::net::TcpListener;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

const URL: &str = "127.0.0.1:64842";

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "axum_app=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    Database::init().await?;

    let app = dashium::setup();
    let listener = TcpListener::bind(URL).await?;

    tracing::info!("Server running at: http://{URL}");
    axum::serve(listener, app).await?;

    Ok(())
}
