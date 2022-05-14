#[macro_use]
extern crate rocket;
use rocket_dyn_templates::{context, Template};

#[get("/")]
fn index() -> Template {
  Template::render(
    "index",
    context! {
      title :"Home",
      conten : "Hello, world!"
    },
  )
}

#[get("/about")]
fn indexabout() -> &'static str {
  "About"
}

#[post("/about")]
fn createindexabout() -> &'static str {
  "About"
}

#[launch]
fn rocket() -> _ {
  rocket::build()
    .mount("/", routes![index, indexabout, createindexabout])
    .attach(Template::fairing())
}
