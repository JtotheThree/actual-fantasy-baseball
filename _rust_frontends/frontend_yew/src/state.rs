use std::rc::Rc;

use bounce::*;
use graphql_client::GraphQLQuery;

use crate::api::*;

#[derive(Debug)]
pub struct Reset;

// ME
#[future_notion(QueryMe)]
pub async fn query_me(_input: Rc<()>) -> Rc<me::ResponseData> {
    let body = Me::build_query(me::Variables{});
    // errors should be handled properly in actual application.
    let resp = post::<Me>(body).await.unwrap();

    resp.into()
}

#[derive(PartialEq, Atom)]
#[with_notion(Deferred<QueryMe>)]
#[with_notion(Reset)]
pub enum MeState {
    NotStarted,
    Pending,
    Complete(Rc<me::ResponseData>),
}

impl Default for MeState {
    fn default() -> MeState {
        Self::NotStarted
    }
}

impl WithNotion<Reset> for MeState {
    fn apply(self: Rc<Self>, _notion: Rc<Reset>) -> Rc<Self> {
        Self::NotStarted.into()
    }
}

impl WithNotion<Deferred<QueryMe>> for MeState {
    fn apply(self: Rc<Self>, notion: Rc<Deferred<QueryMe>>) -> Rc<Self> {
        match notion.output() {
            Some(m) => Self::Complete(m).into(),
            None => Self::Pending.into(),
        }
    }
}

// MY_LEAGUES

#[future_notion(QueryMyLeagues)]
pub async fn query_my_leagues(_input: Rc<()>) -> Rc<Vec<League>> {
    // errors should be handled properly in actual application.

    let body = MyLeagues::build_query(my_leagues::Variables{});

    let resp = post::<MyLeagues>(body).await.unwrap();

    let leagues: Vec<League> = resp.me.joined_leagues.into_iter()
        .map(|league| League {
            id: league.id,
            name: league.name,
            ..Default::default()
        })
        .collect();

    leagues.into()
}

#[derive(PartialEq, Atom)]
#[with_notion(Deferred<QueryMyLeagues>)]
#[with_notion(Reset)]
pub enum MyLeaguesState {
    NotStarted,
    Pending,
    Complete(Vec<League>),
}

impl Default for MyLeaguesState {
    fn default() -> MyLeaguesState {
        Self::NotStarted
    }
}

impl WithNotion<Reset> for MyLeaguesState {
    fn apply(self: Rc<Self>, _notion: Rc<Reset>) -> Rc<Self> {
        Self::NotStarted.into()
    }
}

impl WithNotion<Deferred<QueryMyLeagues>> for MyLeaguesState {
    fn apply(self: Rc<Self>, notion: Rc<Deferred<QueryMyLeagues>>) -> Rc<Self> {
        match notion.output() {
            Some(m) => Self::Complete(m.to_vec()).into(),
            None => Self::Pending.into(),
        }
    }
}