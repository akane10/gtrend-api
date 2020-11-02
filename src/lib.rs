#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

mod helpers;
mod routes;

use rocket::request::Request;

#[catch(500)]
fn internal_error() -> &'static str {
    "Whoops! Looks like we messed up."
}
#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Couldn't find '{}'. Try something else?", req.uri())
}
pub fn rocket_app() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![routes::repo::repo_index,])
        .mount(
            "/repositories",
            routes![routes::repo::repo_index, routes::repo::repo_repositories,],
        )
        .mount("/developers", routes![routes::developers::developers])
        .mount("/languages", routes![routes::languages::languages])
        .mount(
            "/spoken_languages",
            routes![routes::spoken_languages::spoken_languages],
        )
        .register(catchers![not_found, internal_error])
}
