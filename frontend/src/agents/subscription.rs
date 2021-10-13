use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use yew::worker::HandlerId;
use yew::worker::Job;
use yew::{
    prelude::*,
    worker::{Agent, AgentLink},
};

use crate::agents::socket::{Socket, Response};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GqlPayload {
    pub query: String,
    pub variables: Option<String>,
    pub operation_name: Option<String>,
}

/*#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GqlMessage {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub _type: String,
    pub payload: Option<GqlPayload>,
    pub id: Option<String>,
}*/

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum GqlMessage {
    #[serde(rename = "connection_init")]
    ConnectionInit,
    #[serde(rename = "connection_ack")]
    ConnectionAck,
    #[serde(rename = "start")]
    Start {
        id: Option<String>,
        payload: GqlPayload,
    },
    #[serde(rename = "data")]
    Data {
        id: Option<String>,
        payload: serde_json::Value,
    },
}

pub enum Msg {
    WsConnected,
    WsDisconnected,
    WsMessage(String),
    //GqlConnected,
}

pub struct Subscription {
    link: AgentLink<Self>,
    ws_agent: Box<dyn Bridge<Socket>>,
    subscribers: HashSet<HandlerId>,
    //connected: bool
}

impl Agent for Subscription {
    type Reach = Job<Self>;
    type Message = Msg;
    type Input = String;
    type Output = GqlMessage;

    fn create(link: AgentLink<Self>) -> Self {
        let cb = link.callback(|ws_msg| match ws_msg {
            Response::Ready => Msg::WsConnected,
            Response::Disconnected => Msg::WsDisconnected,
            Response::Message(msg) => Msg::WsMessage(msg),
        });
        let ws_agent = Socket::bridge(cb);

        Self {
            link,
            ws_agent,
            subscribers: HashSet::new(),
            //connected: false,
        }
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            Msg::WsConnected => {
                log::info!("WebSocket connected, sending GQL_CONNECTION_INIT");
                let connection_msg = GqlMessage::ConnectionInit;
                /*let connection_msg = GqlMessage { 
                    _type: "connection_init".to_string(),
                    id: Some("20320394023940923".to_string()),
                    payload: None
                };*/
                self.send(serde_json::to_string(&connection_msg).unwrap());
            }
            Msg::WsDisconnected => {
                log::info!("Subscription Agent: Disconnected, I should do something here...");
                //self.respond_to_all(RoomMessage::Disconnected);   
            }
            Msg::WsMessage(msg) => {                
                let gql_message: GqlMessage = serde_json::from_str(&msg).unwrap();
                self.respond_to_all(gql_message);
            },
            /*Msg::GqlConnected => {
                log::info!("GraphQL subscription socket initialized");
                self.connected = true;
            }*/
        }
    }

    fn handle_input(&mut self, msg: Self::Input, _id: worker::HandlerId) {
        self.send(msg);
    }

    fn connected(&mut self, id: worker::HandlerId) {
        self.subscribers.insert(id);
    }

    fn disconnected(&mut self, id: worker::HandlerId) {
        self.subscribers.remove(&id);
    }

    fn destroy(&mut self) {
        log::info!("room agent destroyed")
    }
}

impl Subscription {
    fn send(&mut self, msg: String) {
        self.ws_agent.send(msg);
    }

    fn respond(&self, sub: HandlerId, response: GqlMessage) {
        self.link.respond(sub, response);
    }

    fn respond_to_all(&self, response: GqlMessage) {
        for sub in self.subscribers.iter() {
            self.respond(*sub, response.clone())
        }
    }
}