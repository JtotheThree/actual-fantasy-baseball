use std::collections::HashMap;

use crate::models::*;

use common::*;
use async_graphql::*;
use jsonwebtoken::TokenData;
use wither::prelude::*;
use wither::{mongodb::Database};

use common::filter::process_filter;

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

/// Object Implementations
#[Object]
impl Team {
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

    async fn owner(&self) -> User {
        User{ id: ID::from(&self.owner) }
    }

    async fn league(&self) -> League {
        League{ id: ID::from(&self.league) }
    }

    async fn gold(&self) -> i64 {
        self.gold
    }

    async fn roster(&self) -> &Roster {
        &self.roster
    }

    async fn lineup(&self) -> &Vec<Option<LineupSlot>> {
        &self.lineup
    }
}

// USER
pub struct User {
    pub id: ID,
}

// LEAGUE
pub struct League {
    pub id: ID,
}


#[Object(extends, cache_control(max_age = 60))]
impl User {
    #[graphql(external)]
    async fn id(&self) -> &ID {
        &self.id
    }

    async fn teams(&self, ctx: &Context<'_>) -> Result<Vec<Team>> {
        let db: &Database = ctx.data().expect("Cannot connect to database");

        let maybe_teams = Team::find_by_owner_id(db, &self.id).await;

        if let Ok(teams) = maybe_teams {
            Ok(teams)
        } else {
            Err("Can't get leagues for owner".into())
        }
    }
}

#[Object(extends, cache_control(max_age = 60))]
impl League {
    #[graphql(external)]
    async fn id(&self) -> &ID {
        &self.id
    }

    async fn team(&self, ctx: &Context<'_>) -> Option<Team> {
        let db: &Database = ctx.data().expect("Cannot connect to database");
        let redis_client: &redis::Client = ctx.data().expect("Cannot connect to cache");

        let mut con = redis_client.get_connection().expect("Cannot connect to cache");
        let token_data = ctx.data_opt::<TokenData<Claims>>().unwrap();

        let maybe_current_user = get_current_user(&mut con, token_data);

        if let Some(current_user) = maybe_current_user {
            Team::find_user_team_for_league(db, &current_user.id, &self.id).await
        } else {
            None
        }
    }
}

/// Query
pub struct Query;

#[Object(extends, cache_control(max_age = 60))]
impl Query {
    async fn teams(&self, ctx: &Context<'_>, filter: Option<HashMap<String, serde_json::Value>>) -> Result<Vec<Team>> {
        let db: &Database = ctx.data().expect("Cannot connect to database");

        match filter {
            Some(filter) => {
                let filter = process_filter(filter)?;
                let teams =  Team::find_all(db, Some(filter)).await?;

                Ok(teams)
            },
            None => {
                let teams =  Team::find_all(db, None).await?;

                Ok(teams)
            }
        }

    }

    async fn team(&self, ctx: &Context<'_>, id: ID) -> Result<Team> {
        let db: &Database = ctx.data().expect("Cannot connect to database");

        let maybe_team = Team::find_by_id(db, &id).await;

        if let Some(team) = maybe_team {
            Ok(team)
        } else {
            Err("Can't get team by id".into())
        }
    }

    #[graphql(entity)]
    async fn find_user_by_id(&self, id: ID) -> User {
        User { id }
    }

    #[graphql(entity)]
    async fn find_league_by_id(&self, id: ID) -> League {
        League { id }
    }

    #[graphql(entity)]
    async fn find_team_by_id(&self, ctx: &Context<'_>, id: ID) -> Result<Team> {
        let db: &Database = ctx.data()?;
        let maybe_team = Team::find_by_id(db, &id).await;
        if let Some(team) = maybe_team {
            Ok(team)
        } else {
            Err("No user found".into())
        }
    }
}
/// Mutation
pub struct Mutation;

#[Object(extends, cache_control(max_age = 60))]
impl Mutation {
    async fn create_team(&self, ctx: &Context<'_>, input: CreateTeamInput) -> Result<Team, Error> {
        let db: &Database = ctx.data()?;
        let redis_client: &redis::Client = ctx.data()?;

        let mut con = redis_client.get_connection()?;
        let token_data = ctx.data_opt::<TokenData<Claims>>().unwrap();

        let maybe_current_user = get_current_user(&mut con, token_data);

        if let Some(current_user) = maybe_current_user {
            let mut new_team = Team::new_team(&input.name, &input.league_id, &current_user.id);

            if let Ok(_) = new_team.save(&db, None).await {
                Ok(new_team)
            } else {
                Err(Error::new("Can't create team, bad user"))
            }
        } else {
            Err("Can't create team".into())
        }
    }
}


// Inputs

#[derive(Clone, InputObject)]
pub struct CreateTeamInput {
    pub name: String,
    pub league_id: ID,
}
