#![allow(non_snake_case)] // To rustc: no. I have my own standards for naming.


use std::collections::HashMap;
use rocket::{http::{Cookie, CookieJar, Status}, form::Form, fairing::AdHoc, response::Redirect};
use rocket_dyn_templates::Template;

use crate::res::RES;
use crate::user::{lookupUserIDFromUsername, userExists, lookupUsernameFromUserID, descriptionFromUserID, hashPassword, getPasswordHash, createUser};

#[derive(FromForm)]
pub struct LoginForm<'r> {
   #[field(validate = len(1..))]
   username: &'r str,
   password: &'r str
}

#[derive(FromForm)]
pub struct RegisterForm<'r> {
   #[field(validate = len(1..))]
   username: &'r str,
   password: &'r str,
   email:    &'r str
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

#[get("/register")]
pub fn registerPage() -> RES {
    let mut context: HashMap<&str, &str> = HashMap::new();
    context.insert("error", "");
    RES::T(Template::render("register", &context))
}

#[post("/register", data="<registerform>")]
pub fn registerPagePost(registerform: Form<RegisterForm<'_>>) -> RES {
    let username = registerform.username;
    let password = registerform.password;
    let email    = registerform.email;

    if userExists(lookupUserIDFromUsername(username)) {
        let mut context: HashMap<&str, &str> = HashMap::new();
        context.insert("error", "Username already exists");
        return RES::T(Template::render("register", &context));
    }

    createUser(username, password, email);

    RES::R(Redirect::to(uri!("/login")))
}

#[get("/logout")]
pub fn logoutPage(cookies: &CookieJar<'_>) -> RES{
    cookies.remove(Cookie::named("user_id"));
    RES::R(Redirect::to("/"))
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Login Stage", |rocket| async {
        rocket.mount("/", routes![loginPage, loginPagePost, registerPage, registerPagePost, logoutPage])
    })
}