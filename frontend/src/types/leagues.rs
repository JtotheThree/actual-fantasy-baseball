use serde::{Deserialize, Serialize};
use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/leagues/my_leagues.graphql",
    response_derives = "Debug, Clone"
)]
pub struct MyLeagues;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/leagues/select_league.graphql",
    response_derives = "Debug, Clone"
)]
pub struct SelectLeague;



#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct League {
    pub id: String,
    pub name: String,
    pub team: Option<Team>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    pub id: String,
    pub name: String,
}
