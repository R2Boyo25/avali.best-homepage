#![allow(non_snake_case)] // To rustc: no. I have my own standards for naming.

pub fn lookupUserIDFromUsername(username: &str) -> i128 {
    0
}

pub fn userExists(userid: i128) -> bool {
    if userid < 0 {
        return false;
    } else {
        true
    }
}

pub fn lookupUsernameFromUserID(userid: i128) -> &'static str {
    if userid == 0 {
        return "Kazani";
    } else {
        ""
    }
}