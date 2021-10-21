use crate::models::*;
use common::*;
use async_graphql::*;
use jsonwebtoken::TokenData;
use wither::prelude::*;
use wither::{mongodb::Database};

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

    async fn owner(&self) -> User {
        User{ id: ID::from(&self.owner) }
    }

    async fn users(&self) -> Vec<User> {
        self.users.iter().map(|id| User{
            id: ID::from(id)
        })
        .collect()
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
    async fn leagues(&self, ctx: &Context<'_>) -> Vec<League> {
        let db: &Database = ctx.data().expect("Cannot connect to database");

        let leagues = League::find_all(db).await.expect("Cannot get leagues");

        leagues
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

    async fn add_user_to_league(
        &self, ctx: &Context<'_>,
        input: AddUserInput
    ) -> Result<League, Error> {
        let db: &Database = ctx.data()?;

        if let Ok(league) = League::add_user(db, input.id, input.user_id).await {
            Ok(league)
        } else {
            Err("Cannot insert user into league".into())
        }
    }
}



#[derive(Clone, InputObject)]
pub struct CreateLeagueInput {
    pub name: String,
}

#[derive(Clone, InputObject)]
pub struct AddUserInput {
    pub id: String,
    pub user_id: String, 
}
