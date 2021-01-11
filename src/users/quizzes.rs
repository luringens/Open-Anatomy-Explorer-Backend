use crate::{authentication, diesel::BoolExpressionMethods, models::UserQuiz, schema, MainDbConn};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::{delete, get, put};
use rocket_contrib::{json::Json, uuid::Uuid};
use std::error::Error;

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonUserQuiz {
    pub id: i32,
    pub uuid: String,
    pub name: String,
}

impl From<crate::models::Quiz> for JsonUserQuiz {
    fn from(quiz: crate::models::Quiz) -> Self {
        return Self {
            id: quiz.id,
            uuid: quiz.uuid,
            name: quiz.name,
        };
    }
}

#[put("/<uuid>")]
pub fn add(
    uuid: Uuid,
    user: &authentication::User,
    conn: MainDbConn,
) -> Result<Option<()>, Box<dyn Error>> {
    let quiz = schema::quizzes::dsl::quizzes
        .filter(schema::quizzes::dsl::uuid.eq(&uuid.to_string()))
        .limit(1)
        .load::<crate::models::Quiz>(&*conn)?
        .pop();
    if quiz.is_none() {
        return Ok(None);
    }
    let set = quiz.unwrap();

    let data = UserQuiz {
        userid: user.0.id,
        quiz: set.id,
    };

    rocket_contrib::databases::diesel::insert_into(schema::userquizzes::table)
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
    use schema::userquizzes::dsl::{quiz, userid};

    let set = schema::quizzes::dsl::quizzes
        .filter(schema::quizzes::dsl::uuid.eq(&uuid.to_string()))
        .limit(1)
        .load::<crate::models::Quiz>(&*conn)?
        .pop();
    if set.is_none() {
        return Ok(None);
    }
    let set = set.unwrap();

    let filter1 = quiz.eq(&set.id);
    let filter2 = userid.eq(&user.0.id);
    let deleted = rocket_contrib::databases::diesel::delete(schema::userquizzes::table)
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
) -> Result<Json<Vec<JsonUserQuiz>>, Box<dyn Error>> {
    let quiz_ids: Vec<_> = schema::userquizzes::dsl::userquizzes
        .filter(schema::userquizzes::dsl::userid.eq(&user.0.id))
        .load::<crate::models::UserQuiz>(&*conn)?
        .into_iter()
        .map(|uq| uq.quiz)
        .collect();

    let result: Vec<_> = schema::quizzes::dsl::quizzes
        .filter(schema::quizzes::dsl::id.eq_any(&quiz_ids))
        .load::<crate::models::Quiz>(&*conn)?
        .into_iter()
        .map(From::from)
        .collect();

    Ok(Json(result))
}
