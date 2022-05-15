#![allow(non_snake_case)] // To rustc: no. I have my own standards for naming.


use std::collections::HashMap;
use rocket::{http::{Cookie, CookieJar, Status}, form::Form, fairing::AdHoc, response::Redirect};
use rocket_dyn_templates::Template;

use crate::res::RES;
use crate::user::{lookupUserIDFromUsername, userExists, lookupUsernameFromUserID, descriptionFromUserID, hashPassword, getPasswordHash};

#[derive(FromForm)]
pub struct LoginForm<'r> {
   #[field(validate = len(1..))]
   username: &'r str,
   password: &'r str
}

#[get("/login")]
pub fn loginPage() -> RES {
    let mut context: HashMap<&str, &str> = HashMap::new();
    context.insert("error", "");
    RES::T(Template::render("login", &context))
}

#[post("/login", data = "<loginform>")]
pub fn loginPagePost(loginform: Form<LoginForm<'_>>, cookies: &CookieJar<'_>) -> RES {
    let username = loginform.username;
    let password = loginform.password;

    if !userExists(lookupUserIDFromUsername(username)) {
        let mut context: HashMap<&str, &str> = HashMap::new();
        context.insert("error", "User does not exist.");
        return RES::T(Template::render("login", &context));
    }

    if hashPassword(password, username) != getPasswordHash(username) {
        let mut context: HashMap<&str, &str> = HashMap::new();
        context.insert("error", "Incorrect password or username.");
        return RES::T(Template::render("login", &context));
    }

    cookies.add(Cookie::build("user_id", lookupUserIDFromUsername(username).to_string()).finish());
    RES::R(Redirect::to(uri!("/")))
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Login Stage", |rocket| async {
        rocket.mount("/", routes![loginPage, loginPagePost])
    })
}