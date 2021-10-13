use std::collections::HashSet;

use anyhow::Error;
use serde::{Deserialize, Serialize};
use yew::{
    prelude::*,
    services::websocket::{WebSocketService, WebSocketStatus, WebSocketTask},
    worker::{Agent, AgentLink, Context, HandlerId},
};


pub enum Msg {
    Connect,
    Received(Result<String, Error>),
    Connected,
    Disconnected,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Response {
    Ready,
    Disconnected,
    Message(String),
}

pub struct Socket {
    link: AgentLink<Self>,
    ws: Option<WebSocketTask>,
    subscribers: HashSet<HandlerId>,
    connected: bool,
}

impl Agent for Socket {
    type Reach = Context<Self>;
    type Message = Msg;
    type Input = String;
    type Output = Response;

    fn create(link: AgentLink<Self>) -> Self {
        link.send_message(Msg::Connect);

        Self {
            link,
            ws: None,
            subscribers: HashSet::new(),
            connected: false,
        }
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            Msg::Connect => {
                self.connect();
            }
            Msg::Connected => {
                log::info!("socket connected");

                self.connected = true;

                self.respond_to_all(Response::Ready);
            }
            Msg::Disconnected => {
                log::info!("socket disconnected");

                self.connected = false;
                self.ws = None;

                self.respond_to_all(Response::Disconnected);
            }
            Msg::Received(res) => match res {
                Ok(msg) => {
                    log::info!("socket message: {:?}", msg);

                    self.respond_to_all(Response::Message(msg.clone()));
                    //let gql_message: GqlMessage = serde_json::from_str(&msg.clone()).unwrap();

                    //log::info!("{:?}", gql_message);
                    //self.respond_to_all(Response::Message(gql_message));
                }
                Err(err) => {
                    log::error!("socket error: {:?}", err);
                }
            }
        }
    }

    fn handle_input(&mut self, msg: Self::Input, _id: worker::HandlerId) {
        if let Some(ws) = self.ws.as_mut() {
            log::info!("WebSocket sending: {:?}", msg);
            ws.send(Ok(msg));
        }
    }
    
    fn connected(&mut self, id: worker::HandlerId) {
        self.subscribers.insert(id);
    
        if self.connected {
            self.respond(id, Response::Ready);
        }
    }

    fn disconnected(&mut self, id: worker::HandlerId) {
        self.subscribers.remove(&id);
    }

    fn destroy(&mut self) {
        log::info!("socket agent destroyed");
    }
}

impl Socket {
    fn respond(&self, sub: HandlerId, response: Response) {
        self.link.respond(sub, response);
    }

    fn respond_to_all(&self, response: Response) {
        for sub in self.subscribers.iter() {
            self.respond(*sub, response.clone())
        }
    }

    fn connect(&mut self) {
        let ws_msg_callback = self.link.callback(|data| Msg::Received(data));

        let ws_notification_callback = self.link.callback(|status| match status {
            WebSocketStatus::Opened => Msg::Connected,
            WebSocketStatus::Closed | WebSocketStatus::Error => Msg::Disconnected,
        });

        let ws = WebSocketService::connect_text(
            "ws://localhost:4000/graphql",
            ws_msg_callback,
            ws_notification_callback,
        )
        .unwrap();

        self.ws = Some(ws);
    }
}