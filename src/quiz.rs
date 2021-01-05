use crate::{
    models,
    schema::{questions::dsl as questions_dsl, quizzes::dsl as quizzes_dsl},
    util, MainDbConn,
};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::{delete, get, post, put};
use rocket_contrib::{json::Json, uuid::Uuid};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct Quiz {
    pub label_set: String,
    pub shuffle: bool,
    pub questions: Vec<Question>,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct Question {
    pub question_type: i16,
    pub text_prompt: String,
    pub text_answer: Option<String>,
    pub label_id: Option<i32>,
    pub show_regions: Option<bool>,
}

impl Quiz {
    pub fn to_db_quiz<'a>(&self, uuid: &'a str, label_set_id: i32) -> models::NewQuiz<'a> {
        models::NewQuiz {
            labelset: label_set_id,
            shuffle: self.shuffle as i16,
            uuid,
        }
    }

    pub fn to_db_questions(&'_ self, quiz_id: i32) -> Vec<models::NewQuestion<'_>> {
        self.questions
            .iter()
            .map(|q| models::NewQuestion {
                quiz: quiz_id,
                questiontype: q.question_type as i16,
                textprompt: q.text_prompt.as_ref(),
                textanswer: q.text_answer.as_ref().map(|s| s.as_str()),
                label: q.label_id,
                showregions: q.show_regions.map(|_| 1).unwrap_or(0),
            })
            .collect()
    }
}

impl From<(models::Quiz, Vec<models::Question>)> for Quiz {
    fn from(_: (models::Quiz, Vec<models::Question>)) -> Self {
        todo!()
    }
}

#[get("/<uuid>")]
pub fn load(conn: MainDbConn, uuid: Uuid) -> Result<Option<Json<Quiz>>, Box<dyn Error>> {
    let quiz = quizzes_dsl::quizzes
        .filter(quizzes_dsl::uuid.eq(&uuid.to_string()))
        .limit(1)
        .load::<crate::models::Quiz>(&*conn)?
        .pop();

    let quiz = match quiz {
        Some(q) => q,
        None => return Ok(None),
    };

    let questions = questions_dsl::questions
        .filter(questions_dsl::quiz.eq(&quiz.id))
        .load::<crate::models::Question>(&*conn)?;

    Ok(Some(Json((quiz, questions).into())))
}

#[post("/", format = "json", data = "<data>")]
pub fn create(conn: MainDbConn, data: Json<Quiz>) -> Result<Option<Json<String>>, Box<dyn Error>> {
    put(conn, util::create_uuid(), data)
}

#[put("/<uuid>", format = "json", data = "<data>")]
pub fn put(
    conn: MainDbConn,
    uuid: Uuid,
    data: Json<Quiz>,
) -> Result<Option<Json<String>>, Box<dyn Error>> {
    use crate::schema::labelsets::dsl as labelset_dsl;

    let quiz = data.into_inner();
    let uuid = uuid.to_string();

    let label_set = labelset_dsl::labelsets
        .filter(labelset_dsl::uuid.eq(&uuid))
        .limit(1)
        .load::<crate::models::LabelSet>(&*conn)?
        .pop();

    let label_set_id = if let Some(set) = label_set {
        set.id
    } else {
        return Ok(None);
    };

    let dbquiz = quiz.to_db_quiz(&uuid, label_set_id);
    rocket_contrib::databases::diesel::insert_into(quizzes_dsl::quizzes)
        .values(&dbquiz)
        .execute(&*conn)?;

    // Get the ID for the inserted set to apply to the questions.
    let inserted_quiz = quizzes_dsl::quizzes
        .limit(1)
        .load::<crate::models::Quiz>(&*conn)?
        .pop()
        .ok_or("Can't find quiz that was just inserted.")?;

    let questions = quiz.to_db_questions(inserted_quiz.id);

    rocket_contrib::databases::diesel::insert_into(questions_dsl::questions)
        .values(&questions)
        .execute(&*conn)?;

    Ok(Some(Json(uuid)))
}

#[delete("/<uuid>")]
pub fn delete(conn: MainDbConn, uuid: Uuid) -> Result<Option<()>, Box<dyn Error>> {
    let uuid = uuid.to_string();
    let quiz = quizzes_dsl::quizzes
        .filter(quizzes_dsl::uuid.eq(&uuid))
        .limit(1)
        .load::<crate::models::Quiz>(&*conn)?
        .pop();
    let quiz = match quiz {
        Some(q) => q,
        None => return Ok(None),
    };

    rocket_contrib::databases::diesel::delete(quizzes_dsl::quizzes)
        .filter(quizzes_dsl::uuid.eq(&uuid))
        .execute(&*conn)?;
    rocket_contrib::databases::diesel::delete(questions_dsl::questions)
        .filter(questions_dsl::quiz.eq(&quiz.id))
        .execute(&*conn)?;

    Ok(Some(()))
}
