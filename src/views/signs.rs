use loco_rs::prelude::*;

use crate::models::_entities::signs;

/// Render a list view of `signs`.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn list(v: &impl ViewRenderer, items: &Vec<signs::Model>) -> Result<Response> {
    format::render().view(v, "signs/list.html", data!({"items": items}))
}

/// Render a single `signs` view.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn show(v: &impl ViewRenderer, item: &signs::Model) -> Result<Response> {
    format::render().view(v, "signs/show.html", data!({"item": item}))
}

/// Render a `signs` create form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn create(v: &impl ViewRenderer) -> Result<Response> {
    format::render().view(v, "signs/create.html", data!({}))
}

/// Render a `signs` edit form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn edit(v: &impl ViewRenderer, item: &signs::Model) -> Result<Response> {
    format::render().view(v, "signs/edit.html", data!({"item": item}))
}
