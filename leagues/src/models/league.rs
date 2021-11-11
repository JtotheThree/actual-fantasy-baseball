use async_graphql::*;
use futures::stream::TryStreamExt;
use std::collections::HashMap;
use strum_macros::EnumString;
use serde::{Deserialize, Serialize};
use wither::prelude::*;
use wither::{bson::{doc, oid::ObjectId, Document}, bson, mongodb::Database};

#[derive(Copy, Clone, Eq, PartialEq, Debug, Enum, EnumString, Serialize, Deserialize)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum LeagueState {
    Manual,
    Created,
    Drafting,
    SeasonStart,
    Playoffs,
    RealmSeries,
    SeasonEnd,
}

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
    pub description: String,

    pub public: bool,
    pub password: Option<String>,
    pub max_players: i64,

    pub state: LeagueState,
    pub manual_state: bool,

    /// Owner: User!
    pub owner: ObjectId,
    // Users: [User!]!
    pub managers: Vec<ObjectId>,

    // Season
    pub draft_start: Option<bson::DateTime>,
    pub season_start: Option<bson::DateTime>,
    pub season_end: Option<bson::DateTime>,
    pub games_per_season: Option<i64>,
    pub playoff_rounds: Option<i64>,
    pub realm_series_games: Option<i64>,
}

impl League {
    pub fn new_league(
        name: &str,
        description: &str,
        public: bool,
        password: Option<String>,
        max_players: i64,
        manual_state: bool,
        owner_id: &str,
    ) -> Self {
        let oid = ObjectId::with_string(&owner_id).expect("Can't get id from String");

        let managers: Vec<ObjectId> = vec![oid.clone()];

        let mut state = LeagueState::Created;

        if manual_state {
            state = LeagueState::Manual;
        }

        League {
            id: None,
            name: String::from(name),
            description: String::from(description),
            owner: oid,
            draft_start: None,
            season_start: None,
            season_end: None,
            games_per_season: None,
            playoff_rounds: None,
            realm_series_games: None,
            state,
            manual_state,
            password,
            public,
            managers,
            max_players,
        }
    }

    pub async fn find_all(db: &Database, filter: Option<Document>) -> Result<Vec::<Self>> {
        info!("{:?}", filter);

        let doc = doc!{ "public": true };
        info!("{:?}", doc);

        let cursor = League::find(&db, filter, None).await?;
        let leagues: Vec<League> = cursor.try_collect().await?;

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
        let cursor = League::find(&db, doc! {"managers": oid }, None).await?;

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
    pub async fn add_manager(db: &Database, id: String, user_id: String) -> Result<Self> {
        let query = doc! {
            "_id": ObjectId::with_string(&id)?
        };

        if let Some(mut league) = League::find_one(db, Some(query), None).await? {
            league.managers.push(ObjectId::with_string(&user_id)?);

            league.save(db, None).await?;

            Ok(league)
        } else {
            Err(format!("League with id: {:?} not found", &id).into())
        }
    }
    
}
