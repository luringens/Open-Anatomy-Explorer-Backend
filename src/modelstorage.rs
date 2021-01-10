use crate::{authentication, models::NewModel, schema::models::dsl, MainDbConn};
use diesel::{query_dsl::filter_dsl::FindDsl, RunQueryDsl};
use rocket::{get, put, Data};
use rocket_contrib::json::Json;
use std::{error::Error, io::Read};

const MIB: u64 = 1024u64.pow(2);
const UPLOAD_SIZE_LIMIT: u64 = 75 * MIB;

#[put("/upload/<filename>", data = "<data>")]
pub fn upload(
    _admin: authentication::Admin,
    conn: MainDbConn,
    filename: String,
    data: Data,
) -> Result<Json<String>, Box<dyn Error>> {
    let mut data_dir = std::env::var("MODELS_DIR")
        .map(std::path::PathBuf::from)
        .unwrap();
    data_dir.push(&filename);
    let path = data_dir.to_str().ok_or("Invalid path")?;

    let mut file = std::fs::File::create(path)?;
    let mut stream = data.open().take(UPLOAD_SIZE_LIMIT);
    let written = std::io::copy(&mut stream, &mut file)?;

    let filename = &filename;
    rocket_contrib::databases::diesel::insert_into(dsl::models)
        .values(&NewModel { filename })
        .execute(&*conn)?;

    Ok(Json(written.to_string()))
}

#[get("/")]
pub fn list(conn: MainDbConn) -> Result<Json<Vec<crate::models::Model>>, Box<dyn Error>> {
    let models = dsl::models.load::<crate::models::Model>(&*conn)?;
    Ok(Json(models))
}

#[get("/lookup/<id>")]
pub fn lookup(conn: MainDbConn, id: i32) -> Result<Option<Json<String>>, Box<dyn Error>> {
    let name = dsl::models
        .find(&id)
        .load::<crate::models::Model>(&*conn)?
        .pop()
        .map(|model| Json(model.filename));

    Ok(name)
}
