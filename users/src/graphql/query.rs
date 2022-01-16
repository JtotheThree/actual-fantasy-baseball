use std::collections::HashMap;

use crate::auth::*;
use crate::config::CONFIG;
use crate::models::*;
use common::*;
use async_graphql::*;
use jsonwebtoken::TokenData;
use wither::prelude::*;
use wither::{bson::doc, mongodb::Database};

use common::filter::process_filter;

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;


#[Object]
impl User {
    async fn id(&self) -> ID {
        if let Some(id) = &self.id {
            ID::from(id)
        } else {
            ID::from("")
        }
    }
    async fn username(&self) -> &str {
        &self.username
    }
    async fn email(&self) -> &str {
        &self.email
    }
    async fn role(&self) -> &str {
        &self.role
    }
    async fn selected_league(&self) -> Option<League> {
        if let Some(selected_league) = &self.selected_league {
            Some(League{
                id: ID::from(selected_league),
            })
        } else {
            None
        }
    }
}

pub struct League {
    pub id: ID,
}

pub struct Team {
    pub id: ID,
}


pub struct Query;

#[Object(extends)]
impl Query {
    /// Get current user info
    async fn me(&self, ctx: &Context<'_>) -> Result<User> {
        let redis_client: &redis::Client = ctx.data()?;

        let mut con = redis_client.get_connection()?;
        let token_data = ctx.data_opt::<TokenData<Claims>>().unwrap();

        let maybe_current_user = get_current_user(&mut con, token_data);

        if let Some(current_user) = maybe_current_user {
            let db: &Database = ctx.data()?;
            let id: ID = ID::from(current_user.id);
            let user = User::find_by_id(db, id).await.unwrap();

            Ok(user)
        } else {
            Err("Not logged in".into())
        }
    }

    async fn users(&self, ctx: &Context<'_>, filter: Option<HashMap<String, serde_json::Value>>) -> Result<Vec<User>> {
        let db: &Database = ctx.data().expect("Cannot connect to database");

        match filter {
            Some(filter) => {
                let filter = process_filter(filter)?;
                let users = User::find_all(db, Some(filter)).await?;

                Ok(users)
            },
            None => {
                let users = User::find_all(db, None).await?;

                Ok(users)
            }
        }
    }

    async fn user(&self, ctx: &Context<'_>, id: ID) -> Result<User> {
        let db: &Database = ctx.data().expect("Cannot connect to database");

        let maybe_user = User::find_by_id(db, id).await;

        if let Some(user) = maybe_user {
            Ok(user)
        } else {
            Err("Can't get league by id".into())
        }
    }

    /// Get a user by its ID
    #[graphql(entity)]
    async fn find_user_by_id(&self, ctx: &Context<'_>, id: ID) -> Result<User> {
        let db: &Database = ctx.data()?;
        let maybe_user = User::find_by_id(db, id).await;
        if let Some(user) = maybe_user {
            Ok(user)
        } else {
            Err("No user found".into())
        }
    }

    #[graphql(entity)]
    async fn find_league_by_id(&self, id: ID) -> League {
        League { id }
    }

    #[graphql(entity)]
    async fn find_team_by_id(&self, id: ID) -> Team {
        Team { id }
    }
}

#[Object(extends)]
impl League {
    #[graphql(external)]
    async fn id(&self) -> &ID {
        &self.id
    }

    async fn user(&self, ctx: &Context<'_>) -> Result<User> {
        let db: &Database = ctx.data().expect("Cannot connect to database");

        let maybe_user = User::find_by_id(db, self.id.clone()).await;

        if let Some(user) = maybe_user {
            Ok(user)
        } else {
            Err("Can't get user".into())
        }
    }
}

#[Object(extends)]
impl Team {
    #[graphql(external)]
    async fn id(&self) -> &ID {
        &self.id
    }
}

pub struct Mutation;

#[Object]
impl Mutation {
    async fn signup(&self, ctx: &Context<'_>, new_user: SignupInput) -> Result<User, Error> {
        let db: &Database = ctx.data()?;

        let password = hash::hash_password(&new_user.password);

        let mut user = User::new_user(&new_user.username, &new_user.email, &password);

        if let Ok(_) = user.save(&db, None).await {
            Ok(user)
        } else {
            Err(Error::new("Can't signup user"))
        }
    }

    async fn login(&self, ctx: &Context<'_>, credentials: LoginInput) -> Result<LoginResponse> {
        let db: &Database = ctx.data()?;

        if let Some(user) = User::find_by_username(db, &credentials.username_or_email).await {
            let clear_password = &credentials.password;
            let hashed_password = &user.password;

            let password_verified = hash::verify_hash(hashed_password, clear_password);

            if password_verified {
                let redis_client: &redis::Client = ctx.data()?;
                let mut con = redis_client.get_connection()?;

                let user_id = user.id.unwrap().to_string();

                let session = generate_session(&mut con, &user_id);
                let token = generate_token(&user_id, &user.username, &user.role, &session, &CONFIG.session.secret);

                Ok(LoginResponse{
                    id: ID::from(user_id),
                    username: user.username.clone(),
                    email: user.email.clone(),
                    role: user.role.clone(),
                    token,
                })
            } else {
                Err(Error::new("Incorrect password"))
            }
        } else {
            Err(Error::new("User not found"))
        }
    }

    async fn logout(&self, ctx: &Context<'_>) -> Result<LogoutResponse> {
        let redis_client: &redis::Client = ctx.data()?;

        let mut con = redis_client.get_connection()?;
        let token_data = ctx.data_opt::<TokenData<Claims>>().unwrap();

        remove_session(&mut con, &token_data.claims.id);

        Ok(LogoutResponse{
            status: "Success".to_string(),
        })
    }

    // State
    async fn select_league(&self, ctx: &Context<'_>, id: ID) -> Result<League> {
        let redis_client: &redis::Client = ctx.data()?;

        let mut con = redis_client.get_connection()?;
        let token_data = ctx.data_opt::<TokenData<Claims>>().unwrap();

        let maybe_current_user = get_current_user(&mut con, token_data);

        if let Some(current_user) = maybe_current_user {
            let db: &Database = ctx.data()?;
            Ok(User::select_league(db, ID::from(current_user.id), id).await.expect("Can't select league"))
        } else {
            Err("Not logged in".into())
        }
    }
}


#[derive(InputObject)]
pub struct SignupInput {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(InputObject)]
pub struct LoginInput {
    pub username_or_email: String,
    pub password: String,
}

#[derive(SimpleObject)]
pub struct LoginResponse {
    pub id: ID,
    pub username: String,
    pub email: String,
    pub role: String,
    pub token: String,
}

#[derive(SimpleObject)]
pub struct LogoutResponse {
    pub status: String,
}
