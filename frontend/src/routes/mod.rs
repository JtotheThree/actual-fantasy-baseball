pub mod create_league;
pub mod create_team;
pub mod home;
pub mod join_league;
pub mod league;
pub mod login;
pub mod rules;
pub mod signup;
pub mod team;

use yew_router::prelude::*;

/// App routes
#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/login"]
    Login,
    #[to = "/signup"]
    Signup,
    #[to = "/rules"]
    Rules,
    #[to = "/create_league"]
    CreateLeagueForm,
    #[to = "/join_league"]
    JoinLeague,
    #[to = "/league/{id}"]
    League(String),
    #[to = "/create_team/{league_id}"]
    CreateTeamForm(String),
    #[to = "/team/{id}"]
    Team(String),
    #[to = "/"]
    Home,
}
