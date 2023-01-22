use crate::model::user::{User};
use rocket::http::{CookieJar};

//************* UTIL FNs *************//

fn get_cookie_value(cookie_jar: CookieJar) -> String {
    if cookie_jar.get("token").is_some() {
        return cookie_jar.get("token").unwrap().value().to_owned();
    }
    return String::from("");
}

fn get_user_from_token(token: String) -> User{
    let token_decoded = User::decode(token);
    if token_decoded.is_ok() {
        let user_registered_in_token = token_decoded.unwrap().get_user();
        return user_registered_in_token;
    }
    return get_anonymous_user();
}

//************* EXPORTED FNs *************//

pub fn get_user_from_cookie(cookie_jar: CookieJar) -> User {
    let token = get_cookie_value(cookie_jar);
    return get_user_from_token(token);
}

pub fn get_anonymous_user() -> User {
    return User {
        id: -1,
        username: String::from("Anonymous"),
        pass_hash: String::new(),
    };
}