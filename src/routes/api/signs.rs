use tuono_app::signs::Sign;
use tuono_lib::{axum::Json, Request};

use serde_json::{json, Value};

#[tuono_lib::api(GET)]
async fn signs_list(_req: Request) -> Json<Value> {
    let signs = Sign::signs();

    Json(json!(signs))
}
