use graphql_client::GraphQLQuery;
use serde::{Deserialize, Serialize};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/auth/login.graphql",
    response_derives = "Debug, Clone",
    variables_derives = "Deserialize,Debug"
)]
pub struct Login;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/auth/logout.graphql",
    response_derives = "Debug, Clone",
    variables_derives = "Deserialize,Debug"
)]
pub struct Logout;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/auth/signup.graphql",
    response_derives = "Debug, Clone",
    variables_derives = "Deserialize,Debug"
)]
pub struct Signup;

#[derive(GraphQLQuery, PartialEq)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/auth/me.graphql",
    response_derives = "Debug, Clone",
    variables_derives = "Deserialize,Debug"
)]
pub struct Me;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub role: String,
}
