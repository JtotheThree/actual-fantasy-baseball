use crate::models::User;
use async_graphql::*;
use futures::stream::{StreamExt, TryStreamExt};
use serde::{Deserialize, Serialize};
use wither::prelude::*;
use wither::{bson::{doc, oid::ObjectId}, mongodb::Database};

/// League representation
#[derive(Clone, Debug, Model, Serialize, Deserialize)]
#[model(
    collection_name = "leagues",
    index(keys = r#"doc!{"name": 1}"#, options = r#"doc!{"unique": true}"#)
)]
#[serde(rename_all = "camelCase")]
pub struct League {
    /// The ID of the model.
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,

    /// Owner: User!
    pub owner: ObjectId,
    // Users: [User!]!
    pub users: Vec<ObjectId>,
}

#[Object]
impl League {
    async fn id(&self) -> ID {
        if let Some(id) = &self.id {
            ID::from(id)
        } else {
            let crap = String::from("");
            ID::from(crap)
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

impl League {
    pub fn new_league(name: &str, owner_id: &str) -> Self {
        let oid = ObjectId::with_string(&owner_id).expect("Can't get id from String");

        let users: Vec<ObjectId> = vec![oid.clone()];

        League {
            id: None,
            name: String::from(name),
            owner: oid,
            users: users,
        }
    }

    pub async fn find_all(db: &Database) -> Result<Vec::<Self>> {
        let mut leagues: Vec<League> = Vec::new();
        let mut cursor = League::find(&db, None, None).await?;

        while let Some(league) = cursor.next().await {
            leagues.push(league.unwrap());
        }

        Ok(leagues)
    }

    pub async fn find_by_id(db: &Database, id: &ID) -> Option<Self> {
        let oid = ObjectId::with_string(id).expect("Can't get id from String");
        League::find_one(&db, doc! { "_id": oid }, None).await.unwrap()
    }

    pub async fn find_by_name(db: &Database, name: &str) -> Option<Self> {
        League::find_one(&db, doc! { "username": name }, None)
            .await
            .unwrap()
    }

    pub async fn find_by_user_id(db: &Database, user_id: &ID) -> Result<Vec::<Self>> {
        let oid = ObjectId::with_string(&user_id).expect("Can't get id from String");
        let cursor = League::find(&db, doc! {"users": oid }, None).await?;

        let leagues: Vec<League> = cursor.try_collect().await?;

        Ok(leagues)
    }

    pub async fn find_by_owner_id(db: &Database, owner_id: &ID) -> Result<Vec::<Self>> {
        let oid = ObjectId::with_string(&owner_id).expect("Can't get id from String");
        let cursor = League::find(&db, doc! {"ownerId": oid }, None).await?;

        let leagues: Vec<League> = cursor.try_collect().await?; 

        Ok(leagues)
    }

    // mutation
    pub async fn add_user(db: &Database, id: String, user_id: String) -> Result<Self> {
        let query = doc! {
            "_id": ObjectId::with_string(&id)?
        };

        if let Some(mut league) = League::find_one(db, Some(query), None).await? {
            league.users.push(ObjectId::with_string(&user_id)?);

            league.save(db, None).await?;

            Ok(league)
        } else {
            Err(format!("League with id: {:?} not found", &id).into())
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
