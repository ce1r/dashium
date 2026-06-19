use cornucopia::deadpool_postgres;
use cornucopia::deadpool_postgres::Config;
use cornucopia::deadpool_postgres::Object;
use cornucopia::deadpool_postgres::Pool;
use cornucopia::deadpool_postgres::Runtime;
use cornucopia::tokio_postgres::NoTls;
use std::env;
use tokio::sync::OnceCell;

static POOL: OnceCell<Pool> = OnceCell::const_new();

pub struct Database;

impl Database {
    pub async fn init() -> anyhow::Result<()> {
        POOL.get_or_try_init(|| async {
            let db_url = env::var("DATABASE_URL")?;

            let mut cfg = Config::new();
            cfg.url = Some(db_url);

            let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls)?;

            Ok::<Pool, anyhow::Error>(pool)
        })
        .await?;

        Ok(())
    }

    pub async fn acquire() -> anyhow::Result<Object, deadpool_postgres::PoolError> {
        let pool = POOL.get().expect("Database::init must be called first");
        let client = pool.get().await?;

        Ok(client)
    }
}
