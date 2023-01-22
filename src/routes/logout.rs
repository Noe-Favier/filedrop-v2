use crate::{
    form::login_form::LoginForm, model::user::User,
    utils::user_session_manager::get_user_from_cookie,
};
use rocket::http::{Cookie, CookieJar};
use rocket::serde::json::Json;

#[post("/logout")]
pub fn logout(jar: &CookieJar<'_>) -> String {
    let user_result = get_user_from_cookie(jar.to_owned());

    if user_result.id != -1 {
        jar.remove(Cookie::named("token"));
        return String::from("Good Bye");
    }
    return String::from("You weren't connected");
}
