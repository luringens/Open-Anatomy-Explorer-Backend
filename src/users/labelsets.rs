use crate::{
    authentication, diesel::BoolExpressionMethods, models::UserLabelSet, schema, MainDbConn,
};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::{delete, get, put};
use rocket_contrib::{json::Json, uuid::Uuid};
use std::error::Error;

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonUserLabelSets {
    pub name: String,
    pub id: i32,
    pub uuid: String,
}

impl From<crate::models::LabelSet> for JsonUserLabelSets {
    fn from(set: crate::models::LabelSet) -> Self {
        Self {
            id: set.id,
            name: set.name,
            uuid: set.uuid,
        }
    }
}

#[put("/<uuid>")]
pub fn add(
    uuid: Uuid,
    user: &authentication::User,
    conn: MainDbConn,
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

#[delete("/<uuid>")]
pub fn delete(
    uuid: Uuid,
    user: &authentication::User,
    conn: MainDbConn,
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

        // Since we're effectively deleting by primary key, this should not be possible.
        n => Err(format!("Expected 1 deleted userset, but deleted {}!", n).into()),
    }
}

#[get("/")]
pub fn get(
    user: &authentication::User,
    conn: MainDbConn,
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
