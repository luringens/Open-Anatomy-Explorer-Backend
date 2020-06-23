use crate::label::*;
use actix::prelude::*;
use std::{env, error::Error};
use uuid::Uuid;

pub use messages::*;

pub struct LabelDbExecutor();

impl LabelDbExecutor {
    pub fn new() -> LabelDbExecutor {
        LabelDbExecutor()
    }
}

impl Actor for LabelDbExecutor {
    type Context = SyncContext<Self>;
}

impl Handler<CreateLabelPoint> for LabelDbExecutor {
    type Result = Result<uuid::Uuid, Box<dyn Error + Send + Sync>>;

    fn handle(&mut self, msg: CreateLabelPoint, _: &mut Self::Context) -> Self::Result {
        let data_dir = env::var("LABEL_DATA_DIR").unwrap();
        let id = msg.uuid.unwrap_or_else(Uuid::new_v4);
        let data = serde_json::to_string(&msg.data)?;
        std::fs::write(format!("{}/{}.json", data_dir, id), data)?;
        Ok(id)
    }
}

impl Handler<LoadLabelPoint> for LabelDbExecutor {
    type Result = Result<Vec<LabelPoint>, Box<dyn Error + Send + Sync>>;

    fn handle(&mut self, msg: LoadLabelPoint, _: &mut Self::Context) -> Self::Result {
        let data_dir = env::var("LABEL_DATA_DIR").unwrap();
        let data = std::fs::read(format!("{}/{}.json", data_dir, &msg.id))?;
        let string = std::str::from_utf8(&data)?;
        let result = serde_json::from_str(string)?;
        Ok(result)
    }
}

impl Handler<DeleteLabelPoint> for LabelDbExecutor {
    type Result = Result<(), Box<dyn Error + Send + Sync>>;

    fn handle(&mut self, msg: DeleteLabelPoint, _: &mut Self::Context) -> Self::Result {
        let data_dir = env::var("LABEL_DATA_DIR").unwrap();
        std::fs::remove_file(format!("{}/{}.json", data_dir, &msg.id))?;
        Ok(())
    }
}

pub mod messages {
    use crate::label::*;
    use actix::prelude::*;
    use std::error::Error;
    use uuid::Uuid;

    pub struct CreateLabelPoint {
        pub data: Vec<LabelPoint>,
        pub uuid: Option<Uuid>,
    }

    impl Message for CreateLabelPoint {
        type Result = Result<uuid::Uuid, Box<dyn Error + Send + Sync>>;
    }

    pub struct LoadLabelPoint {
        pub id: Uuid,
    }

    impl Message for LoadLabelPoint {
        type Result = Result<Vec<LabelPoint>, Box<dyn Error + Send + Sync>>;
    }

    pub struct DeleteLabelPoint {
        pub id: Uuid,
    }

    impl Message for DeleteLabelPoint {
        type Result = Result<(), Box<dyn Error + Send + Sync>>;
    }
}
