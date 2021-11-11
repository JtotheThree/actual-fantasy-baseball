use serde::{Deserialize, Serialize};
use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/teams/my_teams.graphql",
    response_derives = "Debug, Clone"
)]
pub struct MyTeams;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/teams/create_team.graphql",
    response_derives = "Debug, Clone"
)]
pub struct CreateTeam;


#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    pub id: String,
    pub name: String,
    pub league_id: String,
}
