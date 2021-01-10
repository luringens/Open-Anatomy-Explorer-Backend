use crate::{
    models::{NewLabel, NewLabelSet},
    util, MainDbConn,
};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::{delete, get, post, put};
use rocket_contrib::{json::Json, uuid::Uuid};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JsonLabelSet {
    pub id: Option<i32>,
    pub uuid: Option<String>,
    pub name: String,
    pub model: i32,
    pub labels: Vec<JsonLabel>,
}

impl JsonLabelSet {
    fn from_db(set: crate::models::LabelSet, labels: Vec<crate::models::Label>) -> Self {
        Self {
            id: Some(set.id),
            uuid: Some(set.uuid),
            name: set.name,
            model: set.model,
            labels: labels.into_iter().map(From::from).collect(),
        }
    }

    fn to_new_label_set<'a>(&'a self, uuid: &'a str) -> NewLabelSet<'a> {
        NewLabelSet {
            id: self.id,
            name: self.name.as_ref(),
            model: self.model,
            uuid,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JsonLabel {
    pub colour: String,
    pub name: String,
    pub vertices: String,
}

impl From<crate::models::Label> for JsonLabel {
    fn from(l: crate::models::Label) -> Self {
        Self {
            name: l.name,
            vertices: String::from_utf8(l.vertices).unwrap(),
            colour: l.colour,
        }
    }
}

impl<'a> From<&'a JsonLabel> for crate::models::NewLabel<'a> {
    fn from(p: &'a JsonLabel) -> Self {
        Self {
            colour: p.colour.as_ref(),
            labelset: Default::default(),
            name: p.name.as_ref(),
            vertices: p.vertices.as_bytes(),
        }
    }
}

#[post("/", format = "json", data = "<data>")]
pub fn create(conn: MainDbConn, data: Json<JsonLabelSet>) -> Result<Json<String>, Box<dyn Error>> {
    put(conn, util::create_uuid(), data)
}

#[put("/<uuid>", format = "json", data = "<data>")]
pub fn put(
    conn: MainDbConn,
    uuid: Uuid,
    data: Json<JsonLabelSet>,
) -> Result<Json<String>, Box<dyn Error>> {
    use crate::schema::labels::dsl::{self as labels_dsl, labels};
    use crate::schema::labelsets::dsl::{self as labelsets_dsl, labelsets};

    let data = data.into_inner();
    let uuid = (&uuid).to_string();
    let mut new_set = data.to_new_label_set(uuid.as_ref());
    let mut new_labels: Vec<_> = data.labels.iter().map(NewLabel::from).collect();

    // Check if it's already in the database, and if so, use it's ID.
    let set_id = data.id.or_else(|| {
        labelsets_dsl::labelsets
            .filter(labelsets_dsl::uuid.eq(&uuid))
            .load::<crate::models::LabelSet>(&*conn)
            .ok()
            .map(|mut sets| sets.pop().map(|set| set.id))
            .flatten()
    });
    new_set.id = set_id;

    rocket_contrib::databases::diesel::replace_into(labelsets)
        .values(&new_set)
        .execute(&*conn)?;

    // If we didn't previously get the ID for the set, retrieve it now to apply to the labels.
    let set_id = set_id
        .or_else(|| {
            labelsets_dsl::labelsets
                .filter(labelsets_dsl::uuid.eq(&uuid))
                .load::<crate::models::LabelSet>(&*conn)
                .ok()
                .map(|mut sets| sets.pop().map(|set| set.id))
                .flatten()
        })
        .ok_or("Can't find set that was just inserted.")?;

    new_labels.iter_mut().for_each(|l| l.labelset = set_id);

    rocket_contrib::databases::diesel::delete(labels)
        .filter(labels_dsl::labelset.eq(&set_id))
        .execute(&*conn)?;
    rocket_contrib::databases::diesel::insert_into(labels)
        .values(&new_labels)
        .execute(&*conn)?;

    Ok(Json(uuid))
}

#[get("/uuid/<uuid>")]
pub fn load_by_uuid(
    conn: MainDbConn,
    uuid: Uuid,
) -> Result<Option<Json<JsonLabelSet>>, Box<dyn Error>> {
    use crate::schema::labels::dsl as labels_dsl;
    use crate::schema::labelsets::dsl as labelsets_dsl;

    let uuid = uuid.to_string();
    let labelset = labelsets_dsl::labelsets
        .filter(labelsets_dsl::uuid.eq(&uuid))
        .limit(1)
        .load::<crate::models::LabelSet>(&*conn)?
        .pop();

    let labelset = match labelset {
        Some(l) => l,
        None => return Ok(None),
    };

    let labels: Vec<crate::models::Label> = labels_dsl::labels
        .filter(labels_dsl::labelset.eq(&labelset.id))
        .load::<crate::models::Label>(&*conn)?;

    let result = JsonLabelSet::from_db(labelset, labels);
    Ok(Some(Json(result)))
}

#[get("/<id>")]
pub fn load(conn: MainDbConn, id: i32) -> Result<Option<Json<JsonLabelSet>>, Box<dyn Error>> {
    use crate::schema::labels::dsl as labels_dsl;
    use crate::schema::labelsets::dsl as labelsets_dsl;

    let labelset = labelsets_dsl::labelsets
        .find(&id)
        .load::<crate::models::LabelSet>(&*conn)?
        .pop();

    let labelset = match labelset {
        Some(l) => l,
        None => return Ok(None),
    };

    let labels: Vec<crate::models::Label> = labels_dsl::labels
        .filter(labels_dsl::labelset.eq(&labelset.id))
        .load::<crate::models::Label>(&*conn)?;

    let result = JsonLabelSet::from_db(labelset, labels);
    Ok(Some(Json(result)))
}

#[delete("/<uuid>")]
pub fn delete(conn: MainDbConn, uuid: Uuid) -> Result<Option<()>, Box<dyn Error>> {
    use crate::schema::labels::dsl as labels_dsl;
    use crate::schema::labelsets::dsl as labelsets_dsl;

    let uuid = uuid.to_string();
    let labelset = labelsets_dsl::labelsets
        .filter(labelsets_dsl::uuid.eq(&uuid))
        .limit(1)
        .load::<crate::models::LabelSet>(&*conn)?
        .pop();
    let labelset = match labelset {
        Some(l) => l,
        None => return Ok(None),
    };

    rocket_contrib::databases::diesel::delete(labelsets_dsl::labelsets)
        .filter(labelsets_dsl::uuid.eq(&uuid))
        .execute(&*conn)?;
    rocket_contrib::databases::diesel::delete(labels_dsl::labels)
        .filter(labels_dsl::labelset.eq(&labelset.id))
        .execute(&*conn)?;

    Ok(Some(()))
}
