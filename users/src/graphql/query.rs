use crate::auth::*;
use crate::config::CONFIG;
use crate::models::*;
use common::*;
use async_graphql::*;
use jsonwebtoken::TokenData;
use wither::prelude::*;
use wither::{bson::doc, bson::oid::ObjectId, mongodb::Database};

pub struct Query;

#[Object(extends)]
impl Query {
    /// Get current user info
    async fn me(&self, ctx: &Context<'_>) -> Result<MeResponse> {
        let redis_client: &redis::Client = ctx.data()?;

        let mut con = redis_client.get_connection()?;
        let token_data = ctx.data_opt::<TokenData<Claims>>().unwrap();
    
        let maybe_current_user = get_current_user(&mut con, token_data);

        if let Some(current_user) = maybe_current_user {
            let db: &Database = ctx.data()?;
            let oid = ObjectId::with_string(&current_user.id).expect("Can't convert user id string to ObjectId");
            let user = User::find_by_id(db, &oid).await.unwrap();
            Ok(MeResponse{
                username: user.username.clone(),
                email: user.email.clone(),
                role: user.role.clone(),
            })
        } else {
            Err("Not logged in".into())
        }
    }

    /// Get a user by its ID
    #[graphql(entity)]
    async fn find_user_info_by_id(&self, ctx: &Context<'_>, id: ID) -> Result<UserInfo> {
        let oid_result = ObjectId::with_string(&id.to_string());
        if let Ok(oid) = oid_result {
            let db: &Database = ctx.data()?;
            let maybe_user = User::find_by_id(db, &oid).await;
            if let Some(user) = maybe_user {
                Ok(user.to_user_info())
            } else {
                Err("No user found".into())
            }
        } else {
            Err("Invalid ID".into())
        }
    }
}

pub struct Mutation;

#[Object]
impl Mutation {
    async fn signup(&self, ctx: &Context<'_>, new_user: SignupInput) -> Result<SignupResponse, Error> {
        let db: &Database = ctx.data()?;

        let password = hash::hash_password(&new_user.password);

        let mut user = User::new_user(&new_user.username, &new_user.email, &password);

        if let Ok(_) = user.save(&db, None).await {
            Ok(SignupResponse{
                status: "success".to_string()
            })
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
                    username: user.username.clone(),
                    email: user.email.clone(),
                    role: user.role.clone(),
                    token: token,
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
}