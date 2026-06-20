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

#[macro_export]
macro_rules! gd_format {
    () => {
        String::new()
    };

    ($($key:literal => $value:expr),+ $(,)?) => {{
        let mut s = String::new();

        $(
            if !s.is_empty() {
                s.push(':');
            }

            let _ = ::std::fmt::Write::write_fmt(&mut s, format_args!("{}:{}", $key, $value));
        )*

        s
    }};
}
