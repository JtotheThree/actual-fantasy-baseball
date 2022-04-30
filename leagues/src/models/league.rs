use std::fmt;

use async_graphql::*;
use futures::stream::TryStreamExt;
use strum_macros::{EnumIter, EnumString};
use serde::{Deserialize, Serialize};
use wither::prelude::*;
use wither::{bson::{doc, oid::ObjectId, Document}, bson, mongodb::Database};

use crate::graphql::UpdateLeagueInput;

#[derive(Copy, Clone, Debug, Eq, EnumIter, PartialEq, Enum, EnumString, Serialize, Deserialize)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LeagueStatus {
    PlayerGeneration,
    PlayersCompleted,
}

#[derive(Copy, Clone, Eq, EnumIter, PartialEq, Enum, EnumString, Serialize, Deserialize)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LeagueState {
    Manual,
    Created,
    Drafting,
    SeasonStart,
    Playoffs,
    RealmSeries,
    SeasonEnd,
}

impl fmt::Display for LeagueState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            LeagueState::Manual => write!(f, "Manual"),
            LeagueState::Created => write!(f, "Created"),
            LeagueState::Drafting => write!(f, "Drafting"),
            LeagueState::SeasonStart=> write!(f, "Season Start"),
            LeagueState::Playoffs => write!(f, "Playoffs"),
            LeagueState::RealmSeries=> write!(f, "Realm Series"),
            LeagueState::SeasonEnd=> write!(f, "Season End"),
        }
    }
}

impl fmt::Debug for LeagueState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            LeagueState::Manual => write!(f, "MANUAL"),
            LeagueState::Created => write!(f, "CREATED"),
            LeagueState::Drafting => write!(f, "DRAFTING"),
            LeagueState::SeasonStart=> write!(f, "SEASON_START"),
            LeagueState::Playoffs => write!(f, "PLAYOFFS"),
            LeagueState::RealmSeries=> write!(f, "REALM_SERIES"),
            LeagueState::SeasonEnd=> write!(f, "SEASON_END"),
        }
    }
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

    pub status: LeagueStatus,

    /// Owner: User!
    pub owner: String,
    // Users: [User!]!
    pub managers: Vec<String>,

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
        //let oid = ObjectId::with_string(&owner_id).expect("Can't get id from String");

        let managers: Vec<String> = vec![owner_id.to_string()];

        let mut state = LeagueState::Created;

        if manual_state {
            state = LeagueState::Manual;
        }

        League {
            id: None,
            name: String::from(name),
            description: String::from(description),
            owner: owner_id.to_string(),
            draft_start: None,
            season_start: None,
            season_end: None,
            games_per_season: None,
            playoff_rounds: None,
            realm_series_games: None,
            status: LeagueStatus::PlayerGeneration,
            state,
            manual_state,
            password,
            public,
            managers,
            max_players,
        }
    }

    pub async fn find_all(db: &Database, filter: Option<Document>) -> Result<Vec::<Self>> {
        let cursor = League::find(&db, filter, None).await?;
        let leagues: Vec<League> = cursor.try_collect().await?;

        Ok(leagues)
    }

    pub async fn find_by_id(db: &Database, id: &ID) -> Option<Self> {
        let oid = ObjectId::with_string(id).expect("Can't get id from String");
        League::find_one(&db, doc! { "_id": oid }, None).await.unwrap()
    }

    /* pub async fn find_by_name(db: &Database, name: &str) -> Option<Self> {
        League::find_one(&db, doc! { "username": name }, None)
            .await
            .unwrap()
    } */

    pub async fn find_by_user_id(db: &Database, user_id: &str) -> Result<Vec::<Self>> {
        //let oid = ObjectId::with_string(&user_id).expect("Can't get id from String");
        let cursor = League::find(&db, doc! {"managers": user_id }, None).await?;

        let leagues: Vec<League> = cursor.try_collect().await?;

        Ok(leagues)
    }

    pub async fn find_by_owner_id(db: &Database, owner_id: &str) -> Result<Vec::<Self>> {
        //let oid = ObjectId::with_string(&owner_id).expect("Can't get id from String");
        let cursor = League::find(&db, doc! {"ownerId": owner_id }, None).await?;

        let leagues: Vec<League> = cursor.try_collect().await?;

        Ok(leagues)
    }

    pub async fn update(db: &Database, id: &ID, input: UpdateLeagueInput) -> Result<Self> {
        let query = doc! {
            "_id": ObjectId::with_string(&id)?
        };

        if let Some(mut league) = League::find_one(db, Some(query), None).await? {
            if let Some(description) = input.description {
                league.description = description;
            }

            if let Some(state) = input.state {
                league.state = state;
            }

            if let Some(password) = input.password {
                league.password = Some(password);
            } else {
                league.password = None;
            }

            if let Some(public) = input.public {
                league.public = public;
            }

            if let Some(status) = input.status {
                league.status = status;
            }

            if let Some(max_players) = input.max_players {
                league.max_players = max_players;
            }

            league.save(db, None).await?;

            Ok(league)
        } else {
            Err(format!("league with id: {:?} not found", &id).into())
        }
    }

    // mutation
    pub async fn add_manager(db: &Database, id: String, user_id: String) -> Result<Self> {
        let query = doc! {
            "_id": ObjectId::with_string(&id)?
        };

        if let Some(mut league) = League::find_one(db, Some(query), None).await? {
            league.managers.push(user_id);

            league.save(db, None).await?;

            Ok(league)
        } else {
            Err(format!("league with id: {:?} not found", &id).into())
        }
    }

    pub async fn set_league_state(db: &Database, id: String, state: LeagueState) -> Result<Self> {
        let query = doc! {
            "_id": ObjectId::with_string(&id)?
        };

        if let Some(mut league) = League::find_one(db, Some(query), None).await? {
            league.state = state;

            league.save(db, None).await?;

            Ok(league)
        } else {
            Err(format!("League with id: {:?} not found", &id).into())
        }
    }

    pub async fn set_league_status(db: &Database, id: String, status: LeagueStatus) -> Result<Self> {
        let query = doc! {
            "_id": ObjectId::with_string(&id)?
        };

        if let Some(mut league) = League::find_one(db, Some(query), None).await? {
            league.status = status;

            league.save(db, None).await?;

            Ok(league)
        } else {
            Err(format!("League with id: {:?} not found", &id).into())
        }
    }


}
