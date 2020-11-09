pub fn create_uuid() -> rocket_contrib::uuid::Uuid {
    uuid::Uuid::new_v4()
        .to_string()
        .parse::<rocket_contrib::uuid::Uuid>()
        .unwrap()
}
