use actix::prelude::*;
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};

pub struct DatabaseExecutor(pub Pool<ConnectionManager<PgConnection>>);

impl Actor for DatabaseExecutor {
    type Context = SyncContext<Self>;
}
