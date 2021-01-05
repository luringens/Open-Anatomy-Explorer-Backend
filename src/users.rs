use super::schema::*;
use crate::{authentication, schema, MainDbConn};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::{
    http::{Cookie, Cookies},
    post, put,
};
use rocket_contrib::json::Json;
use serde::Deserialize;
use sodiumoxide::crypto::pwhash::argon2id13;
use std::error::Error;

#[serde(rename_all = "camelCase")]
#[derive(Deserialize, Debug)]
pub struct Login {
    pub name: String,
    pub password: String,
}

#[post("/login", format = "json", data = "<data>")]
pub fn login(
    conn: MainDbConn,
    mut cookies: Cookies,
    data: Json<Login>,
) -> Result<(), Box<dyn Error>> {
    use schema::users::dsl::*;

    let results = users
        .filter(username.eq(&data.name))
        .load::<crate::models::User>(&*conn)?;
    let user = results.get(0).ok_or("Could not find user.")?;

    sodiumoxide::init().map_err(|_| "Failed to init sodiumoxide.")?;
    let hash = argon2id13::HashedPassword::from_slice(&user.password)
        .ok_or("Could not recover password hash")?;
    let password_matches = argon2id13::pwhash_verify(&hash, data.password.as_bytes());

    if !password_matches {
        return Err("Incorrect password.".into());
    }

    cookies.add_private(Cookie::new("user_id", user.id.to_string()));
    Ok(())
}

#[post("/logout")]
pub fn logout(_user: &authentication::User, mut cookies: Cookies) -> Result<(), !> {
    if let Some(cookie) = cookies.get_private("user_id") {
        cookies.remove_private(cookie);
    }
    Ok(())
}

#[put("/create", format = "json", data = "<data>")]
pub fn create(conn: MainDbConn, data: Json<Login>) -> Result<(), Box<dyn Error>> {
    sodiumoxide::init().map_err(|_| "Failed to init sodiumoxide.")?;
    let hash = argon2id13::pwhash(
        data.password.as_bytes(),
        argon2id13::OPSLIMIT_INTERACTIVE,
        argon2id13::MEMLIMIT_INTERACTIVE,
    )
    .map_err(|_| "Failed to hash password.")?;

    let insert = super::models::NewUser {
        username: data.name.as_ref(),
        password: hash.as_ref(),
    };

    rocket_contrib::databases::diesel::insert_into(users::table)
        .values(&insert)
        .execute(&*conn)?;

    Ok(())
}
