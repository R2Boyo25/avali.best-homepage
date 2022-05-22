#![allow(non_snake_case)] // To rustc: no. I have my own standards for naming.

use rocket::{http::Status, response::Redirect};
use rocket_dyn_templates::Template;

#[derive(Debug, Responder)]
pub enum RES {
    R(Redirect),
    T(Template),
    S(Status),
    STR(&'static str)
}