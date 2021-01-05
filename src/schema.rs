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
    questions (id) {
        id -> Integer,
        quiz -> Integer,
        questiontype -> SmallInt,
        textprompt -> Text,
        textanswer -> Nullable<Text>,
        label -> Nullable<Integer>,
        showregions -> SmallInt,
    }
}

table! {
    quizzes (id) {
        id -> Integer,
        uuid -> Text,
        labelset -> Integer,
        shuffle -> SmallInt,
    }
}

table! {
    userlabelsets (userid, labelset) {
        userid -> Integer,
        labelset -> Integer,
    }
}

table! {
    users (id) {
        id -> Integer,
        username -> Text,
        password -> Binary,
        privilege -> Integer,
    }
}

allow_tables_to_appear_in_same_query!(
    labels,
    labelsets,
    models,
    questions,
    quizzes,
    userlabelsets,
    users,
);
