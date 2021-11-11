use graphql_client::{GraphQLQuery, QueryBody};
use yew::prelude::*;
use yew::services::fetch::FetchTask;
use yew_router::{agent::RouteRequest::ChangeRoute, prelude::*};

use crate::components::list_errors::ListErrors;
use crate::error::Error;
use crate::services::GraphQL;
use crate::types::*;

use crate::routes::AppRoute;

#[derive(Default)]
struct CreateTeamFormData {
    name: String,
}

pub struct CreateTeamForm {
    gql: GraphQL,
    error: Option<crate::error::Error>,
    request: CreateTeamFormData,
    response: Callback<Result<create_team::ResponseData, Error>>,
    task: Option<FetchTask>,
    router_agent: Box<dyn Bridge<RouteAgent>>,
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(PartialEq, Properties, Clone)]
pub struct Props {
    pub league_id: String,
}

pub enum Msg {
    Request,
    Response(Result<create_team::ResponseData, Error>),
    Ignore,
    UpdateName(String),
}

impl Component for CreateTeamForm {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        CreateTeamForm {
            gql: GraphQL::new(),
            error: None,
            request: CreateTeamFormData::default(),
            response: link.callback(Msg::Response),
            router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
            task: None,
            props,
            link,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Request => {
                let body = CreateTeam::build_query(create_team::Variables{
                    name: self.request.name.clone(),
                    league_id: self.props.league_id.clone(),
                });

                self.task = Some(
                    self.gql.post::<QueryBody<create_team::Variables>, create_team::ResponseData>(
                        body, self.response.clone()
                    )
                );

                return false;
            }
            Msg::Response(Ok(response)) => {
                // TODO: Do it.
                let id = response.create_team.id;

                self.router_agent.send(ChangeRoute(AppRoute::Team(id).into()));
            }
            Msg::Response(Err(err)) => {
                //error!{"{:?}", err};
                self.error = Some(err);
                self.task = None;
            }
            Msg::UpdateName(name) => {
                self.request.name = name;
            }
            Msg::Ignore => {
                println!("Ignore me");
            }
        }

        true
    }

    fn view(&self) -> Html {
        let onsubmit = self.link.callback(|ev: FocusEvent| {
            ev.prevent_default(); /* Prevent event propagation */
            Msg::Request
        });
        let oninput_name = self
            .link
            .callback(|ev: InputData| Msg::UpdateName(ev.value));

        html! {
            <>
            // TEAM FORM
            <div class="w-full justify-self-center md:w-1/2 mx-auto flex flex-col flex-wrap justify-between p-16">
            <h2 class="h-5 text-center border-b-2 border-gray-700 tracking-wid px-4 text-3xl font-title"><span class="bg-paper px-8">{ "Create a Team" }</span></h2>
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
                        { "Team Name" }
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
