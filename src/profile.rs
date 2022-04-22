#![allow(non_snake_case)] // To rustc: no. I have my own standards for naming.

use std::collections::HashMap;
use rocket::{http::{Cookie, CookieJar, Status}, fairing::AdHoc};
use rocket_dyn_templates::Template;

use crate::res::RES;
use crate::user::{lookupUserIDFromUsername, userExists, lookupUsernameFromUserID};

#[get("/<userid>", rank = 1)]
pub fn profileUserID(userid: i128) -> RES {
    if !userExists(userid) {
        return RES::S(Status { code: 404 });
    }

    let username = lookupUsernameFromUserID(userid);
    let suserid = &userid.to_string();

    let mut a: HashMap<&str, &str> = HashMap::new();

    a.insert("userid", suserid);
    a.insert("username", username);

    RES::T(Template::render("profile", &a))
}

#[get("/<username>", rank = 0)]
pub fn profileUsername(username: &str) -> RES {
    let userid = lookupUserIDFromUsername(username);

    if userid == -1 {
        return RES::S(Status { code: 404 });
    }

    profileUserID(userid)
}

#[get("/", rank = 2)]
pub fn profileNone(cookies: &CookieJar<'_>) -> RES {
    let userid = match cookies.get("user_id") {
        None => 0,
        Some(x) => x
    };
    profileUserID(userid)
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Profile Stage", |rocket| async {
        rocket.mount("/profile", routes![profileNone, profileUserID, profileUsername])
    })
}