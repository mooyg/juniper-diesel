use crate::database::database_conn;

pub struct Context {
    pub database: database_conn::DbPool,
}
impl juniper::Context for Context {}
