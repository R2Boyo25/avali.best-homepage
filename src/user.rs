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

pub fn descriptionFromUserID(userid: i128) -> &'static str {
    if userid == 0 {
        return "HELLO IM KAZANI AND THIS IS A TEST FOR THE PROFILE PAGES AND IM GOING TO KEEP TYPING IN CAPS TO SEE HOW WRAPPING WORKS AAAAAAA EEEEEEE IIIIII OOOOOO UUUUUU";
    } else {
        ""
    }
}