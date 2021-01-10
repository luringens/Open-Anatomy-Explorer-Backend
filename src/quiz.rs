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
pub struct JsonQuiz {
    pub id: Option<i32>,
    pub label_set: i32,
    pub shuffle: bool,
    pub questions: Vec<JsonQuestion>,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct JsonQuestion {
    pub question_type: i16,
    pub text_prompt: String,
    pub text_answer: Option<String>,
    pub label_id: Option<i32>,
    pub show_regions: Option<bool>,
}

impl JsonQuiz {
    pub fn to_db_quiz<'a>(&self, uuid: &'a str) -> models::NewQuiz<'a> {
        models::NewQuiz {
            id: self.id,
            labelset: self.label_set,
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

impl From<(models::Quiz, Vec<models::Question>)> for JsonQuiz {
    fn from((quiz, questions): (models::Quiz, Vec<models::Question>)) -> Self {
        JsonQuiz {
            id: Some(quiz.id),
            label_set: quiz.labelset,
            shuffle: quiz.shuffle != 0,
            questions: questions
                .into_iter()
                .map(|q| JsonQuestion {
                    question_type: q.questiontype,
                    text_prompt: q.textprompt,
                    text_answer: q.textanswer,
                    label_id: q.label,
                    show_regions: Some(q.showregions != 0),
                })
                .collect(),
        }
    }
}

#[get("/<uuid>")]
pub fn load(conn: MainDbConn, uuid: Uuid) -> Result<Option<Json<JsonQuiz>>, Box<dyn Error>> {
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
pub fn create(
    conn: MainDbConn,
    data: Json<JsonQuiz>,
) -> Result<Option<Json<String>>, Box<dyn Error>> {
    put(conn, util::create_uuid(), data)
}

#[put("/<uuid>", format = "json", data = "<data>")]
pub fn put(
    conn: MainDbConn,
    uuid: Uuid,
    data: Json<JsonQuiz>,
) -> Result<Option<Json<String>>, Box<dyn Error>> {
    use crate::schema::labelsets::dsl as labelset_dsl;

    let quiz = data.into_inner();
    let uuid = uuid.to_string();

    // Make sure the label set exists.
    let label_set = labelset_dsl::labelsets
        .find(&quiz.label_set)
        .load::<crate::models::LabelSet>(&*conn)?
        .pop();
    if label_set.is_none() {
        return Ok(None);
    }

    // Check if there's a previous ID to overwrite..
    let previous_id: Option<i32> = quiz.id.or_else(|| {
        quizzes_dsl::quizzes
            .filter(quizzes_dsl::uuid.eq(&uuid.to_string()))
            .limit(1)
            .load::<crate::models::Quiz>(&*conn)
            .ok()
            .map(|mut sets| sets.pop().map(|set| set.id))
            .flatten()
    });

    if let Some(previous_id) = previous_id {
        rocket_contrib::databases::diesel::delete(questions_dsl::questions)
            .filter(questions_dsl::quiz.eq(&previous_id))
            .execute(&*conn)?;
    }

    let mut dbquiz = quiz.to_db_quiz(&uuid);
    dbquiz.id = previous_id;
    rocket_contrib::databases::diesel::insert_into(quizzes_dsl::quizzes)
        .values(&dbquiz)
        .execute(&*conn)?;

    // Get the ID for the inserted set if needed to apply to the questions.
    let previous_id = previous_id
        .or_else(|| {
            quizzes_dsl::quizzes
                .filter(quizzes_dsl::uuid.eq(&uuid.to_string()))
                .limit(1)
                .load::<crate::models::Quiz>(&*conn)
                .ok()
                .map(|mut sets| sets.pop().map(|set| set.id))
                .flatten()
        })
        .ok_or("Can't find quiz that was just inserted.")?;

    let questions = quiz.to_db_questions(previous_id);

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
