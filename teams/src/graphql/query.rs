use std::collections::HashMap;

use crate::models::{Lineup, Team};

use common::*;
use common::enums::Position;
use async_graphql::*;
use jsonwebtoken::TokenData;
use wither::prelude::*;
use wither::{mongodb::Database};

use common::filter::process_filter;

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

/// Object Implementations
#[Object]
impl Team {
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

    async fn owner(&self) -> User {
        User{ id: ID::from(&self.owner) }
    }

    async fn league(&self) -> League {
        League{ id: ID::from(&self.league) }
    }

    async fn gold(&self) -> i64 {
        self.gold
    }

    async fn roster(&self) -> Roster {
        Roster {
            starting_pitcher: option_to_player(&self.roster.starting_pitcher),
            relief_pitchers: vec_to_players(&self.roster.relief_pitchers),
            catcher: option_to_player(&self.roster.catcher),
            catcher_reserves: vec_to_players(&self.roster.catcher_reserves),
            first_base: option_to_player(&self.roster.first_base),
            second_base: option_to_player(&self.roster.second_base),
            third_base: option_to_player(&self.roster.third_base),
            shortstop: option_to_player(&self.roster.shortstop),
            infield_reserves: vec_to_players(&self.roster.infield_reserves),
            left_field: option_to_player(&self.roster.left_field),
            center_field: option_to_player(&self.roster.center_field),
            right_field: option_to_player(&self.roster.right_field),
            outfield_reserves: vec_to_players(&self.roster.outfield_reserves),
        }
    }

    async fn lineup(&self) -> &Lineup {
        &self.lineup
    }
}

// USER
pub struct User {
    pub id: ID,
}

// LEAGUE
pub struct League {
    pub id: ID,
}

// PLAYER
pub struct Player {
    pub id: ID,
}

/*impl From<Option<String>> for Player {
    fn from(item: String) -> Self {
        if let Some(item) = item {
            Player {
                id: Some(ID::from(item)),
            }
        } else {
            Player {
                id: "",
            }
        }
    }
}*/

fn option_to_player(item: &Option<String>) -> Option<Player> {
    if let Some(item) = item {
        Some(Player { id: ID::from(item) })
    } else {
        None
        //Player { id: ID::from("") }
    }
}

fn vec_to_players(list: &Vec<String>) -> Vec<Player> {
    list.iter().map(|id| Player{
        id: ID::from(id)
    })
    .collect()
}

#[derive(SimpleObject)]
pub struct Roster {
    pub starting_pitcher: Option<Player>,
    pub relief_pitchers: Vec<Player>,
    pub catcher: Option<Player>,
    pub catcher_reserves: Vec<Player>,
    pub first_base: Option<Player>,
    pub second_base: Option<Player>,
    pub third_base: Option<Player>,
    pub shortstop: Option<Player>,
    pub infield_reserves: Vec<Player>,
    pub left_field: Option<Player>,
    pub center_field: Option<Player>,
    pub right_field: Option<Player>,
    pub outfield_reserves: Vec<Player>,
}

/*impl From<crate::models::Roster> for Roster {
    fn from(item: crate::models::Roster) -> Self {
        Roster {
            starting_pitcher: option_to_player(item.starting_pitcher),
            relief_pitchers: vec_to_players(item.relief_pitchers),
            catcher: option_to_player(item.catcher),
            catcher_reserves: vec_to_players(item.catcher_reserves),
            first_base: option_to_player(item.first_base),
            second_base: option_to_player(item.second_base),
            third_base: option_to_player(item.third_base),
            shortstop: option_to_player(item.shortstop),
            infield_reserves: vec_to_players(item.infield_reserves),
            left_field: option_to_player(item.left_field),
            center_field: option_to_player(item.center_field),
            right_field: option_to_player(item.right_field),
            outfield_reserves: vec_to_players(item.outfield_reserves),
        }
    }
}*/

#[Object(extends, cache_control(max_age = 60))]
impl User {
    #[graphql(external)]
    async fn id(&self) -> &ID {
        &self.id
    }

    async fn teams(&self, ctx: &Context<'_>) -> Result<Vec<Team>> {
        let db: &Database = ctx.data().expect("Cannot connect to database");

        let maybe_teams = Team::find_by_owner_id(db, &self.id).await;

        if let Ok(teams) = maybe_teams {
            Ok(teams)
        } else {
            Err("Can't get leagues for owner".into())
        }
    }
}

#[Object(extends, cache_control(max_age = 60))]
impl League {
    #[graphql(external)]
    async fn id(&self) -> &ID {
        &self.id
    }

    async fn teams(&self, ctx: &Context<'_>) -> Result<Vec<Team>> {
        let db: &Database = ctx.data().expect("Cannot connect to database");

        let maybe_teams = Team::find_by_league_id(db, &self.id).await;

        if let Ok(teams) = maybe_teams {
            Ok(teams)
        } else {
            Err("Can't get teams for league".into())
        }
    }
}

#[Object(extends, cache_control(max_age = 60))]
impl Player {
    #[graphql(external)]
    async fn id(&self) -> &ID {
        &self.id
    }

