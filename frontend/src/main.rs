#[macro_use]
extern crate log;

mod agents;
mod components;
mod error;
mod routes;
mod services;
mod types;
mod util;

use console_error_panic_hook::set_once as set_panic_hook;
use yew::services::fetch::FetchTask;
use yew::prelude::*;
use yew_router::prelude::*;
use yewtil::store::{Bridgeable, ReadOnly, StoreWrapper};
//use yew_router::components::RouterAnchor;

use crate::agents::{State, state::Request};
use crate::components::{header::Header};
use error::Error;
use routes::{
    home::Home,
    login::Login,
    signup::Signup,
    rules::Rules,
    AppRoute
};
use services::{is_authenticated, Auth};
use types::*;


struct App {
    auth: Auth,
    route: Option<AppRoute>,
    user_response: Callback<Result<me::ResponseData, Error>>,
    user_task: Option<FetchTask>,
    state: Box<dyn Bridge<StoreWrapper<State>>>,
    #[allow(unused)]
    router_agent: Box<dyn Bridge<RouteAgent>>,
    //socket_agent: Box<dyn Bridge<agents::socket::Socket>>,
}

pub enum Msg {
    StateMsg(ReadOnly<State>),
    UserResponse(Result<me::ResponseData, Error>),
    Route(Route),
}


impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(Msg::StateMsg);

        //let socket_agent = agents::socket::Socket::bridge(link.callback(|_| {}));
        let router_agent = RouteAgent::bridge(link.callback(Msg::Route));
        let route_service: RouteService = RouteService::new();
        let route = route_service.get_route();
        Self {
            auth: Auth::new(),
            state: State::bridge(callback),
            route: AppRoute::switch(route),
            router_agent,
            user_response: link.callback(Msg::UserResponse),
            user_task: None,
            //socket_agent: socket_agent,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        // Get current user info if a token is available when mounted
        if first_render && is_authenticated() {
            let task = self.auth.current(self.user_response.clone());
            self.user_task = Some(task);
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            // We don't really care about callbacks here, can we handle this differently?
            Msg::StateMsg(_) => {
                return false;
            }
            Msg::UserResponse(Ok(me)) => {
                let mut selected_league: Option<League> = None;

                if let Some(league) = me.me.selected_league {
                    selected_league = Some(League {
                        id: league.id,
                        name: league.name,
                        team: None,
                    });
                }

                self.state.send(Request::UpdateUser(Some(User {
                    id: me.me.id,
                    username: me.me.username,
                    email: me.me.email,
                    role: me.me.role,
                    selected_league,                    
                })));

                self.user_task = None;
            }
            Msg::UserResponse(Err(err)) => {
                error!("Me response error: {:?}", err);
                self.user_task = None;
            }
            Msg::Route(route) => {
                self.route = AppRoute::switch(route)
            }
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        //type Anchor = RouterAnchor<Route>;
        html! {
            <>
                <Header />
                {
                    // Routes to render sub components
                    if let Some(route) = &self.route {
                        match route {
                            AppRoute::Login => html!{<Login />},
                            AppRoute::Signup => html!{<Signup />},
                            AppRoute::Rules => html!{<Rules />},
                            AppRoute::League(id) => html!{<routes::league::League league_id=id.clone() />},
                            AppRoute::Home => html!{<Home />},
                        }
                    } else {
                        // 404 when route matches no component
                        html! { "No child component available" }
                    }
                }
            </>
        }
    }
}

fn main() {
    set_panic_hook();
    wasm_logger::init(wasm_logger::Config::default());

    yew::start_app::<App>();
}
