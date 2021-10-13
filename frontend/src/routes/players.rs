use graphql_client::{GraphQLQuery, Response};
use std::fmt::Debug;
use wasm_bindgen::{prelude::*};
use wasm_bindgen_futures::{spawn_local};
use yew::prelude::*;
use yew::format::Json;
use std::error::Error;

use crate::agents::subscription::{GqlMessage, GqlPayload, Subscription};
use crate::util::{common::gql_uri};


#[derive(Debug, Clone, PartialEq)]
pub struct FetchError {
    err: JsValue,
}

impl From<JsValue> for FetchError {
    fn from(value: JsValue) -> Self {
        Self { err: value }
    }
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/all_players.graphql",
    response_derives = "Debug"
)]
struct FetchAll;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/player_created.graphql",
    response_derives = "Debug"
)]
struct PlayerCreated;

async fn fetch_players() -> Result<Vec<fetch_all::FetchAllPlayers>, Box<dyn Error>> {
    let request_body = FetchAll::build_query(fetch_all::Variables);
    let client = reqwest::Client::new();
    let res = client.post(gql_uri()).json(&request_body).send().await?;

    let resp_data: Response<fetch_all::ResponseData> = res.json().await?;
    let data = resp_data.data.unwrap();

    Ok(data.players)
}


#[derive(Debug)]
pub enum Msg {
    GqlReceived(GqlMessage),
    GqlConnected,
    UpdateList(Vec<fetch_all::FetchAllPlayers>),
}


pub struct Players {
    link: ComponentLink<Self>,
    list: Vec<fetch_all::FetchAllPlayers>,
    subscription: Box<dyn Bridge<Subscription>>,
    connected: bool
}


impl Component for Players {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let subscription = Subscription::bridge(link.callback(|msg| Msg::GqlReceived(msg)));

        Self {
            link,
            list: Vec::new(),
            subscription,
            connected: false
        }
    }

    fn rendered(&mut self, first_render: bool) {
        let link = self.link.clone();
        if first_render {
            spawn_local(async move {
                let res = fetch_players().await;
                link.send_message(Msg::UpdateList(res.unwrap()));
            });
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GqlReceived(gql_message) => {
                let mut render = false;
                match gql_message {
                    GqlMessage::ConnectionAck => {
                        self.connected = true;
                        self.link.send_message(Msg::GqlConnected);
                    }
                    GqlMessage::Data{id, payload} => {
                        let link = self.link.clone();
                        spawn_local(async move {
                            let res = fetch_players().await;
                            link.send_message(Msg::UpdateList(res.unwrap()));
                        });
                        render = true;
                    }
                    _ => {
                        log::info!("Uknown message : {:?}", gql_message);
                    }  
                }
                render
            }
            Msg::GqlConnected => {
                let subsciption_request = PlayerCreated::build_query(player_created::Variables);
                log::info!("Sending PlayerCreated subscription request");

                let payload = GqlPayload {
                    query: subsciption_request.query.to_string(),
                    operation_name: Some(subsciption_request.operation_name.to_string()),
                    variables: None,
                };

                let gql_message = GqlMessage::Start {
                    id: Some("36a45153-7633-405c-957c-4fb1c66908b3".to_string()),
                    payload: payload,
                };

                log::info!("{:?}", serde_json::to_string(&gql_message));
                self.subscription.send(serde_json::to_string(&gql_message).unwrap());
                false
            }
            Msg::UpdateList(res) => {
                self.list = res;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let players = self.list.iter().map(|player| {
            html! {
                <div>
                    <h3><strong>{ &player.name }</strong></h3>
                    <p>{ "Age:" } { &player.age }</p>
                    <p>{ "Image:" } { &player.image_url }</p>
                </div>
            }
        });

        html! {
            <>
                <h1>{ "All Players" }</h1>
                { for players }
            </>
        }
    }
}