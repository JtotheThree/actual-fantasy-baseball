use async_graphql::*;
use futures::stream::StreamExt;
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

    // query
    pub async fn find_all(db: &Database) -> Result<Vec::<Self>> {
        let mut users: Vec<User> = Vec::new();
        let mut cursor = User::find(&db, None, None).await?;

        while let Some(league) = cursor.next().await {
            users.push(league.unwrap());
        }

        Ok(users)
    }

    pub async fn find_by_id(db: &Database, id: ID) -> Option<Self> {
        let oid = ObjectId::with_string(&id).expect("Can't get id from String");
        User::find_one(&db, doc! { "_id": oid }, None).await.unwrap()
    }

    pub async fn find_by_username(db: &Database, username: &str) -> Option<Self> {
        User::find_one(&db, doc! { "username": username }, None)
            .await
            .unwrap()
    }
}

#[Object]
impl User {
    async fn id(&self) -> ID {
        if let Some(id) = &self.id {
            ID::from(id)
        } else {
            let crap = String::from("");
            ID::from(crap)
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
    pub username: String,
    pub email: String,
    pub role: String,
    pub token: String,
}

#[derive(SimpleObject)]
pub struct LogoutResponse {
    pub status: String,
}
