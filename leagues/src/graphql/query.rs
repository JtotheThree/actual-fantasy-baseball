
use crate::config::CONFIG;
use crate::models::*;
use common::*;
use async_graphql::*;
use jsonwebtoken::TokenData;
use wither::prelude::*;
use wither::{bson::doc, bson::oid::ObjectId, mongodb::Database};

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

        let maybe_league = League::find_by_id(db, id).await;

        if let Some(league) = maybe_league {
            Ok(league)
        } else {
            Err("Can't get league by id".into())
        }
    }

    async fn get_league_by_owner(&self, ctx: &Context<'_>, owner_id: ID) -> Result<Vec<League>> {
        let db: &Database = ctx.data().expect("Cannot connect to database");

        let maybe_leagues = League::find_by_owner(db, owner_id).await;

        if let Ok(leagues) = maybe_leagues {
            Ok(leagues)
        } else {
            Err("Can't get leagues for owner".into())
        }
    }

    #[graphql(entity)]
    async fn find_league_by_id(&self, ctx: &Context<'_>, id: ID) -> Result<League> {
        self.get_league(ctx, id).await
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