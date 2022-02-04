use crate::models::*;
use common::*;
use async_graphql::*;
use jsonwebtoken::TokenData;
use strum::IntoEnumIterator;
use wither::prelude::*;
use wither::{mongodb::Database, bson::Document};

use std::collections::HashMap;
use std::convert::TryFrom;

use common::enums::Class;
use common::filter::process_filter;
use common::meta::MetaSelect;

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

#[Object]
impl Player {
    async fn id(&self) -> ID {
        if let Some(id) = &self.id {
            ID::from(id)
        } else {
            ID::from("")
        }
    }

    async fn name(&self) -> &str {
        &self.name
    }

    async fn league(&self) -> League {
        League{ id: ID::from(&self.league) }
    }

    async fn team(&self) -> Option<Team> {
        if let Some(id) = &self.team {
            Some(Team{ id: ID::from(&id) })
        } else {
            None
        }
    }

    async fn cost(&self) -> i64 {
        self.cost
    }

    async fn gender(&self) -> enums::Gender {
        self.gender
    }

    async fn race(&self) -> enums::Race {
        self.race
    }

    async fn class(&self) -> enums::Class {
        self.class
    }

    async fn health(&self) -> i64 {
        self.health
    }

    async fn max_health(&self) -> i64 {
        self.max_health
    }

    async fn strength(&self) -> i64 {
        self.strength
    }

    async fn dexterity(&self) -> i64 {
        self.dexterity
    }

    async fn constitution(&self) -> i64 {
        self.constitution
    }

    async fn intelligence(&self) -> i64 {
        self.intelligence
    }

    async fn wisdom(&self) -> i64 {
        self.wisdom
    }

    async fn charisma(&self) -> i64 {
        self.charisma
    }

    async fn traits(&self) -> Vec::<enums::Trait> {
        self.traits.clone()
    }

    async fn hidden_traits(&self) -> Option<Vec::<enums::Trait>> {
        if let Some(traits) = &self.hidden_traits {
            Some(traits.clone())
        } else {
            None
        }
    }
}

pub struct League {
    pub id: ID,
}

pub struct Team {
    pub id: ID,
}

pub struct Query;

#[Object(extends, cache_control(max_age = 60))]
impl Query {
    async fn players(
        &self, ctx: &Context<'_>,
        filter: Option<HashMap<String, serde_json::Value>>,
        sort: Option<HashMap<String, serde_json::Value>>,
    ) -> Result<Vec<Player>>
    {
        let db: &Database = ctx.data().expect("Can't connect to database");

        let filter = if let Some(filter) = filter {
            let filter = process_filter(filter)?;
            Some(filter)
        } else {
            None
        };

        let sort = if let Some(sort) = sort {
            let sort: serde_json::Map<String, serde_json::Value> = sort.into_iter().collect();
            let sort = Document::try_from(sort)?;
            Some(sort)
        } else {
            None
        };

        let players = Player::find_all(db, filter, sort).await?;

        Ok(players)
    }

    async fn player(&self, ctx: &Context<'_>, id: ID) -> Result<Player> {
        let db: &Database = ctx.data().expect("Cannot connect to database");

        let maybe_player = Player::find_by_id(db, &id).await;

        if let Some(player) = maybe_player {
            Ok(player)
        } else {
            Err("Can't find player by id".into())
        }
    }

    #[graphql(entity)]
    async fn find_league_by_id(&self, id: ID) -> League {
        League { id }
    }

    #[graphql(entity)]
    async fn find_team_by_id(&self, id: ID) -> Team {
        Team { id }
    }

    #[graphql(entity)]
    async fn find_player_by_id(&self, ctx: &Context<'_>, id: ID) -> Result<Player> {
        let db: &Database = ctx.data()?;
        let maybe_player = Player::find_by_id(db, &id).await;
        if let Some(player) = maybe_player {
            Ok(player)
        } else {
            Err("No player found".into())
        }
    }

    async fn meta_class(&self) -> MetaSelect {
        let mut select_values = MetaSelect::default();

        for value in Class::iter() {
            select_values.values.push(format!("{:?}", value));
            select_values.labels.push(format!("{}", value));
        }

        select_values
    }
}

#[Object(extends, cache_control(max_age = 60))]
impl League {
    #[graphql(external)]
    async fn id(&self) -> &ID {
        &self.id
    }

    async fn players(&self, ctx: &Context<'_>) -> Result<Vec<Player>> {
        let db: &Database = ctx.data().expect("Cannot connect to database");

        let maybe_players = Player::find_by_league(db, &self.id).await;

        if let Ok(players) = maybe_players {
            Ok(players)
        } else {
            Err("Can't get players for league".into())
        }
    }
}

#[Object(extends, cache_control(max_age = 60))]
impl Team {
    #[graphql(external)]
    async fn id(&self) -> &ID {
        &self.id
    }

    async fn players(&self, ctx: &Context<'_>) -> Result<Vec<Player>> {
        let db: &Database = ctx.data().expect("Cannot connect to database");

        let maybe_players = Player::find_by_team(db, &self.id).await;

        if let Ok(players) = maybe_players {
            Ok(players)
        } else {
            Err("Can't get players for league".into())
        }
    }
}


pub struct Mutation;

#[Object(extends, cache_control(max_age = 60))]
impl Mutation {
    async fn create_player(&self, ctx: &Context<'_>, input: CreatePlayerInput) -> Result<Player, Error> {
        let db: &Database = ctx.data()?;

        // TODO: Restrict to integrations role only
        //
        /*let redis_client: &redis::Client = ctx.data()?;

        let mut con = redis_client.get_connection()?;
        let token_data = ctx.data_opt::<TokenData<Claims>>().unwrap();

        let maybe_current_user = get_current_user(&mut con, token_data);

        if maybe_current_user.is_none() {
            return Err("Creating players requires authentication".into())
        }*/

        let mut new_player = Player::new_player(input);

        new_player.save(&db, None).await?;

        Ok(new_player)
    }
}



#[derive(Clone, InputObject)]
pub struct CreatePlayerInput {
    pub name: String,
    pub league: String,
    pub cost: i64,
    pub gender: enums::Gender,
    pub race: enums::Race,
    pub class: enums::Class,
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
