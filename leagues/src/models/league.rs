use async_graphql::*;
use futures::stream::StreamExt;
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
    pub owner_id: ObjectId,
}

#[Object]
impl League {
    async fn id(&self) -> String {
        if let Some(id) = &self.id {
            id.clone().to_string()
        } else {
            String::from("")
        }
    }

    async fn name(&self) -> &str {
        &self.name
    }
}

impl League {
    pub fn new_league(name: &str, owner_id: &str) -> Self {
        let oid = ObjectId::with_string(&owner_id).expect("Can't get id from String");

        League {
            id: None,
            name: String::from(name),
            owner_id: oid,
        }
    }

    pub async fn get_all(db: &Database) -> Result<Vec::<Self>> {
        let mut leagues: Vec<League> = Vec::new();
        let mut cursor = League::find(&db, None, None).await?;

        while let Some(league) = cursor.next().await {
            leagues.push(league.unwrap());
        }

        Ok(leagues)
    }

    pub async fn get_by_id(db: &Database, id: ID) -> Option<Self> {
        let oid = ObjectId::with_string(&id).expect("Can't get id from String");
        League::find_one(&db, doc! { "_id": oid }, None).await.unwrap()
    }

    pub async fn get_by_name(db: &Database, name: &str) -> Option<Self> {
        League::find_one(&db, doc! { "username": name }, None)
            .await
            .unwrap()
    }

    pub async fn get_by_owner_id(db: &Database, owner_id: &ID) -> Result<Vec::<Self>> {
        let oid = ObjectId::with_string(&owner_id).expect("Can't get id from String");
        let mut leagues: Vec<League> = Vec::new();
        let mut cursor = League::find(&db, doc! {"ownerId": oid }, None).await?;

        while let Some(league) = cursor.next().await {
            leagues.push(league.unwrap());
        }

        Ok(leagues)
    }
}

#[derive(Clone, InputObject)]
pub struct CreateLeagueInput {
    pub name: String,
}

//Test commit 