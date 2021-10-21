use crate::graphql::League;

use async_graphql::*;
use futures::stream::TryStreamExt;
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

    // move to details, "state"?
    pub selected_league: Option<ObjectId>,
}

impl User {
    pub fn new_user(username: &str, email: &str, password: &str) -> Self {
        User {
            id: None,
            username: String::from(username),
            email: String::from(email),
            password: String::from(password),
            role: String::from("user"),
            selected_league: None,
        }
    }

    // query
    pub async fn find_all(db: &Database) -> Result<Vec::<Self>> {
        let cursor = User::find(&db, None, None).await?;
        let users: Vec<User> = cursor.try_collect().await?;

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

    pub async fn select_league(db: &Database, user_id: ID, league_id: ID) -> Result<League> {
       let query = doc! {
            "_id": ObjectId::with_string(&user_id).expect("Can't get id from String"),
        };

        if let Some(mut user) = User::find_one(db, Some(query), None).await? {
            user.selected_league = Some(ObjectId::with_string(&league_id).expect("Can't get id from String"));
            user.save(db, None).await?;

            Ok(League{
                id: league_id
            })
        } else {
            Err(format!("User with id: {:?} not found", &user_id).into())
        }
    }
}

