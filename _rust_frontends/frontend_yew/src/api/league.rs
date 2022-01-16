use serde::{Deserialize, Serialize};
use graphql_client::GraphQLQuery;
use std::fmt;

use crate::api::{Manager};

/*
MyLeagues
*/
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/leagues/my_leagues.graphql",
    response_derives = "Debug, Clone, PartialEq",
    variables_derives = "Deserialize,Debug"
)]
pub struct MyLeagues;

/*
GetLeagues
*/
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/leagues/get_league.graphql",
    response_derives = "Debug, Clone, PartialEq",
    variables_derives = "Deserialize,Debug"
)]
pub struct GetLeague;

/*
CreateLeague
*/
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/leagues/create_league.graphql",
    response_derives = "Debug, Clone",
    variables_derives = "Deserialize,Debug"
)]
pub struct CreateLeague;

/*
PublicLeagues
*/
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/leagues/public_leagues.graphql",
    response_derives = "Debug, Clone",
    variables_derives = "Deserialize,Debug"
)]
pub struct PublicLeagues;

/*
JoinLeague
*/
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/leagues/join_league.graphql",
    response_derives = "Debug, Clone",
    variables_derives = "Deserialize,Debug"
)]
pub struct JoinLeague;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LeagueState {
    Manual,
    Created,
    Drafting,
    SeasonStart,
    Playoffs,
    RealmSeries,
    SeasonEnd,
    ErrorLoading,
}

impl PartialEq for LeagueState {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

impl fmt::Display for LeagueState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LeagueState::Manual => write!{f, "Manual"},
            LeagueState::Created => write!{f, "Created"},
            LeagueState::Drafting => write!{f, "Drafting"},
            LeagueState::SeasonStart => write!{f, "Season Started"},
            LeagueState::Playoffs => write!{f, "Playoffs"},
            LeagueState::RealmSeries => write!{f, "Realm Series"},
            LeagueState::SeasonEnd => write!{f, "Season Ended"},
            _ => write!{f, ""}
        }
    }
}

impl From<public_leagues::LeagueState> for LeagueState
{
    fn from(state: public_leagues::LeagueState) -> Self {
        match state {
            public_leagues::LeagueState::MANUAL => LeagueState::Manual,
            public_leagues::LeagueState::CREATED => LeagueState::Created,
            public_leagues::LeagueState::DRAFTING => LeagueState::Drafting,
            public_leagues::LeagueState::SEASON_START => LeagueState::SeasonStart,
            public_leagues::LeagueState::PLAYOFFS => LeagueState::Playoffs,
            public_leagues::LeagueState::REALM_SERIES => LeagueState::RealmSeries,
            public_leagues::LeagueState::SEASON_END => LeagueState::SeasonEnd,
            _ => LeagueState::ErrorLoading,
        }
    }
}

impl From<get_league::LeagueState> for LeagueState
{
    fn from(state: get_league::LeagueState) -> Self {
        match state {
            get_league::LeagueState::MANUAL => LeagueState::Manual,
            get_league::LeagueState::CREATED => LeagueState::Created,
            get_league::LeagueState::DRAFTING => LeagueState::Drafting,
            get_league::LeagueState::SEASON_START => LeagueState::SeasonStart,
            get_league::LeagueState::PLAYOFFS => LeagueState::Playoffs,
            get_league::LeagueState::REALM_SERIES => LeagueState::RealmSeries,
            get_league::LeagueState::SEASON_END => LeagueState::SeasonEnd,
            _ => LeagueState::ErrorLoading,
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
    pub state: LeagueState,
    pub owner: Option<Manager>,
    pub managers: Option<Vec<Manager>>,
}

impl Default for League {
    fn default() -> League {
        League {
            id: "".to_string(),
            name: "".to_string(),
            description: "".to_string(),
            max_players: 0,
            state: LeagueState::Created,
            owner: None,
            managers: None,
        }
    }
}