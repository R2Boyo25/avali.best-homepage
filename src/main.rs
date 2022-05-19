#![allow(non_snake_case)] // To rustc: no. I have my own standards for naming.

#[macro_use] extern crate rocket;

use std::collections::HashMap;

use rocket::{Request, Data, Route, route};
use rocket::{http::Status};
use rocket::fs::{FileServer, relative};
use rocket::http::Method::Get;
use rocket_dyn_templates::Template;

mod profile;
mod res;
mod user;
mod login;
mod cassandra;

fn embedPage<'a>(req: &'a Request, _: Data) -> route::BoxFuture<'a> {
    let mut uri: String = req.uri().to_string();
    uri.replace_range(..5, "");

    let e = match uri.find("/") {
        None => uri.chars().count(),
        Some(x) => x,
    };

    let page = &uri.clone()[..e];

    uri.replace_range(..e, "");

    if vec!["art", "chat", "wiki", "chars"].contains(&page) {
        let mut a: HashMap<&str, &str> = HashMap::new();
        a.insert("page", &page);
        a.insert("pageargs", &uri);
        route::Outcome::from(req, Template::render("embed", &a)).pin()
    } else {
        route::Outcome::from(req, Status { code: 404 }).pin()
    }
}

#[get("/")]
fn index() -> res::RES {
    let a: HashMap<&str, &str> = HashMap::new();
    res::RES::T(Template::render("main", a))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/source", FileServer::from(relative!("source")))
        .mount("/", routes![index])
        .mount("/", vec![Route::new(Get, "/sub/<anything..>", embedPage)])
        .attach(profile::stage())
        .attach(login::stage())
        .attach(Template::fairing())

}