use serde::{Deserialize, Serialize};
use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/leagues/my_leagues.graphql",
    response_derives = "Debug, Clone"
)]
pub struct MyLeagues;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct League {
    pub id: String,
    pub name: String,
}
