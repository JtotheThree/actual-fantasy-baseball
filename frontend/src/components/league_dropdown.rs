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

pub struct LeagueDropdown {
    gql: GraphQL,
    link: ComponentLink<Self>,
    task: Option<FetchTask>,
    state: Box<dyn Bridge<StoreWrapper<State>>>,
    selected_league: Option<League>,
    joined_leagues: Vec<League>,
    my_leagues_callback: Callback<Result<my_leagues::ResponseData, Error>>,
    set_league_callback: Callback<Result<select_league::ResponseData, Error>>,
}

pub enum Msg {
    LeaguesResponse(Result<my_leagues::ResponseData, Error>),
    SetLeagueResponse(Result<select_league::ResponseData, Error>),
    SetSelectedLeague(String),
    StateMsg(ReadOnly<State>),
}

impl Component for LeagueDropdown {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(Msg::StateMsg);

        LeagueDropdown {
            gql: GraphQL::new(),
            my_leagues_callback: link.callback(Msg::LeaguesResponse),
            set_league_callback: link.callback(Msg::SetLeagueResponse),
            selected_league: None,
            joined_leagues: Vec::new(),
            state: State::bridge(callback),
            task: None,
            link,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            let body = MyLeagues::build_query(my_leagues::Variables);
            self.task = Some(
                self.gql.post::<QueryBody<my_leagues::Variables>, my_leagues::ResponseData>(
                    body, self.my_leagues_callback.clone()
                )
            );
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::LeaguesResponse(Ok(resp)) => {
                if let Some(league) = resp.me.selected_league {
                    self.state.send(Request::UpdateLeague(Some(League {
                        id: league.id.clone(),
                        name: league.name,
                        ..Default::default()
                    })));
                } else {
                    error!("Msg::LeagueResponse: No league is selected?");
                    // TODO: Do I need to set state league to none here??
                };
                
                let joined_leagues: Vec<League> = resp.me.joined_leagues.into_iter()
                    .map(|league| League {
                        id: league.id,
                        name: league.name,
                        ..Default::default()
                    })
                    .collect();

                self.joined_leagues = joined_leagues;

                true
            }
            Msg::LeaguesResponse(Err(err)) => {
                error!("{:?}", err);

                false
            }
            Msg::SetSelectedLeague(id) => {
                info!("Selected League: {:?}", id);
                let body = SelectLeague::build_query(select_league::Variables{id});
                self.task = Some(
                    self.gql.post::<QueryBody<select_league::Variables>, select_league::ResponseData>(
                        body, self.set_league_callback.clone()
                    )
                );

                false
            }
            Msg::SetLeagueResponse(Ok(resp)) => {
                self.state.send(Request::UpdateLeague(Some(League {
                    id: resp.select_league.id.clone(),
                    name: resp.select_league.name.clone(),
                    ..Default::default()
                })));

                if let Some(team) = resp.select_league.team {
                    info!("Selected League Team: {:?}", team);
                    self.state.send(Request::UpdateTeam(Some(Team {
                        id: team.id,
                        name: team.name,
                        league_id: resp.select_league.id.clone(),
                    })));
                } else {
                    info!("Selected League Team: No team for selected league");
                    self.state.send(Request::UpdateTeam(None));
                };

                true
            }
            Msg::SetLeagueResponse(Err(err)) => {
                error!("{:?}", err);

                false
            }
            Msg::StateMsg(state) => {
                let state = state.borrow();

                if state.league != self.selected_league {
                    self.selected_league = state.league.clone();
                    true
                } else {
                    return false;
                }
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        // Selected League
        let selected_league: Option<League> = if let Some(selected_league) = self.selected_league.clone() {
            Some(selected_league)
        } else if self.joined_leagues.len() > 0 {
            Some(self.joined_leagues[0].clone())
        } else {
            None
        };

        // Callbacks
        let callback_league = self.link.callback(|id| Msg::SetSelectedLeague(id));

        html! {
            { 
                if self.joined_leagues.len() > 0 {
                    if let Some(selected_league) = selected_league {
                        html! {         
                            <Dropdown
                                class="md:px-6 px-6"
                                main_content=html!{
                                    <RouterAnchor<AppRoute> route=AppRoute::League(selected_league.id.clone())>
                                    { selected_league.name.clone() }
                                    </RouterAnchor<AppRoute>>
                                }
                            >
                                { for self.joined_leagues.iter().map(|league| {
                                    html! {
                                        <DropdownItem 
                                            class="block p-4 text-lg font-normal font-bold hover:text-red-800"
                                            data=league.id.clone()
                                            onclick=&callback_league
                                        >
                                        <RouterAnchor<AppRoute> route=AppRoute::League(league.id.clone())>
                                            { league.name.clone() }
                                        </RouterAnchor<AppRoute>>
                                        </DropdownItem>
                                    }
                                })}

                                <DropdownItem 
                                    class="block p-4 text-lg font-normal font-bold hover:text-red-800 border-t-2"
                                    data=""
                                >
                                <RouterAnchor<AppRoute> route=AppRoute::CreateLeagueForm>
                                    { "Create a League" }
                                </RouterAnchor<AppRoute>>
                                </DropdownItem>

                                <DropdownItem 
                                    class="block p-4 text-lg font-normal font-bold hover:text-red-800"
                                    data=""
                                >
                                <RouterAnchor<AppRoute> route=AppRoute::JoinLeague>
                                    { "Join a League" }
                                </RouterAnchor<AppRoute>>
                                </DropdownItem>

                            </Dropdown>
                        }
                    } else {
                        html!{}
                    }
                } else {
                    html! {
                        <>
                        <span class="px-24 text-lg text-center font-normal font-bold">
                        <RouterAnchor<AppRoute> 
                            classes="underline"
                            route=AppRoute::CreateLeagueForm>
                            { "Create" }
                        </RouterAnchor<AppRoute>>
                        { " or "}
                        <RouterAnchor<AppRoute>
                            classes="underline"
                            route=AppRoute::JoinLeague>
                            { "Join" }
                        </RouterAnchor<AppRoute>>
                        { " a league" }
                        </span>
                        </>
                    }
                }
            }            
        }
    }
}

