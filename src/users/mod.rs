#![allow(clippy::unit_arg)] // False positives.

use crate::{
    authentication,
    schema::{self, users::dsl::*},
    MainDbConn,
};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::{
    get,
    http::{Cookie, Cookies, Status},
    post, put,
};
use rocket_contrib::json::Json;
use serde::Deserialize;
use sodiumoxide::crypto::pwhash::argon2id13;
use std::error::Error;

pub mod labelsets;
pub mod quizzes;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Login {
    pub username: String,
    pub password: String,
}

#[post("/login", format = "json", data = "<data>")]
pub fn login(
    conn: MainDbConn,
    mut cookies: Cookies,
    data: Json<Login>,
) -> Result<Status, Box<dyn Error>> {
    let results = users
        .filter(username.eq(&data.username))
        .load::<crate::models::User>(&*conn)?;

    let user = if let Some(user) = results.get(0) {
        user
    } else {
        return Ok(Status::Unauthorized);
    };

    sodiumoxide::init().map_err(|_| "Failed to init sodiumoxide.")?;
    let hash = argon2id13::HashedPassword::from_slice(&user.password)
        .ok_or("Could not recover password hash")?;
    let password_matches = argon2id13::pwhash_verify(&hash, data.password.as_bytes());

    if !password_matches {
        return Ok(Status::Unauthorized);
    }

    add_login_cookie(&mut cookies, user.id);
    Ok(Status::Ok)
}

#[post("/logout")]
pub fn logout(mut cookies: Cookies) -> Result<(), !> {
    remove_login_cookie(&mut cookies);
    Ok(())
}

#[put("/create", format = "json", data = "<data>")]
pub fn create(
    conn: MainDbConn,
    data: Json<Login>,
    _auth: authentication::Admin,
) -> Result<Status, Box<dyn Error>> {
    sodiumoxide::init().map_err(|_| "Failed to init sodiumoxide.")?;
    let hash = argon2id13::pwhash(
        data.password.as_bytes(),
        argon2id13::OPSLIMIT_INTERACTIVE,
        argon2id13::MEMLIMIT_INTERACTIVE,
    )
    .map_err(|_| "Failed to hash password.")?;

    let insert = super::models::NewUser {
        username: data.username.as_ref(),
        password: hash.as_ref(),
    };

    let result = rocket_contrib::databases::diesel::insert_into(schema::users::table)
        .values(&insert)
        .execute(&*conn);

    // Explicitly return HTTP 409 "Conflict" if the user already exists.
    match result {
        Ok(_) => {}
        Err(diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::UniqueViolation,
            _,
        )) => return Ok(Status::Conflict),
        Err(e) => return Err(e.into()),
    }

    users
        .filter(username.eq(insert.username))
        .load::<crate::models::User>(&*conn)?;

    Ok(Status::Ok)
}

#[get("/isadmin", rank = 1)]
pub fn is_admin(_admin: authentication::Admin) -> Json<bool> {
    Json(true)
}

#[get("/isadmin", rank = 2)]
pub fn is_not_admin(_user: &authentication::User) -> Json<bool> {
    Json(false)
}

#[get("/ismoderator", rank = 1)]
pub fn is_moderator(_moderator: authentication::Moderator) -> Json<bool> {
    Json(true)
}

#[get("/ismoderator", rank = 2)]
pub fn is_not_moderator(_user: &authentication::User) -> Json<bool> {
    Json(false)
}

#[post("/refresh", rank = 1)]
pub fn refresh_session_user(user: &authentication::User, mut cookies: Cookies) {
    let user_id = user.0.id;
    remove_login_cookie(&mut cookies);
    add_login_cookie(&mut cookies, user_id);
}

#[post("/refresh", rank = 2)]
pub fn refresh_session_loggedout(
    mut cookies: Cookies,
) -> rocket::response::status::Unauthorized<()> {
    remove_login_cookie(&mut cookies);
    rocket::response::status::Unauthorized(None)
}

fn add_login_cookie(cookies: &mut Cookies, user_id: i32) {
    cookies.add_private(Cookie::new("user_id", user_id.to_string()));
}

fn remove_login_cookie(cookies: &mut Cookies) {
    if let Some(cookie) = cookies.get_private("user_id") {
        cookies.remove_private(cookie);
    }
}
