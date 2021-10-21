use crate::types::*;

use yew::agent::AgentLink;
use yewtil::store::{Store, StoreWrapper};

#[derive(Debug)]
pub enum Request {
    UpdateUser(Option<User>),
    UpdateLeague(Option<League>),
    UpdateTeam(Option<Team>),
}

#[derive(Debug)]
pub enum Action {
    SetUser(Option<User>),
    SetLeague(Option<League>),
    SetTeam(Option<Team>),
}

pub struct State {
    pub user: Option<User>,
    pub league: Option<League>,
    pub team: Option<Team>,
}

impl Store for State {
    type Action = Action;
    type Input = Request;

    fn new() -> Self {
        State {
            user: None,
            league: None,
            team: None,
        }
    }

    fn handle_input(&self, link: AgentLink<StoreWrapper<Self>>, msg: Self::Input) {
        match msg {
            Request::UpdateUser(user) => {
                link.send_message(Action::SetUser(user));
            }
            Request::UpdateLeague(league) => {
                link.send_message(Action::SetLeague(league));
            }
            Request::UpdateTeam(team) => {
                link.send_message(Action::SetTeam(team));
            }
        }
    }

    fn reduce(&mut self, msg: Self::Action) {
        match msg {
            Action::SetUser(user) => {
                self.user = user;
            }
            Action::SetLeague(league) => {
                self.league = league;
            }
            Action::SetTeam(team) => {
                self.team = team;
            }
        }
    }
}
