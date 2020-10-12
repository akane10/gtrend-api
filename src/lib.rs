#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

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
        .mount(
            "/",
            routes![
                routes::repo::repo_index,
                routes::repo::repo_repositories,
                routes::developers::developers
            ],
        )
        .register(catchers![not_found, internal_error])
}
