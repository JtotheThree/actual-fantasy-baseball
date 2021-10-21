use async_graphql::*;
use futures::stream::{StreamExt, TryStreamExt};
use serde::{Deserialize, Serialize};
use wither::prelude::*;
use wither::{bson::{doc, oid::ObjectId}, mongodb::Database};

/// League representation
#[derive(Clone, Debug, Model, Serialize, Deserialize)]
#[model(
    collection_name = "teams",
    index(keys = r#"doc!{"name": 1}"#, options = r#"doc!{"unique": true}"#)
)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    /// The ID of the model.
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,

    pub league: ObjectId,
    pub owner: ObjectId,
}

impl Team {
    pub fn new_team(name: &str, league_id: &str, owner_id: &str
    ) -> Self {
        let league_id = ObjectId::with_string(&league_id).expect("Can't get id from String");
        let owner_id = ObjectId::with_string(&owner_id).expect("Can't get id from String");

        Team {
            id: None,
            name: String::from(name),
            league: league_id,
            owner: owner_id,
        }
    }

    pub async fn find_all(db: &Database) -> Result<Vec::<Self>> {
        let cursor = Team::find(&db, None, None).await?;
        let teams: Vec<Team> = cursor.try_collect().await?;

        Ok(teams)
    }

    pub async fn find_by_id(db: &Database, id: &str) -> Option<Self> {
        let id = ObjectId::with_string(id).expect("Can't get id from String");
        Team::find_one(&db, doc! { "_id": id }, None).await.unwrap()
    }

    pub async fn find_by_name(db: &Database, name: &str) -> Option<Self> {
        Team::find_one(&db, doc! { "name": name }, None)
            .await
            .unwrap()
    }

    pub async fn find_by_owner_id(db: &Database, owner_id: &str) -> Result<Vec::<Self>> {
        let owner_id = ObjectId::with_string(&owner_id).expect("Can't get id from String");
        let cursor = Team::find(&db, doc! {"ownerId": owner_id }, None).await?;

        let teams: Vec<Team> = cursor.try_collect().await?; 

        Ok(teams)
    }

    pub async fn find_by_league_id(db: &Database, league_id: &str) -> Result<Vec::<Self>> {
        let league_id = ObjectId::with_string(&league_id).expect("Can't get id from String");
        let cursor = Team::find(&db, doc! {"leagueId": league_id }, None).await?;

        let teams: Vec<Team> = cursor.try_collect().await?; 

        Ok(teams)
    }

    pub async fn find_user_team_for_league(
        db: &Database, 
        owner_id: &str, 
        league_id: &str
    ) -> Option<Self> {
        let league_id = ObjectId::with_string(&league_id).expect("Can't get id from String");
        let owner_id = ObjectId::with_string(&owner_id).expect("Can't get id from String");

        Team::find_one(&db, doc! { "leagueId": league_id, "ownerId": owner_id }, None).await.unwrap()
    }    
}