use async_graphql::*;
use serde::{Deserialize, Serialize};
use wither::prelude::*;
use wither::{
    bson::{doc, oid::ObjectId},
    mongodb::Database,
};

/// User representation
#[derive(Debug, Model, Serialize, Deserialize)]
#[model(index(keys = r#"doc!{"username": 1}"#, options = r#"doc!{"unique": true}"#))]
pub struct User {
    /// The ID of the model.
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: String,
}

impl User {
    pub fn new_user(username: &str, email: &str, password: &str) -> Self {
        User {
            id: None,
            username: String::from(username),
            email: String::from(email),
            password: String::from(password),
            role: String::from("user"),
        }
    }

    pub fn to_user_info(&self) -> UserInfo {
        UserInfo {
            id: self.id.clone(),
            username: self.username.clone(),
            role: self.role.clone(),
            email: self.email.clone(),
        }
    }

    pub async fn find_by_id(db: &Database, id: &ObjectId) -> Option<Self> {
        User::find_one(&db, doc! { "_id": id }, None).await.unwrap()
    }

    pub async fn find_by_username(db: &Database, username: &str) -> Option<Self> {
        User::find_one(&db, doc! { "username": username }, None)
            .await
            .unwrap()
    }
}

/// Available User info
#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    /// The ID of the user.
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    /// The username.
    pub username: String,
    /// Role
    pub role: String,
    /// Email
    pub email: String,
}

#[Object]
impl UserInfo {
    async fn id(&self) -> String {
        if let Some(id) = &self.id {
            id.clone().to_string()
        } else {
            String::from("")
        }
    }

    async fn username(&self) -> &str {
        &self.username
    }
}

/// New User Input
#[derive(Debug, Serialize, Deserialize)]
pub struct UserInput {
    /// The new user username, must be unique.
    pub username: String,
    /// The new user password.
    pub password: String,
    // User email
    pub email: String,
}

#[derive(InputObject)]
pub struct SignupInput {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(SimpleObject)]
pub struct SignupResponse {
    pub status: String,
}

#[derive(InputObject)]
pub struct LoginInput {
    pub username_or_email: String,
    pub password: String,
}

#[derive(SimpleObject)]
pub struct LoginResponse {
    pub username: String,
    pub email: String,
    pub role: String,
    pub token: String,
}

#[derive(SimpleObject)]
pub struct LogoutResponse {
    pub status: String,
}

#[derive(SimpleObject)]
pub struct MeResponse {
    pub username: String,
    pub email: String,
    pub role: String,
}
