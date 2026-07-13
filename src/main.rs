use dashium::Database;
use dotenvy::dotenv;
use tokio::net::TcpListener;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
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
    let listener = TcpListener::bind("127.0.0.1:64842").await?;

    axum::serve(listener, app).await?;

    Ok(())
}
