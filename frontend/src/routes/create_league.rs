use graphql_client::{GraphQLQuery, QueryBody};
use yew::prelude::*;
use yew::services::fetch::FetchTask;
use yew_router::{agent::RouteRequest::ChangeRoute, prelude::*};

use crate::components::list_errors::ListErrors;
use crate::components::card::Card;
use crate::error::Error;
use crate::services::GraphQL;
use crate::types::*;

use crate::routes::AppRoute;

#[derive(Default)]
struct CreateLeagueFormData {
    name: String,
    description: String,
    public: bool,
    password: Option<String>,
    max_players: i64,
    manual_state: bool,
}

pub struct CreateLeagueForm {
    gql: GraphQL,
    error: Option<crate::error::Error>,
    request: CreateLeagueFormData,
    response: Callback<Result<create_league::ResponseData, Error>>,
    task: Option<FetchTask>,
    router_agent: Box<dyn Bridge<RouteAgent>>,
    link: ComponentLink<Self>,
}

pub enum Msg {
    Request,
    Response(Result<create_league::ResponseData, Error>),
    Ignore,
    UpdateName(String),
    UpdateDescription(String),
    UpdatePublic,
    UpdatePassword(String),
    UpdateMaxPlayers(String),
    UpdateManualState,
}

impl Component for CreateLeagueForm {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        CreateLeagueForm {
            gql: GraphQL::new(),
            error: None,
            request: CreateLeagueFormData::default(),
            response: link.callback(Msg::Response),
            router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
            task: None,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Request => {
                let body = CreateLeague::build_query(create_league::Variables{
                    name: self.request.name.clone(),
                    description: self.request.description.clone(),
                    public: self.request.public,
                    max_players: self.request.max_players,
                    password: self.request.password.clone(),
                    manual_state: self.request.manual_state,
                });

                self.task = Some(
                    self.gql.post::<QueryBody<create_league::Variables>, create_league::ResponseData>(
                        body, self.response.clone()
                    )
                );

                return false;
            }
            Msg::Response(Ok(response)) => {
                // TODO: Do it.
                let id = response.create_league.id;

                self.router_agent.send(ChangeRoute(AppRoute::League(id).into()));
            }
            Msg::Response(Err(err)) => {
                //error!{"{:?}", err};
                self.error = Some(err);
                self.task = None;
            }
            Msg::UpdateName(name) => {
                self.request.name = name;
            }
            Msg::UpdateDescription(desc) => {
                self.request.description = desc;
            }
            Msg::UpdatePublic => {
                self.request.public = !self.request.public;
            }
            Msg::UpdatePassword(password) => {
                self.request.password = Some(password);
            }
            Msg::UpdateMaxPlayers(max_players) => {
                self.request.max_players = max_players.parse::<i64>().unwrap();
            }
            Msg::UpdateManualState => {
                self.request.manual_state = !self.request.manual_state;
            }
            Msg::Ignore => {
                println!("Ignore me");
            }
        }

        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let onsubmit = self.link.callback(|ev: FocusEvent| {
            ev.prevent_default(); /* Prevent event propagation */
            Msg::Request
        });
        let oninput_name = self
            .link
            .callback(|ev: InputData| Msg::UpdateName(ev.value));
        let oninput_desc = self
            .link
            .callback(|ev: InputData| Msg::UpdateDescription(ev.value));
        let oninput_max_players = self
            .link
            .callback(|ev: InputData| Msg::UpdateMaxPlayers(ev.value));
        let onclick_public = self
            .link
            .callback(|_| Msg::UpdatePublic);
        let oninput_password = self
            .link
            .callback(|ev: InputData| Msg::UpdatePassword(ev.value));
        let onclick_manual_state = self
            .link
            .callback(|_| Msg::UpdateManualState);

        html! {
            <>
            // LEAGUE FORM
            <div class="w-full justify-self-center md:w-1/2 mx-auto flex flex-col flex-wrap justify-between p-16">
            <h2 class="h-5 text-center border-b-2 border-gray-700 tracking-wid px-4 text-3xl font-title"><span class="bg-paper px-8">{ "Create a League" }</span></h2>
            //<h1 class="text-3xl tracking-wid text-center px-4 font-title">{ "Login" }</h1>
            //<div class="border-b-2 border-black"></div>
            <form class="md:w-1/2-screen m-0 p-12 w-full tw-h-full shadow-md" onsubmit=onsubmit>
                <ListErrors error=self.error.clone() />
                <div class="flex flex-col py-3">
                    <input
                        class="input"
                        type="text"
                        value=self.request.name.clone()
                        oninput=oninput_name
                    />
                    <label class="px-4 pb-2 font-semibold">
                        { "League Name" }
                    </label>
         
                    <input
                        class="input"
                        type="text"
                        value=self.request.description.clone()
                        oninput=oninput_desc
                    />
                    <label class="px-4 pb-2 font-semibold">
                        { "League Description" }
                    </label>

                    <input
                        class="input"
                        type="number"
                        value=self.request.max_players.to_string()
                        oninput=oninput_max_players
                    />
                    <label class="px-4 pb-2 font-semibold">
                        { "Max Players" }
                    </label>

                    <label class="inline-flex items-center mt-3 m-8">
                        <input
                            type="checkbox"
                            class="form-checkbox h-5 w-5 text-red-700"
                            checked=self.request.public
                            onclick=onclick_public />
                        <span class="font-semibold ml-2">{ "Public "}</span>
                    </label>

                    <label class="inline-flex items-center mt-3 m-8">
                        <input
                            type="checkbox"
                            class="form-checkbox h-5 w-5 text-red-700"
                            checked=self.request.manual_state
                            onclick=onclick_manual_state />
                        <span class="font-semibold ml-2">{ " Change State Manually "}</span>
                    </label>



                    /*<div class="flex flex-row">
                    <input
                        type="checkbox"
                        checked=self.request.public
                        onclick=onclick_public
                    />
                    <label class="font-semibold mx-4 text-red-700">
                        { "Public" }
                    </label>
                    </div>*/


                    <input
                        class="input"
                        value=self.request.password.clone()
                        oninput=oninput_password
                    />
                    <label class="px-4 pb-2 font-semibold">
                        { "Password" }
                    </label>
                    
                </div>
                <div class="mt-2">
                    <button
                        class="btn p-3 my-2 bg-gray-700 text-paper rounded-sm border-b-4 border-paper w-full font-bold hover:bg-red-800"
                        type="submit"
                        disabled=false>
                        { "Create" }
                    </button>
                </div>
            </form>
            </div>
         
            </>
        }
    }
}
