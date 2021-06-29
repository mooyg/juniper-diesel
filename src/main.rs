#![feature(decl_macro, proc_macro_hygiene)]
#[macro_use]
extern crate rocket;
extern crate api;
extern crate diesel;
extern crate dotenv;
extern crate r2d2;
mod database;
use juniper::{EmptyMutation, EmptySubscription};
mod graphql;
fn main() {
    rocket::ignite()
        .manage(graphql::context::Context {
            database: database::database_conn::establish_connection(),
        })
        .manage(graphql::graphql::Schema::new(
            graphql::queries::queries::Query,
            EmptyMutation::new(),
            EmptySubscription::new(),
        ))
        .mount(
            "/",
            rocket::routes![
                graphql::graphql::graphiql,
                graphql::graphql::get_graphql_handler,
                graphql::graphql::post_graphql_handler
            ],
        )
        .launch();
}
