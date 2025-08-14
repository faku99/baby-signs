use serde::Serialize;
use tuono_app::models::sign::Sign;
use tuono_lib::{Props, Request, Response};

#[derive(Serialize)]
struct SignsProps {
    results: Vec<Sign>
}

#[tuono_lib::handler]
async fn get_all_signs(_req: Request) -> Response {
    let signs = SignsProps { results: Sign::signs() };

    Response::Props(Props::new(signs))
}
