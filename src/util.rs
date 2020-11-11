use std::path::PathBuf;

pub fn create_uuid() -> rocket_contrib::uuid::Uuid {
    uuid::Uuid::new_v4()
        .to_string()
        .parse::<rocket_contrib::uuid::Uuid>()
        .unwrap()
}

pub fn json_path(path: &str, file: &str) -> PathBuf {
    PathBuf::from(path).join(format!("{}.{}", file, "json"))
}
