use crate::{
    authentication, diesel::BoolExpressionMethods, models::UserLabelSet, schema, MainDbConn,
};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::{
    delete, get,
    http::{Cookie, Cookies},
    post, put,
};
use rocket_contrib::{json::Json, uuid::Uuid};
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

    rocket_contrib::databases::diesel::insert_into(schema::users::table)
        .values(&insert)
        .execute(&*conn)?;

    Ok(())
}

#[derive(Debug, serde::Serialize)]
pub struct JsonUserLabelSets {
    pub name: String,
    pub id: i32,
}

impl From<crate::models::LabelSet> for JsonUserLabelSets {
    fn from(set: crate::models::LabelSet) -> Self {
        Self {
            id: set.id,
            name: set.name,
        }
    }
}

#[put("/labelsets/<uuid>")]
pub fn add_labelset(
    conn: MainDbConn,
    uuid: Uuid,
    user: &authentication::User,
) -> Result<Option<()>, Box<dyn Error>> {
    let set = schema::labelsets::dsl::labelsets
        .filter(schema::labelsets::dsl::uuid.eq(&uuid.to_string()))
        .limit(1)
        .load::<crate::models::LabelSet>(&*conn)?
        .pop();
    if set.is_none() {
        return Ok(None);
    }
    let set = set.unwrap();

    let data = UserLabelSet {
        userid: user.0.id,
        labelset: set.id,
    };

    rocket_contrib::databases::diesel::insert_into(schema::userlabelsets::table)
        .values(&data)
        .execute(&*conn)?;

    Ok(Some(()))
}

#[delete("/labelsets/<uuid>")]
pub fn delete_labelset(
    conn: MainDbConn,
    uuid: Uuid,
    user: &authentication::User,
) -> Result<Option<()>, Box<dyn Error>> {
    use schema::userlabelsets::dsl::{labelset, userid};

    let set = schema::labelsets::dsl::labelsets
        .filter(schema::labelsets::dsl::uuid.eq(&uuid.to_string()))
        .limit(1)
        .load::<crate::models::LabelSet>(&*conn)?
        .pop();
    if set.is_none() {
        return Ok(None);
    }
    let set = set.unwrap();

    let filter1 = labelset.eq(&set.id);
    let filter2 = userid.eq(&user.0.id);
    let deleted = rocket_contrib::databases::diesel::delete(schema::userlabelsets::table)
        .filter(filter1.and(filter2))
        .execute(&*conn)?;

    match deleted {
        0 => Ok(None),
        1 => Ok(Some(())),
        n => Err(format!("Expected 1 deleted userset, but deleted {}!", n).into()),
    }
}

#[get("/labelsets")]
pub fn get_labelsets(
    conn: MainDbConn,
    user: &authentication::User,
) -> Result<Json<Vec<JsonUserLabelSets>>, Box<dyn Error>> {
    let set_ids: Vec<_> = schema::userlabelsets::dsl::userlabelsets
        .filter(schema::userlabelsets::dsl::userid.eq(&user.0.id))
        .load::<crate::models::UserLabelSet>(&*conn)?
        .into_iter()
        .map(|uls| uls.labelset)
        .collect();

    let result: Vec<_> = schema::labelsets::dsl::labelsets
        .filter(schema::labelsets::dsl::id.eq_any(&set_ids))
        .load::<crate::models::LabelSet>(&*conn)?
        .into_iter()
        .map(From::from)
        .collect();

    Ok(Json(result))
}
