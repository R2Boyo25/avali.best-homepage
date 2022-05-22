use std::collections::HashMap;
use rocket::{http::{CookieJar, Status}, fairing::AdHoc, response::Redirect};
use rocket_dyn_templates::Template;

use crate::res::RES;
use crate::user::{lookupUserIDFromUsername, userExists, lookupUsernameFromUserID, descriptionFromUserID};

#[get("/<username>/pfp", rank = 2)]
pub fn profilePictureUsername(username: &str) -> RES {
    RES::STR("")
}

#[get("/<id>/pfp", rank = 1)]
pub fn profilePictureId(id: i128) -> RES {
    RES::STR("")
}

#[get("/<username>/banner", rank = 2)]
pub fn userBannerUsername(username: &str) -> RES {
    RES::STR("")
}

#[get("/<id>/banner", rank = 1)]
pub fn userBannerId(id: &str) -> RES {
    RES::STR("")
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("User File Stage", |rocket| async {
        rocket.mount("/", routes![profilePictureUsername, profilePictureId, userBannerUsername, userBannerId])
    })
}