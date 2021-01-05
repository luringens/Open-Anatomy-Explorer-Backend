use crate::{authentication, models::NewModel, schema::models::dsl, MainDbConn};
use diesel::RunQueryDsl;
use rocket::{put, Data};
use std::{error::Error, io::Read};

#[put("/upload/<filename>", data = "<data>")]
pub fn upload(
    conn: MainDbConn,
    filename: String,
    data: Data,
    _admin: authentication::Admin,
) -> Result<String, Box<dyn Error>> {
    let mut data_dir = std::env::var("MODELS_DIR")
        .map(std::path::PathBuf::from)
        .unwrap();
    data_dir.push(&filename);
    let path = data_dir.to_str().ok_or("Invalid path")?;

    let mut file = std::fs::File::create(path)?;
    let mut stream = data.open().take(50 * 1024u64.pow(2)); // 50 MiB
    let written = std::io::copy(&mut stream, &mut file)?;

    let filename = &filename;
    rocket_contrib::databases::diesel::insert_into(dsl::models)
        .values(&NewModel { filename })
        .execute(&*conn)?;

    Ok(written.to_string())
}
