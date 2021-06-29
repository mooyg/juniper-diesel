use crate::diesel::RunQueryDsl;
use crate::graphql::context::Context;
use api::models::User;
pub struct Query;
use juniper::graphql_object;
#[graphql_object(
    context = Context,
)]
impl Query {
    fn apiVersion(context: &Context) -> &'static str {
        use api::schema::users::dsl::*;
        let connection = context.database.get().unwrap();
        let result = users
            .load::<User>(&connection)
            .expect("Error loading Users");
        println!("{:?}", result);
        "2.0"
    }
}
