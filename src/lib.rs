#![allow(non_snake_case)]

use axum::Router;

mod database;
mod error;
mod post;
mod user;
mod util;

pub use database::Database;
pub use error::Result;

pub fn setup() -> Router {
    Router::new().merge(routes())
}

pub fn routes() -> Router {
    Router::new().merge(user::routes()).merge(post::routes())
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
