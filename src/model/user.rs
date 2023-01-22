use rocket::config::Config;
use rocket_jwt::jwt;
use diesel::prelude::*;

use crate::utils::hash_utils::check;

static SECRET_KEY: &str = Config::SECRET_KEY;

#[jwt(SECRET_KEY, cookie = "token")]
#[derive(Queryable, Clone)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub pass_hash: String,
}

impl User {
    pub fn check_password(&self, password: String) -> bool{
        return check(password, self.pass_hash.as_str());
    }
}

impl UserJwtClaim {
    pub fn get_user(&self) -> User {
        return self.user.clone();
    }
}