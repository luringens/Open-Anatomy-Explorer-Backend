use crate::label::*;
use actix::prelude::*;
use uuid::Uuid;
// use postgres::Client;

pub struct DbExecutor();

impl DbExecutor {
    pub fn new() -> DbExecutor {
        DbExecutor()
    }
}

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

pub struct CreateLabelPoint {
    pub data: LabelPoint,
}

impl Message for CreateLabelPoint {
    type Result = Result<uuid::Uuid, Box<dyn std::error::Error>>;
}

impl Handler<CreateLabelPoint> for DbExecutor {
    type Result = Result<uuid::Uuid, Box<dyn std::error::Error>>;

    fn handle(&mut self, msg: CreateLabelPoint, _: &mut Self::Context) -> Self::Result {
        let id = Uuid::new_v4();
        let data = serde_json::to_string(&msg.data)?;
        std::fs::write(format!("./json/{}", id), data)?;
        Ok(id)
    }
}

pub struct LoadLabelPoint<'a> {
    pub id: &'a str,
}

impl<'a> Message for LoadLabelPoint<'a> {
    type Result = Result<LabelPoint, Box<dyn std::error::Error>>;
}

impl<'a> Handler<LoadLabelPoint<'a>> for DbExecutor {
    type Result = Result<LabelPoint, Box<dyn std::error::Error>>;

    fn handle(&mut self, msg: LoadLabelPoint<'a>, _: &mut Self::Context) -> Self::Result {
        let data = std::fs::read(format!("./json/{}", &msg.id))?;
        let string = std::str::from_utf8(&data)?;
        let result = serde_json::from_str(string)?;
        Ok(result)
    }
}
