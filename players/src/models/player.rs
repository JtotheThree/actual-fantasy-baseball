use async_graphql::*;
use futures::stream::TryStreamExt;
use serde::{Deserialize, Serialize};
use wither::prelude::*;
use wither::{bson::{doc, oid::ObjectId, Document}, mongodb::{Database, options::FindOptions}};

use common::enums;
use crate::graphql::CreatePlayerInput;

/// Player representation
#[derive(Clone, Debug, Model, Serialize, Deserialize)]
#[model(
    collection_name = "players",
    index(keys = r#"doc!{"name": 1}"#)
)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    /// The ID of the model.
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,

    pub league: String,
    pub team: Option<String>,

    pub cost: i64,

    pub gender: enums::Gender,
    pub race: enums::Race,
    pub class: enums::Class,
    pub handedness: enums::Handedness,

    pub health: i64,
    pub max_health: i64,

    pub strength: i64,
    pub dexterity: i64,
    pub constitution: i64,
    pub intelligence: i64,
    pub wisdom: i64,
    pub charisma: i64,

    pub traits: Vec::<enums::Trait>,
    pub hidden_traits: Option<Vec::<enums::Trait>>,
}

impl Player {
    pub fn new_player(
        input: CreatePlayerInput,
    ) -> Self {
        let health = input.max_health;

        Player {
            id: None,
            name: String::from(input.name),
            league: String::from(input.league),
            team: None,
            cost: input.cost,
            gender: input.gender,
            race: input.race,
            class: input.class,
            handedness: input.handedness,
            max_health: input.max_health,
            strength: input.strength,
            dexterity: input.dexterity,
            constitution: input.constitution,
            intelligence: input.intelligence,
            wisdom: input.wisdom,
            charisma: input.charisma,
            traits: input.traits,
            hidden_traits: input.hidden_traits,
            health,
        }
    }

    pub async fn find_all(db: &Database, filter: Option<Document>, sort: Option<Document>) -> Result<Vec::<Self>> {
        let options = FindOptions::builder().sort(sort).build();

        let cursor = Player::find(&db, filter, options).await?;


        let players: Vec<Player> = cursor.try_collect().await?;

        Ok(players)
    }

    pub async fn find_by_id(db: &Database, id: &str) -> Option<Self> {
        let id = ObjectId::with_string(id).expect("Can't get id from String");
        Player::find_one(&db, doc! { "_id": id }, None).await.unwrap()
    }

    pub async fn find_by_league(db: &Database, league_id: &str) -> Result<Vec::<Self>> {
        let cursor = Player::find(&db, doc!{ "league": league_id }, None).await?;
        let players: Vec<Player> = cursor.try_collect().await?;

        Ok(players)
    }

    pub async fn find_by_team(db: &Database, team_id: &str) -> Result<Vec::<Self>> {
        let cursor = Player::find(&db, doc!{ "team": team_id}, None).await?;
        let players: Vec<Player> = cursor.try_collect().await?;

        Ok(players)
    }

    pub async fn set_league(db: &Database, id: &str, league_id: &str) -> Result<Self> {
        let query = doc! {
            "_id": ObjectId::with_string(id)?
        };

        if let Some(mut player) = Player::find_one(db, Some(query), None).await? {
            player.league = league_id.to_string();

            player.save(db, None).await?;

            Ok(player)
        } else {
            Err(format!("Player with id: {:?} not found", id).into())
        }
    }

    pub async fn set_team(db: &Database, id: &str, team_id: &str) -> Result<Self> {
        let query = doc! {
            "_id": ObjectId::with_string(id)?
        };

        if let Some(mut player) = Player::find_one(db, Some(query), None).await? {
            player.team = Some(team_id.to_string());

            player.save(db, None).await?;

            Ok(player)
        } else {
            Err(format!("Player with id: {:?} not found", id).into())
        }
    }
}
