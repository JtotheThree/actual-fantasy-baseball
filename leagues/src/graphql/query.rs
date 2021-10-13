use crate::models::*;
use common::*;
use async_graphql::*;
use jsonwebtoken::TokenData;
use wither::prelude::*;
use wither::{mongodb::Database};

pub struct Query;

#[Object(extends, cache_control(max_age = 60))]
impl Query {
    async fn get_leagues(&self, ctx: &Context<'_>) -> Vec<League> {
        let db: &Database = ctx.data().expect("Cannot connect to database");

        let leagues = League::get_all(db).await.expect("Cannot get leagues");

        leagues
    }

    async fn get_league(&self, ctx: &Context<'_>, id: ID) -> Result<League> {
        let db: &Database = ctx.data().expect("Cannot connect to database");

        let maybe_league = League::get_by_id(db, id).await;

        if let Some(league) = maybe_league {
            Ok(league)
        } else {
            Err("Can't get league by id".into())
        }
    }

    #[graphql(entity)]
    async fn find_user_by_id(&self, id: ID) -> User {
        User { id }
    }
}

struct User {
    id: ID,
}

#[Object(extends)]
impl User {
    #[graphql(external)]
    async fn id(&self) -> &ID {
        &self.id
    }

    async fn leagues(&self, ctx: &Context<'_>) -> Result<Vec<League>> {
        let db: &Database = ctx.data().expect("Cannot connect to database");

        let maybe_leagues = League::get_by_owner_id(db, &self.id).await;

        if let Ok(leagues) = maybe_leagues {
            Ok(leagues)
        } else {
            Err("Can't get leagues for owner".into())
        }
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
            let mut new_league = League::new_league(&input.name, &current_user.id);

            if let Ok(_) = new_league.save(&db, None).await {
                Ok(new_league)
            } else {
                Err(Error::new("Can't create league user"))
            }
        } else {
            Err("Can't create league".into())
        }
    }
}