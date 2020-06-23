use crate::quiz::*;
use actix::prelude::*;
use std::{env, error::Error};
use uuid::Uuid;

pub use messages::*;

pub struct QuizDbExecutor();

impl QuizDbExecutor {
    pub fn new() -> QuizDbExecutor {
        QuizDbExecutor()
    }
}

impl Actor for QuizDbExecutor {
    type Context = SyncContext<Self>;
}

impl Handler<CreateQuiz> for QuizDbExecutor {
    type Result = Result<uuid::Uuid, Box<dyn Error + Send + Sync>>;

    fn handle(&mut self, msg: CreateQuiz, _: &mut Self::Context) -> Self::Result {
        let data_dir = env::var("QUIZ_DATA_DIR").unwrap();
        let id = msg.uuid.unwrap_or_else(Uuid::new_v4);
        let data = serde_json::to_string(&msg.data)?;
        std::fs::write(format!("{}/{}.json", data_dir, id), data)?;
        Ok(id)
    }
}

impl Handler<LoadQuiz> for QuizDbExecutor {
    type Result = Result<Quiz, Box<dyn Error + Send + Sync>>;

    fn handle(&mut self, msg: LoadQuiz, _: &mut Self::Context) -> Self::Result {
        let data_dir = env::var("QUIZ_DATA_DIR").unwrap();
        let data = std::fs::read(format!("{}/{}.json", data_dir, &msg.id))?;
        let string = std::str::from_utf8(&data)?;
        let result = serde_json::from_str(string)?;
        Ok(result)
    }
}

impl Handler<DeleteQuiz> for QuizDbExecutor {
    type Result = Result<(), Box<dyn Error + Send + Sync>>;

    fn handle(&mut self, msg: DeleteQuiz, _: &mut Self::Context) -> Self::Result {
        let data_dir = env::var("QUIZ_DATA_DIR").unwrap();
        std::fs::remove_file(format!("{}/{}.json", data_dir, &msg.id))?;
        Ok(())
    }
}

mod messages {
    use crate::quiz::*;
    use actix::prelude::*;
    use std::error::Error;
    use uuid::Uuid;

    pub struct CreateQuiz {
        pub data: Quiz,
        pub uuid: Option<Uuid>,
    }

    impl Message for CreateQuiz {
        type Result = Result<uuid::Uuid, Box<dyn Error + Send + Sync>>;
    }

    pub struct LoadQuiz {
        pub id: Uuid,
    }

    impl Message for LoadQuiz {
        type Result = Result<Quiz, Box<dyn Error + Send + Sync>>;
    }

    pub struct DeleteQuiz {
        pub id: Uuid,
    }

    impl Message for DeleteQuiz {
        type Result = Result<(), Box<dyn Error + Send + Sync>>;
    }
}
