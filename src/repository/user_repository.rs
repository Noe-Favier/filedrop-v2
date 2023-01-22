use diesel::{RunQueryDsl, query_dsl::methods::FilterDsl};

use crate::{utils::db_connection_manager::establish_connection, repository::schema::fd_user::dsl::*, model::{self, user::User}};
use diesel::{ExpressionMethods, result::Error};

pub fn get_user_by_id(user_id: i32) -> Result<User, Error> {
    let mut conn = establish_connection();
    return fd_user.filter(id.eq(user_id)).first::<model::user::User>(&mut conn);
}

pub fn get_user_by_username(user_username: String) -> Result<User, Error> {
    let mut conn = establish_connection();
    return fd_user.filter(username.eq(user_username)).first::<model::user::User>(&mut conn);
}