use async_graphql::*;
use futures::stream::TryStreamExt;
use serde::{Deserialize, Serialize};
use wither::bson::Document;
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

    pub league: String,
    pub owner: String,

    pub gold: i64,
    pub roster: Roster,
    pub lineup: Vec<Option<LineupSlot>>,
    pub starting_pitcher: Option<StartingPitcher>,
    pub reserves: Vec<Option<String>>,
    pub pitchers: Vec<Option<String>>,
}

#[derive(Clone, Debug, Serialize, Deserialize, SimpleObject)]
#[serde(rename_all = "camelCase")]
pub struct Roster {
    pub starting_pitchers: Vec<Option<String>>,
    pub relief_pitchers: Vec<Option<String>>,
    pub catchers: Vec<Option<String>>,
    pub infielders: Vec<Option<String>>,
    pub outfielders: Vec<Option<String>>,
}

#[derive(Clone, Debug, Serialize, Deserialize, SimpleObject)]
#[serde(rename_all = "camelCase")]
pub struct LineupSlot {
    pub player_id: String,
    pub position: String,
    pub sub_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, SimpleObject)]
#[serde(rename_all = "camelCase")]
pub struct StartingPitcher {
    pub player_id: String,
    pub sub1_id: String,
    pub sub2_id: Option<String>,
}

impl Roster {
    pub fn new() -> Self {
        Roster {
            starting_pitchers: vec![None; 5],
            relief_pitchers: vec![None; 7],
            catchers: vec![None; 2],
            infielders: vec![None; 6],
            outfielders: vec![None; 5],
        }
    }
}

impl Team {
    pub fn new_team(name: &str, league_id: &str, owner_id: &str
    ) -> Self {
        Team {
            id: None,
            name: String::from(name),
            league: league_id.to_string(),
            owner: owner_id.to_string(),
            gold: 500000,
            lineup: vec![None; 9],
            starting_pitcher: None,
            reserves: vec![None; 6],
            pitchers: vec![None; 6],
            roster: Roster::new(),
        }
    }

    pub async fn find_all(db: &Database, filter: Option<Document>) -> Result<Vec::<Self>> {
        let cursor = Team::find(&db, filter, None).await?;
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
        let cursor = Team::find(&db, doc! {"owner": owner_id }, None).await?;

        let teams: Vec<Team> = cursor.try_collect().await?;

        Ok(teams)
    }

    pub async fn find_by_league_id(db: &Database, league_id: &str) -> Result<Vec::<Self>> {
        //let league_id = ObjectId::with_string(&league_id).expect("Can't get id from String");
        let cursor = Team::find(&db, doc! {"league": league_id }, None).await?;

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

        Team::find_one(&db, doc! { "league": league_id, "owner": owner_id }, None).await.unwrap()
    }
}
