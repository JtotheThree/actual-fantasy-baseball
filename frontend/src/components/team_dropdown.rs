use graphql_client::{GraphQLQuery, QueryBody};
use yew::prelude::*;
use yew_router::prelude::*;
use yew::services::fetch::FetchTask;
use yewtil::store::{Bridgeable, ReadOnly, StoreWrapper};

use crate::agents::{State, state::Request};
use crate::components::dropdown::{Dropdown, DropdownItem};
use crate::error::Error;
use crate::routes::AppRoute;
use crate::services::GraphQL;
use crate::types::*;

pub struct TeamDropdown {
    gql: GraphQL,
    link: ComponentLink<Self>,
    task: Option<FetchTask>,
    state: Box<dyn Bridge<StoreWrapper<State>>>,
    league: Option<League>,
    league_team: Option<Team>,
    teams: Vec<Team>,
    my_teams_callback: Callback<Result<my_teams::ResponseData, Error>>,
    set_league_callback: Callback<Result<select_league::ResponseData, Error>>,
}

pub enum Msg {
    MyTeamsResponse(Result<my_teams::ResponseData, Error>),
    SetTeam(String),
    SetLeagueResponse(Result<select_league::ResponseData, Error>),
    StateMsg(ReadOnly<State>),
}

impl Component for TeamDropdown {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(Msg::StateMsg);

        TeamDropdown {
            gql: GraphQL::new(),
            my_teams_callback: link.callback(Msg::MyTeamsResponse),
            set_league_callback: link.callback(Msg::SetLeagueResponse),
            league: None,
            league_team: None,
            teams: Vec::new(),
            state: State::bridge(callback),
            task: None,
            link,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            let body = MyTeams::build_query(my_teams::Variables);
            self.task = Some(
                self.gql.post::<QueryBody<my_teams::Variables>, my_teams::ResponseData>(
                    body, self.my_teams_callback.clone()
                )
            );
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::MyTeamsResponse(Ok(resp)) => {
                if let Some(league) = resp.me.selected_league {
                    if let Some(team) = league.team {
                        self.state.send(Request::UpdateTeam(Some(Team {
                            id: team.id,
                            name: team.name,
                            league_id: league.id,
                        })));
                    }
                } else {
                    error!("Msg::TeamResponse: No team is selected?");
                    // TODO: Do I need to set state team to none here??
                };

                let mut teams: Vec<Team> = Vec::new();

                for joined_league in resp.me.joined_leagues.into_iter() {
                    if let Some(team) = joined_league.team {
                        teams.push(Team {
                            id: team.id.clone(),
                            name: team.name.clone(),
                            league_id: joined_league.id,
                        });
                    }
                }
                
                self.teams = teams;

                true
            }
            Msg::MyTeamsResponse(Err(err)) => {
                error!("{:?}", err);

                false
            }
            Msg::SetTeam(id) => {
                // NOTE: Update state here, which should trigger the league update as well 
                //
                let mut selected_team: Option<Team> = None;

                for team in self.teams.clone().into_iter() {
                    if id == team.id {
                        selected_team = Some(team);
                    }
                }

                // Now tell state to update team
                //
                if let Some(team) = selected_team {
                    self.state.send(Request::UpdateTeam(Some(Team {
                        id: team.id.clone(),
                        name: team.name.clone(),
                        league_id: team.league_id.clone(),
                    })));

                    let body = SelectLeague::build_query(select_league::Variables{id: team.league_id.clone()});
                         self.task = Some(
                         self.gql.post::<QueryBody<select_league::Variables>, select_league::ResponseData>(
                            body, self.set_league_callback.clone()
                        )
                    );
                }

                false
            }
            Msg::SetLeagueResponse(Ok(resp)) => {
                self.state.send(Request::UpdateLeague(Some(League {
                    id: resp.select_league.id.clone(),
                    name: resp.select_league.name.clone(),
                    ..Default::default()
                })));

                true
            }
            Msg::SetLeagueResponse(Err(err)) => {
                error!("{:?}", err);
                false
            }
            Msg::StateMsg(state) => {
                let state = state.borrow();
                
                info!("Team Drooooooop league team: {:?}", state.team);

                if state.league != self.league {
                    self.league = state.league.clone();
                }

                if state.team != self.league_team {
                    self.league_team = state.team.clone();
                } 

                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        // Selected Team
        let league_team: Option<Team> = if let Some(league_team) = self.league_team.clone() {
            info!("{:?}", league_team);
            Some(league_team)
        //} else if self.teams.len() > 0 {
          //  Some(self.teams[0].clone())
        } else {
            None
        };

        info!("{:?}", league_team);

        // Callbacks
        let callback_team = self.link.callback(|id| Msg::SetTeam(id));

        html! {
            { 
                if self.teams.len() > 0 {

                    // FIXME: Show full drop down menu even if there's no team for the selected
                    // league, if teams > 0, show full menu, top view is Create Team if current
                    // league team is none

                    if let Some(selected_team) = league_team {
                        html! {
                            <>      
                            {"-"}   
                            <Dropdown
                                class="md:px-6 px-6"
                                main_content=html!{
                                    <RouterAnchor<AppRoute> route=AppRoute::Team(selected_team.id.clone())>
                                    { selected_team.name.clone() }
                                    </RouterAnchor<AppRoute>>
                                }
                            >
                                { for self.teams.iter().map(|team| {
                                    html! {
                                        <DropdownItem 
                                            class="block p-4 text-lg font-normal font-bold hover:text-red-800"
                                            data=team.id.clone()
                                            onclick=&callback_team
                                        >
                                        <RouterAnchor<AppRoute> route=AppRoute::Team(team.id.clone())>
                                            { team.name.clone() }
                                        </RouterAnchor<AppRoute>>
                                        </DropdownItem>
                                    }
                                })}
                            </Dropdown>
                            </>
                        }
                    } else {
                        if let Some(league) = &self.league {
                            html! {
                                <>      
                                {"-"}
                                <div class="px-5">
                                    <RouterAnchor<AppRoute> route=AppRoute::CreateTeamForm(league.id.clone())>
                                        { "Create a team" }
                                    </RouterAnchor<AppRoute>>
                                </div>
                                </>
                            }
                        } else {
                            html! {}
                        }
                    }
                } else {
                    if let Some(league) = &self.league {
                        html! {
                            <>      
                            {"-"}
                            <div class="px-5">
                                <RouterAnchor<AppRoute> route=AppRoute::CreateTeamForm(league.id.clone())>
                                    { "Create a team" }
                                </RouterAnchor<AppRoute>>
                            </div>
                            </>
                        }
                    } else {
                        html! {}
                    }
                }
            }            
        }
    }
}

