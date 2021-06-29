use super::queries::queries::Query;
use crate::graphql::context::Context;
use juniper::{EmptyMutation, EmptySubscription, RootNode};
use rocket::{response::content, State};

pub type Schema = RootNode<'static, Query, EmptyMutation<Context>, EmptySubscription<Context>>;

#[get("/")]
pub fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql", None)
}

#[get("/graphql?<request>")]
pub fn get_graphql_handler(
    context: State<Context>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute_sync(&*schema, &context)
}

#[post("/graphql", data = "<request>")]
pub fn post_graphql_handler(
    context: State<Context>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute_sync(&*schema, &context)
}
