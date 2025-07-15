#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use loco_rs::prelude::*;
use axum::debug_handler;

use crate::views;

#[debug_handler]
async fn index(
    ViewEngine(v): ViewEngine<TeraView>,
    State(_ctx): State<AppContext>,
) -> Result<Response> {
    views::home::home(&v)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/")
        .add("/", get(index))
}
