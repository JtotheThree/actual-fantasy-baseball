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
    pub lineup: Lineup,
}

/*#[derive(Clone, Debug, Default, Serialize, Deserialize, SimpleObject)]
#[serde(rename_all = "camelCase")]
pub struct Pitching {
    pub starting_pitcher: Option<String>,
    pub relief_pitchers: Vec<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, SimpleObject)]
#[serde(rename_all = "camelCase")]
pub struct Catching {
    pub catcher: Option<String>,
    pub reserve_catcher: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, SimpleObject)]
#[serde(rename_all = "camelCase")]
pub struct Infield {
    pub first_base: Option<String>,
    pub second_base: Option<String>,
    pub third_base: Option<String>,
    pub shortstop: Option<String>,
    pub reserves: Vec<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, SimpleObject)]
#[serde(rename_all = "camelCase")]
pub struct Outfield {
    pub left_field: Option<String>,
    pub center_field: Option<String>,
    pub right_field: Option<String>,
    pub reserves: Vec<String>,
}*/

#[derive(Clone, Debug, Default, Serialize, Deserialize, SimpleObject)]
#[serde(rename_all = "camelCase")]
pub struct Roster {
    pub starting_pitcher: Option<String>,
    pub relief_pitchers: Vec<String>,
    pub catcher: Option<String>,
    pub catcher_reserves: Vec<String>,
    pub first_base: Option<String>,
    pub second_base: Option<String>,
    pub third_base: Option<String>,
    pub shortstop: Option<String>,
    pub infield_reserves: Vec<String>,
    pub left_field: Option<String>,
    pub center_field: Option<String>,
    pub right_field: Option<String>,
    pub outfield_reserves: Vec<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, SimpleObject)]
#[serde(rename_all = "camelCase")]
pub struct Lineup {
    pub first: Option<String>,
    pub second: Option<String>,
    pub third: Option<String>,
    pub fourth: Option<String>,
    pub fifth: Option<String>,
    pub sixth: Option<String>,
    pub seventh: Option<String>,
    pub eighth: Option<String>,
    pub ninth: Option<String>,
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
            roster: Roster::default(),
            lineup: Lineup::default(),
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

    /*pub async fn find_by_name(db: &Database, name: &str) -> Option<Self> {
        Team::find_one(&db, doc! { "name": name }, None)
            .await
            .unwrap()
    }*/

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

    /*pub async fn find_user_team_for_league(
        db: &Database,
        owner_id: &str,
        league_id: &str
    ) -> Option<Self> {
        let league_id = ObjectId::with_string(&league_id).expect("Can't get id from String");
        let owner_id = ObjectId::with_string(&owner_id).expect("Can't get id from String");

        Team::find_one(&db, doc! { "league": league_id, "owner": owner_id }, None).await.unwrap()
    }

    pub async fn find_by_player_id(db: &Database, player_id: &str) -> Result<Self> {
        if let Some(team) = Team::find_one(&db, doc! { "players": player_id }, None).await? {
            Ok(team)
        } else {
            Err(format!("no team found for player with id: {:?}", &player_id).into())
        }
    }*/

    pub async fn modify_gold(db: &Database, id: &str, gold: i64) -> Result<Self> {
        let query = doc! {
            "_id": ObjectId::with_string(id)?
        };

        if let Some(mut team) = Team::find_one(db, Some(query), None).await? {
            team.gold = team.gold + gold;

            team.save(db, None).await?;

            Ok(team)
        } else {
            Err(format!("team with id: {:?} not found", &id).into())
        }
    }

    /*pub async fn add_player(db: &Database, id: &str, player_id: &str, cost: i64) -> Result<Self> {
        let query = doc! {
            "_id": ObjectId::with_string(id)?
        };

        if let Some(mut team) = Team::find_one(db, Some(query), None).await? {
            team.players.push(player_id.to_string());
            team.gold = team.gold - cost;

            team.save(db, None).await?;

            Ok(team)
        } else {
            Err(format!("team with id: {:?} not found", &id).into())
        }
    }*/
}
