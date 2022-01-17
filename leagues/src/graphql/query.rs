use crate::models::*;
use common::*;
use async_graphql::*;
use jsonwebtoken::TokenData;
use strum::IntoEnumIterator;
use wither::prelude::*;
use wither::{mongodb::Database};

use std::collections::HashMap;

use common::filter::process_filter;

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

#[Object]
impl League {
    async fn id(&self) -> ID {
        if let Some(id) = &self.id {
            ID::from(id)
        } else {
            ID::from("")
        }
    }

    async fn name(&self) -> &str {
        &self.name
    }

    async fn description(&self) -> &str {
        &self.description
    }

    async fn public(&self) -> bool {
        self.public
    }

    async fn max_players(&self) -> i64 {
        self.max_players
    }

    async fn state(&self) -> LeagueState {
        self.state
    }

    async fn manual_state(&self) -> bool {
        self.manual_state
    }

    async fn owner(&self) -> User {
        User{ id: ID::from(&self.owner) }
    }

    async fn managers(&self) -> Vec<User> {
        self.managers.iter().map(|id| User{
            id: ID::from(id)
        })
        .collect()
    }

    async fn managers_count(&self) -> usize {
        self.managers.len()
    }
}

pub struct User {
    pub id: ID,
}

pub struct Team {
    pub id: ID,
}

pub struct Query;

#[Object(extends, cache_control(max_age = 60))]
impl Query {
    async fn leagues(&self, ctx: &Context<'_>, filter: Option<HashMap<String, serde_json::Value>>) -> Result<Vec<League>> {
        let db: &Database = ctx.data().expect("Cannot connect to database");

        match filter {
            Some(filter) => {
                let filter = process_filter(filter)?;
                let leagues = League::find_all(db, Some(filter)).await?;

                Ok(leagues)
            },
            None => {
                let leagues = League::find_all(db, None).await.expect("Cannot get leagues");

                Ok(leagues)
            }
        }
    }

    async fn league(&self, ctx: &Context<'_>, id: ID) -> Result<League> {
        let db: &Database = ctx.data().expect("Cannot connect to database");

        let maybe_league = League::find_by_id(db, &id).await;

        if let Some(league) = maybe_league {
            Ok(league)
        } else {
            Err("Can't get league by id".into())
        }
    }

    async fn meta_league_state(&self) -> HashMap<String, String> {
        let mut values = HashMap::<String, String>::new();

        for value in LeagueState::iter() {
            match value {
                LeagueState::Manual => {values.insert("MANUAL".to_string(), "Manual".to_string());},
                LeagueState::Created => {values.insert("CREATED".to_string(), "Created".to_string());},
                LeagueState::Drafting => {values.insert("DRAFTING".to_string(), "Drafting".to_string());},
                LeagueState::SeasonStart => {values.insert("SEASON_START".to_string(), "Season Start".to_string());},
                LeagueState::Playoffs => {values.insert("PLAYOFFS".to_string(), "Playoffs".to_string());},
                LeagueState::RealmSeries => {values.insert("REALM_SERIES".to_string(), "Realm Series".to_string());},
                LeagueState::SeasonEnd => {values.insert("SEASON_END".to_string(), "Season End".to_string());},
            }
        }

        values
    }


    #[graphql(entity)]
    async fn find_user_by_id(&self, id: ID) -> User {
        User { id }
    }

    #[graphql(entity)]
    async fn find_league_by_id(&self, ctx: &Context<'_>, id: ID) -> Result<League> {
        let db: &Database = ctx.data()?;
        let maybe_league = League::find_by_id(db, &id).await;
        if let Some(league) = maybe_league {
            Ok(league)
        } else {
            Err("No user found".into())
        }
    }
}

#[Object(extends, cache_control(max_age = 60))]
impl User {
    #[graphql(external)]
    async fn id(&self) -> &ID {
        &self.id
    }

    async fn joined_leagues(&self, ctx: &Context<'_>) -> Result<Vec<League>> {
        let db: &Database = ctx.data().expect("Cannot connect to database");

        let maybe_leagues = League::find_by_user_id(db, &self.id).await;

        if let Ok(leagues) = maybe_leagues {
            Ok(leagues)
        } else {
            Err("Can't get leages for user".into())
        }
    }

    async fn owned_leagues(&self, ctx: &Context<'_>) -> Result<Vec<League>> {
        let db: &Database = ctx.data().expect("Cannot connect to database");

        let maybe_leagues = League::find_by_owner_id(db, &self.id).await;

        if let Ok(leagues) = maybe_leagues {
            Ok(leagues)
        } else {
            Err("Can't get leagues for owner".into())
        }
    }
}

#[Object(extends, cache_control(max_age = 60))]
impl Team {
    #[graphql(external)]
    async fn id(&self) -> &ID {
        &self.id
    }
}


pub struct Mutation;

#[Object(extends, cache_control(max_age = 60))]
impl Mutation {
    async fn create_league(&self, ctx: &Context<'_>, input: CreateLeagueInput) -> Result<League, Error> {
        let db: &Database = ctx.data()?;

        let redis_client: &redis::Client = ctx.data()?;

        let mut con = redis_client.get_connection()?;
        let token_data = ctx.data_opt::<TokenData<Claims>>().unwrap();

        let maybe_current_user = get_current_user(&mut con, token_data);

        if let Some(current_user) = maybe_current_user {
            let mut new_league = League::new_league(
                &input.name,
                &input.description,
                input.public,
                input.password,
                input.max_players,
                input.manual_state,
                &current_user.id,
            );

            new_league.save(&db, None).await?;

            Ok(new_league)

            /*if let Ok(_) = new_league.save(&db, None).await {
                Ok(new_league)
            } else {
                Err(Error::new("Can't create league user"))
            }*/
        } else {
            Err("Can't create league".into())
        }
    }

    async fn join_league(&self, ctx: &Context<'_>, id: ID) -> Result<League, Error> {
        let db: &Database = ctx.data()?;

        let redis_client: &redis::Client = ctx.data()?;

        let mut con = redis_client.get_connection()?;
        let token_data = ctx.data_opt::<TokenData<Claims>>().unwrap();

        let maybe_current_user = get_current_user(&mut con, token_data);

        if let Some(current_user) = maybe_current_user {
            let league = League::add_manager(&db, id.to_string(), current_user.id.clone()).await?;

            Ok(league)
        } else {
            Err("Unable to join league".into())
        }
    }

    async fn add_manager_to_league(
        &self, ctx: &Context<'_>,
        league_id: String,
        user_id: String,
    ) -> Result<League, Error> {
        let db: &Database = ctx.data()?;

        if let Ok(league) = League::add_manager(db, league_id, user_id).await {
            Ok(league)
        } else {
            Err("Cannot insert user into league".into())
        }
    }
}



#[derive(Clone, InputObject)]
pub struct CreateLeagueInput {
    pub name: String,
    pub description: String,
    pub public: bool,
    pub password: Option<String>,
    pub max_players: i64,
    pub manual_state: bool,
}
