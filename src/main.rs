#[macro_use] extern crate rocket;
use rocket::Build;
use rocket_dyn_templates::Template;
use rocket_dyn_templates::context;
use rocket::Rocket;

#[get("/")]
fn index() -> Template {
    let context = context! {
        name: "kieran",
        age: 22
    };
    Template::render("index", &context)
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![index])
        .attach(Template::fairing())
}
