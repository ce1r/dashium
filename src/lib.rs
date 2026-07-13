#![allow(non_snake_case)]

use axum::Router;
use tower_http::trace::TraceLayer;

mod database;
mod error;
mod level;
mod post;
mod reward;
mod social;
mod user;
mod util;

pub use database::Database;
pub use error::Result;

pub fn setup() -> Router {
    Router::new()
        .nest("/api/legacy", gd_routes())
        .layer(TraceLayer::new_for_http())
}

pub fn gd_routes() -> Router {
    Router::new()
        .merge(user::routes())
        .merge(post::routes())
        .merge(level::routes())
        .merge(reward::routes())
        .merge(social::routes())
}

#[macro_export]
macro_rules! gd_format {
    () => {
        String::new()
    };

    ($delim:literal, $($key:literal => $value:expr),+ $(,)?) => {{
        let mut s = String::new();

        $(
            if !s.is_empty() {
                s.push_str($delim);
            }

            let _ = ::core::fmt::Write::write_fmt(
                &mut s,
                format_args!("{}{}{}", $key, $delim, $value)
            );
        )*

        s
    }};
}
