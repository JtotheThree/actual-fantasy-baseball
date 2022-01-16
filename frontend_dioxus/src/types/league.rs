use serde::{Deserialize, Serialize};
use graphql_client::GraphQLQuery;
use std::fmt;

use crate::types::Manager;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/leagues/my_leagues.graphql",
    response_derives = "Debug, Clone",
    variables_derives = "Deserialize,Debug"
)]
pub struct MyLeagues;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/leagues/select_league.graphql",
    response_derives = "Debug, Clone",
    variables_derives = "Deserialize,Debug"
)]
pub struct SelectLeague;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/leagues/create_league.graphql",
    response_derives = "Debug, Clone",
    variables_derives = "Deserialize,Debug"
)]
pub struct CreateLeague;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/leagues/public_leagues.graphql",
    response_derives = "Debug, Clone",
    variables_derives = "Deserialize,Debug"
)]
pub struct PublicLeagues;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/leagues/join_league.graphql",
    response_derives = "Debug, Clone",
    variables_derives = "Deserialize,Debug"
)]
pub struct JoinLeague;

impl PartialEq for public_leagues::LeagueState {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

impl fmt::Display for public_leagues::LeagueState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            public_leagues::LeagueState::MANUAL => write!{f, "Manual"},
            public_leagues::LeagueState::CREATED => write!{f, "Created"},
            public_leagues::LeagueState::DRAFTING => write!{f, "Drafting"},
            public_leagues::LeagueState::SEASON_START => write!{f, "Season Started"},
            public_leagues::LeagueState::PLAYOFFS => write!{f, "Playoffs"},
            public_leagues::LeagueState::REALM_SERIES => write!{f, "Realm Series"},
            public_leagues::LeagueState::SEASON_END => write!{f, "Season Ended"},
            _ => write!{f, ""}
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct League {
    pub id: String,
    pub name: String,
    pub description: String,
    pub max_players: i64,
    pub state: Option<public_leagues::LeagueState>,
    pub owner: Option<Manager>,
    pub managers: Option<Vec<Manager>>,
    pub managers_count: i64,
}

impl Default for League {
    fn default() -> League {
        League {
            id: "".to_string(),
            name: "".to_string(),
            description: "".to_string(),
            max_players: 0,
            state: None,
            owner: None,
            managers: None,
            managers_count: 0,
        }
    }
}