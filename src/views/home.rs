use loco_rs::prelude::*;

pub fn home(v: &impl ViewRenderer) -> Result<Response> {
    format::render().view(v, "home/index.html", data!({}))
}
