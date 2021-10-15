use serde::{Deserialize, Serialize};

use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/auth/login.graphql",
    variable_derives = "Debug, Clone",
    response_derives = "Debug, Clone"
)]
pub struct Login;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/auth/logout.graphql",
    response_derives = "Debug, Clone"
)]
pub struct Logout;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/auth/signup.graphql",
    response_derives = "Debug, Clone"
)]
pub struct Signup;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/auth/me.graphql",
    response_derives = "Debug, Clone"
)]
pub struct Me;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub username: String,
    pub email: String,
    pub role: String,
}
