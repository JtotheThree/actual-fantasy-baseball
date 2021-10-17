use graphql_client::{GraphQLQuery, QueryBody};
use yew::prelude::*;
use yew::services::fetch::FetchTask;

use crate::components::dropdown::{Dropdown, DropdownItem};
use crate::error::Error;
use crate::services::GraphQL;
use crate::types::*;

pub struct LeagueDropdown {
    gql: GraphQL,
    link: ComponentLink<Self>,
    task: Option<FetchTask>,
    callback: Callback<Result<my_leagues::ResponseData, Error>>,
    selected_league: Option<League>,
    joined_leagues: Vec<League>,
}

pub enum Msg {
    LeaguesResponse(Result<my_leagues::ResponseData, Error>)
}

impl Component for LeagueDropdown {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        LeagueDropdown {
            gql: GraphQL::new(),
            callback: link.callback(Msg::LeaguesResponse),
            selected_league: None,
            joined_leagues: Vec::new(),
            task: None,
            link,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            let body = MyLeagues::build_query(my_leagues::Variables);
            self.task = Some(
                self.gql.post::<QueryBody<my_leagues::Variables>, my_leagues::ResponseData>(
                    body, self.callback.clone()
                )
            );
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::LeaguesResponse(Ok(resp)) => {
                self.selected_league = if let Some(league) = resp.me.selected_league {
                    Some(League {
                        id: league.clone().id,
                        name: league.clone().name,
                    })
                } else {
                    None
                };
                
                let joined_leagues: Vec<League> = resp.me.joined_leagues.into_iter()
                    .map(|league| League {
                        id: league.id,
                        name: league.name,
                    })
                    .collect();

                self.joined_leagues = joined_leagues;
            }
            Msg::LeaguesResponse(Err(err)) => {
                error!("{:?}", err);
            }
        }

        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let selected_league: Option<League> = if let Some(selected_league) = self.selected_league.clone() {
            Some(selected_league)
        } else if self.joined_leagues.len() > 0 {
            Some(self.joined_leagues[0].clone())
        } else {
            None
        };

        html! {
            { 
                if self.joined_leagues.len() > 0 {
                    if let Some(selected_league) = selected_league {
                        html! {         
                            <Dropdown
                                class="md:px-6 px-6"
                                main_content=html!{ { selected_league.name.clone() } }>
                                { for self.joined_leagues.iter().map(|league| {
                                    html! {
                                        <DropdownItem 
                                            class="block p-4 text-lg font-normal font-bold hover:text-red-800"
                                            >
                                            { league.name.clone() }
                                        </DropdownItem>
                                    }
                                })}
                            </Dropdown>
                        }
                    } else {
                        html!{}
                    }
                } else {
                    html! {
                        <a class="px-24 text-lg text-center font-normal font-bold">
                            { "Create or join a league" }
                        </a>
                    }
                }
            }            
        }
    }
}

