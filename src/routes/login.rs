use crate::{
    form::login_form::LoginForm,
    model::user::{User},
    repository::user_repository::get_user_by_username,
};
use rocket::serde::{json::Json};
use rocket::http::{CookieJar, Cookie};

#[post("/login", format = "json", data = "<login_form>")]
pub fn login(login_form: Json<LoginForm>, jar: &CookieJar<'_>) -> String {
    let username = login_form.0.username;
    let passwd = login_form.0.passwd;
    let user_result = get_user_by_username(username);
    
    if user_result.is_ok() {
        let user = user_result.unwrap();

        if user.check_password(passwd) {
            let token = User::sign(user);
            /* add cookie */
            jar.add(
                Cookie::build("token", token)
                    .http_only(true)
                    .secure(true)
                    .finish()
            );
            /* \\ */
            return String::from("Welcome back to Filedrop");
        }else{
            return String::from("bad password");
        }
    }
    return String::from("error");
}
