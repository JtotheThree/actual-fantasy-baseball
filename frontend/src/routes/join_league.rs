use graphql_client::{GraphQLQuery, QueryBody};
use yew::prelude::*;
use yew::services::fetch::FetchTask;
use yew_router::{agent::RouteRequest::ChangeRoute, prelude::*};
use yewtil::store::{Bridgeable, ReadOnly, StoreWrapper};

use crate::agents::{State, state::Request};
use crate::components::list_errors::ListErrors;
use crate::components::{card::Card, button::Button};
use crate::error::Error;
use crate::services::GraphQL;
use crate::types::*;

use crate::routes::AppRoute;

pub struct JoinLeague {
    gql: GraphQL,
    public_leagues: Vec<League>,
    task: Option<FetchTask>,
    public_leagues_callback: Callback<Result<public_leagues::ResponseData, Error>>,
    join_league_callback: Callback<Result<join_league::ResponseData, Error>>,
    set_league_callback: Callback<Result<select_league::ResponseData, Error>>,
    state: Box<dyn Bridge<StoreWrapper<State>>>,
    router_agent: Box<dyn Bridge<RouteAgent>>,
    link: ComponentLink<Self>,
}

pub enum Msg {
    Response(Result<public_leagues::ResponseData, Error>),
    JoinLeagueResponse(Result<join_league::ResponseData, Error>),
    SetLeagueResponse(Result<select_league::ResponseData, Error>),
    Join(String),
    State(ReadOnly<State>),
    Ignore,
}


impl Component for JoinLeague {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(Msg::State);

        JoinLeague {
            gql: GraphQL::new(),
            public_leagues: Vec::new(),
            task: None,
            public_leagues_callback: link.callback(Msg::Response),
            join_league_callback: link.callback(Msg::JoinLeagueResponse),
            set_league_callback: link.callback(Msg::SetLeagueResponse),
            state: State::bridge(callback),
            router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
            link,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            let body = PublicLeagues::build_query(public_leagues::Variables);
            self.task = Some(
                self.gql.post::<QueryBody<public_leagues::Variables>, public_leagues::ResponseData>(
                    body, self.public_leagues_callback.clone()
                )
            );
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Response(Ok(resp)) => {
                for league in resp.leagues.into_iter() {
                    self.public_leagues.push(
                        League {
                            id: league.id.clone(),
                            name: league.name.clone(),
                            description: league.description.clone(),
                            max_players: league.max_players,
                            managers_count: league.managers_count,
                            state: None, //league.state,
                            owner: Some(Manager {
                                id: league.owner.id.clone(),
                                username: league.owner.username.clone(),
                            }),
                            managers: None,
                        }
                    )
                }
            }
            Msg::Response(Err(err)) => {
                error!("{:?}", err);
            }
            Msg::Join(id) => {
                let body = crate::types::JoinLeague::build_query(join_league::Variables{id});
                self.task = Some(
                    self.gql.post::<QueryBody<join_league::Variables>, join_league::ResponseData>(
                        body, self.join_league_callback.clone()
                    )
                );
            }
            Msg::JoinLeagueResponse(Ok(resp)) => {
                let body = SelectLeague::build_query(select_league::Variables{id: resp.join_league.id.clone()});
                self.task = Some(
                    self.gql.post::<QueryBody<select_league::Variables>, select_league::ResponseData>(
                        body, self.set_league_callback.clone()
                    )
                );

                self.router_agent.send(ChangeRoute(AppRoute::League(resp.join_league.id.clone()).into()));
            }
            Msg::JoinLeagueResponse(Err(err)) => {
                error!("{:?}", err);
            }
            Msg::SetLeagueResponse(Ok(resp)) => {
                self.state.send(Request::UpdateLeague(Some(League {
                    id: resp.select_league.id.clone(),
                    name: resp.select_league.name.clone(),
                    ..Default::default()
                })));

                if let Some(team) = resp.select_league.team {
                    self.state.send(Request::UpdateTeam(Some(Team {
                        id: team.id,
                        name: team.name,
                        league_id: resp.select_league.id.clone(),
                })))};
            }
            Msg::SetLeagueResponse(Err(err)) => {
                error!("{:?}", err);
            }
            Msg::State(state) => {         
            }

            Msg::Ignore => {}
        }
        
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let callback_join = self.link.callback(|id| Msg::Join(id));

        html!{
            <div class="flex flex-col md:w-1/3 mx-auto p-8 space-y-8">
            { for self.public_leagues.iter().map(|league| {
                html!{
                    <Card 
                        header=html!{ league.name.clone() }
                        body=html!{ 
                            <>{" Players: "}{"1/"}{ league.max_players }<br/>
                              {" Owner: "}{ league.owner.as_ref().unwrap().username.clone() }
                              /*{ "State: "}{ &league.state }*/
                            </>
                        }
                        footer=html!{
                            <Button
                                data=league.id.clone()
                                onclick=&callback_join
                                text="Join".to_string()
                            />
                        }
                    />
                }
            })}
            </div>
        }
    }
}
