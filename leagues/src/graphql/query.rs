use crate::models::*;
use common::*;
use async_graphql::*;
use jsonwebtoken::TokenData;
use strum::IntoEnumIterator;
use wither::prelude::*;
use wither::mongodb::Database;

use std::collections::HashMap;

use common::filter::process_filter;
use common::meta::MetaSelect;

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

    async fn status(&self) -> LeagueStatus {
        self.status
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

    async fn meta_league_state(&self) -> MetaSelect {
        let mut select_values = MetaSelect::default();

        for value in LeagueState::iter() {
            select_values.values.push(format!{"{:?}", value});
            select_values.labels.push(format!{"{}", value});
        }

        select_values
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

    async fn update_league(&self, ctx: &Context<'_>, input: UpdateLeagueInput) -> Result<League, Error> {
        let db: &Database = ctx.data()?;

        let redis_client: &redis::Client = ctx.data()?;

        let mut con = redis_client.get_connection()?;
        let token_data = ctx.data_opt::<TokenData<Claims>>().unwrap();

        let maybe_current_user = get_current_user(&mut con, token_data);

        if let Some(current_user) = maybe_current_user {
            let maybe_league = League::find_by_id(&db, &input.id.clone()).await;

            if let Some(league) = maybe_league {
                if current_user.id != league.owner {
                    return Err("Can't update a league you don't own".into())
                }

                League::update(&db, &input.id.clone(), input).await?;

                Ok(league)
            } else {
                Err("League doesn't exist!".into())
            }
        } else {
            Err("Unable to update league".into())
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

    async fn set_league_state(&self, ctx: &Context<'_>, id: ID, state: LeagueState) -> Result<League, Error> {
        let db: &Database = ctx.data()?;

        if let Ok(league) = League::set_league_state(&db, id.to_string(), state).await {
            Ok(league)
        } else {
            Err("Cannot update league state".into())
        }
    }

    async fn set_league_status(&self, ctx: &Context<'_>, id: ID, status: LeagueStatus) -> Result<League, Error> {
        let db: &Database = ctx.data()?;

        if let Ok(league) = League::set_league_status(&db, id.to_string(), status).await {
            Ok(league)
        } else {
            Err("Cannot update league status".into())
        }
    }

    async fn add_manager_to_league(
        &self, ctx: &Context<'_>,
        league_id: ID,
        user_id: ID,
    ) -> Result<League, Error> {
        let db: &Database = ctx.data()?;

        if let Ok(league) = League::add_manager(db, league_id.to_string(), user_id.to_string()).await {
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

#[derive(Clone, InputObject)]
pub struct UpdateLeagueInput {
    pub id: ID,
    pub description: Option<String>,
    pub public: Option<bool>,
    pub password: Option<String>,
    pub max_players: Option<i64>,
    pub state: Option<LeagueState>,
    pub status: Option<LeagueStatus>,
}