    /*async fn team(&self, ctx: &Context<'_>) -> Option<Team> {
        let db: &Database = ctx.data().expect("Cannot connect to database");

        if let Some(id) = &self.id {
            let maybe_team = Team::find_by_player_id(db, &id).await;

            info!("Searching for team by {:?}", self.id);

            if let Ok(team) = maybe_team {
                Some(team)
            } else {
                //Err("Can't get teams for league".into());
                None
            }
        } else {
            None
        }
    }*/
}

/// Query
pub struct Query;

#[Object(extends, cache_control(max_age = 60))]
impl Query {
    async fn teams(&self, ctx: &Context<'_>, filter: Option<HashMap<String, serde_json::Value>>) -> Result<Vec<Team>> {
        let db: &Database = ctx.data().expect("Cannot connect to database");

        match filter {
            Some(filter) => {
                let filter = process_filter(filter)?;
                let teams =  Team::find_all(db, Some(filter)).await?;

                Ok(teams)
            },
            None => {
                let teams =  Team::find_all(db, None).await?;

                Ok(teams)
            }
        }

    }

    async fn team(&self, ctx: &Context<'_>, id: ID) -> Result<Team> {
        let db: &Database = ctx.data().expect("Cannot connect to database");

        let maybe_team = Team::find_by_id(db, &id).await;

        if let Some(team) = maybe_team {
            Ok(team)
        } else {
            Err("Can't get team by id".into())
        }
    }

    #[graphql(entity)]
    async fn find_user_by_id(&self, id: ID) -> User {
        User { id }
    }

    #[graphql(entity)]
    async fn find_league_by_id(&self, id: ID) -> League {
        League { id }
    }

    #[graphql(entity)]
    async fn find_team_by_id(&self, ctx: &Context<'_>, id: ID) -> Result<Team> {
        let db: &Database = ctx.data()?;
        let maybe_team = Team::find_by_id(db, &id).await;
        if let Some(team) = maybe_team {
            Ok(team)
        } else {
            Err("No user found".into())
        }
    }
}
/// Mutation
pub struct Mutation;

#[Object(extends, cache_control(max_age = 60))]
impl Mutation {
    async fn create_team(&self, ctx: &Context<'_>, name: String, league_id: ID) -> Result<Team, Error> {
        let db: &Database = ctx.data()?;
        let redis_client: &redis::Client = ctx.data()?;

        let mut con = redis_client.get_connection()?;
        let token_data = ctx.data_opt::<TokenData<Claims>>().unwrap();

        let maybe_current_user = get_current_user(&mut con, token_data);

        if let Some(current_user) = maybe_current_user {
            let mut new_team = Team::new_team(&name, &league_id, &current_user.id);

            if let Ok(_) = new_team.save(&db, None).await {
                Ok(new_team)
            } else {
                Err(Error::new("Can't create team, bad user"))
            }
        } else {
            Err("Can't create team".into())
        }
    }

    async fn set_player_position(&self, ctx: &Context<'_>, team: ID, player: ID, position: Position) -> Result<Team> {
        let db: &Database = ctx.data()?;

        if let Some(mut team) = Team::find_by_id(db, &team).await {
            let player_id = player.to_string();

            match position {
                Position::Reserve => {},
                Position::StartingPitcher => {team.roster.starting_pitcher = Some(player_id);},
                Position::ReliefPitcher => {team.roster.relief_pitchers.push(player_id);},
                Position::Catcher => {team.roster.catcher = Some(player_id);},
                Position::ReserveCatcher => {team.roster.catcher_reserves.push(player_id);},
                Position::FirstBase => {team.roster.first_base = Some(player_id);},
                Position::SecondBase => {team.roster.second_base = Some(player_id);},
                Position::ThirdBase => {team.roster.third_base = Some(player_id);},
                Position::Shortstop => {team.roster.shortstop = Some(player_id);},
                Position::InfieldReserve => {team.roster.infield_reserves.push(player_id);},
                Position::LeftField => {team.roster.left_field = Some(player_id);},
                Position::CenterField => {team.roster.center_field = Some(player_id);},
                Position::RightField => {team.roster.right_field = Some(player_id);},
                Position::OutfieldReserve => {team.roster.outfield_reserves.push(player_id);},
            }

            team.save(db, None).await?;
            Ok(team)
        } else {
            Err("Can't find team by id".into())
        }
    }

    async fn modify_gold(&self, ctx: &Context<'_>, id: ID, cost: i64) -> Result<Team> {
        let db: &Database = ctx.data()?;

        Team::modify_gold(db, &id, cost).await
    }

    /*async fn add_player(&self, ctx: &Context<'_>, id: ID, player_id: ID, cost: i64) -> Result<Team> {
        let db: &Database = ctx.data()?;

        Team::add_player(db, &id, &player_id, cost).await

        let redis_client: &redis::Client = ctx.data()?;

        let mut con = redis_client.get_connection()?;
        let token_data = ctx.data_opt::<TokenData<Claims>>().unwrap();

        let maybe_current_user = get_current_user(&mut con, token_data);

        if let Some(current_user) = maybe_current_user {
            let mut new_team = Team::new_team(&name, &league_id, &current_user.id);

            if let Ok(_) = new_team.save(&db, None).await {
                Ok(new_team)
            } else {
                Err(Error::new("Can't create team, bad user"))
            }
        } else {
            Err("Can't create team".into())
        }
    }*/


}
