use crate::label::*;
use actix::prelude::*;
use postgres::Client;

pub struct DbExecutor(Client);

impl DbExecutor {
    pub fn new() -> DbExecutor {
        let client = Client::connect("host=localhost user=postgres", postgres::NoTls)
            .expect("Failed to connect to Postgres");
        DbExecutor(client)
    }
}

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

struct CreateLabelPoint {
    data: LabelPoint,
}

impl Message for CreateLabelPoint {
    type Result = Result<(), ()>;
}

impl Handler<CreateLabelPoint> for DbExecutor {
    type Result = Result<(), ()>;

    fn handle(&mut self, msg: CreateLabelPoint, _: &mut Self::Context) -> Self::Result {
        let res = self
            .0
            .batch_execute("INSERT INTO LabelPoint (id, position, color) VALUES ($1, $2, $3)");

        Ok(())
    }
}
