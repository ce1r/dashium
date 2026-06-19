#![allow(non_snake_case)]

use axum::Router;

mod database;
mod error;
mod user;
mod util;

pub use database::Database;
pub use error::Result;

pub fn setup() -> Router {
    Router::new().merge(routes())
}

pub fn routes() -> Router {
    Router::new().merge(user::routes())
}
