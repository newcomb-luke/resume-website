#[macro_use] extern crate rocket;
use rocket::fs::FileServer;
use rocket_dyn_templates::{Template, context};

#[get("/")]
fn index() -> Template {
    Template::render("index", context! {} )
}

#[get("/resume")]
fn resume() -> Template {
    Template::render("resume", context! {} )
}

#[get("/portfolio")]
fn portfolio() -> Template {
    Template::render("portfolio", context! {} )
}

#[get("/contact")]
fn contact() -> Template {
    Template::render("contact", context! {} )
}

#[launch]
fn rocket() -> _ {
    rocket::build().attach(Template::fairing())
        .mount("/static", FileServer::from("static"))
        .mount("/", routes![index, resume, portfolio, contact])
}