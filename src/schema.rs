table! {
    labels (id) {
        id -> Integer,
        labelset -> Integer,
        name -> Text,
        colour -> Text,
        vertices -> Binary,
    }
}

table! {
    labelsets (id) {
        id -> Integer,
        uuid -> Text,
        name -> Text,
        model -> Integer,
    }
}

table! {
    models (id) {
        id -> Integer,
        filename -> Text,
    }
}

table! {
    users (id) {
        id -> Integer,
        username -> Text,
        password -> Binary,
    }
}

allow_tables_to_appear_in_same_query!(
    labels,
    labelsets,
    models,
    users,
);
