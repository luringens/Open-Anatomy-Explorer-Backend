use crate::{authentication, models::NewModel, schema::models::dsl, MainDbConn};
use diesel::{ExpressionMethods, RunQueryDsl};
use rocket::{get, put, Data};
use rocket_contrib::json::Json;
use std::{error::Error, io::Read};

const MIB: u64 = 1024u64.pow(2);
const UPLOAD_SIZE_LIMIT: u64 = 75 * MIB;

#[put("/upload/<filename>", data = "<data>")]
pub fn upload(
    admin: authentication::Admin,
    conn: MainDbConn,
    filename: String,
    data: Data,
) -> Result<Json<u64>, Box<dyn Error>> {
    let written = store_file(admin, &filename, data)?;
    let filename = &filename;
    rocket_contrib::databases::diesel::insert_into(dsl::models)
        .values(&NewModel {
            filename,
            ..Default::default()
        })
        .execute(&*conn)?;

    Ok(Json(written))
}

#[put("/upload/mtl/<id>/<filename>", data = "<data>")]
pub fn upload_material(
    admin: authentication::Admin,
    conn: MainDbConn,
    id: i32,
    filename: String,
    data: Data,
) -> Result<Json<u64>, Box<dyn Error>> {
    use diesel::QueryDsl;
    let written = store_file(admin, &filename, data)?;

    let target = dsl::models.filter(dsl::id.eq(&id));
    rocket_contrib::databases::diesel::update(target)
        .set(dsl::material.eq(&filename))
        .execute(&*conn)?;

    Ok(Json(written))
}

#[put("/upload/tex/<id>/<filename>", data = "<data>")]
pub fn upload_texture(
    admin: authentication::Admin,
    conn: MainDbConn,
    id: i32,
    filename: String,
    data: Data,
) -> Result<Json<u64>, Box<dyn Error>> {
    use diesel::QueryDsl;
    let written = store_file(admin, &filename, data)?;

    let target = dsl::models.filter(dsl::id.eq(&id));
    rocket_contrib::databases::diesel::update(target)
        .set(dsl::texture.eq(&filename))
        .execute(&*conn)?;

    Ok(Json(written))
}

pub fn store_file(
    _admin: authentication::Admin,
    filename: &str,
    data: Data,
) -> Result<u64, Box<dyn Error>> {
    let mut data_dir = std::env::var("MODELS_DIR")
        .map(std::path::PathBuf::from)
        .unwrap();
    data_dir.push(&filename);
    let path = data_dir.to_str().ok_or("Invalid path")?;

    let mut file = std::fs::File::create(path)?;
    let mut stream = data.open().take(UPLOAD_SIZE_LIMIT);
    let written = std::io::copy(&mut stream, &mut file)?;

    Ok(written)
}

#[get("/")]
pub fn list(
    _auth: &authentication::User,
    conn: MainDbConn,
) -> Result<Json<Vec<crate::models::Model>>, Box<dyn Error>> {
    let models = dsl::models.load::<crate::models::Model>(&*conn)?;
    Ok(Json(models))
}

#[get("/lookup/<id>")]
pub fn lookup(
    _auth: &authentication::User,
    conn: MainDbConn,
    id: i32,
) -> Result<Option<Json<crate::models::Model>>, Box<dyn Error>> {
    use diesel::query_dsl::filter_dsl::FindDsl;
    let model = dsl::models
        .find(&id)
        .load::<crate::models::Model>(&*conn)?
        .pop()
        .map(|model| Json(model));

    Ok(model)
}
