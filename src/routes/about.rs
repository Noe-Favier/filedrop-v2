use rocket_dyn_templates::{context, Template};
use rocket::response::content;

#[get("/about")]
pub fn about() -> content::RawHtml<Template> {
    content::RawHtml(Template::render(
        "about",
        context! { index: "inactive", about:"active" },
    ))
}