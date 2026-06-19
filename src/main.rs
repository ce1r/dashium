use dashium::Database;
use dotenvy::dotenv;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    Database::init().await?;

    let app = dashium::setup();
    let listener = TcpListener::bind("0.0.0.0:64842").await?;

    axum::serve(listener, app).await?;

    Ok(())
}
