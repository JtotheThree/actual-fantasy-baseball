use graphql_client::{GraphQLQuery, QueryBody};
use yew::prelude::*;

use crate::agents::{State, state::Request};
use crate::components::card::Card;
use crate::routes::AppRoute;
use crate::services::GraphQL;
use crate::types::*;

pub struct LeagueStatus {
    gql: GraphQL,
    link: ComponentLink<Self>,
    task: Option<FetchTask>,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub league_id,
}

