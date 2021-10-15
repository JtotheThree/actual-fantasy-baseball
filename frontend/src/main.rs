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
//use yew_router::components::RouterAnchor;

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
    current_route: Option<AppRoute>,
    current_user: Option<User>,
    current_user_response: Callback<Result<me::ResponseData, Error>>,
    current_user_task: Option<FetchTask>,
    #[allow(unused)]
    router_agent: Box<dyn Bridge<RouteAgent>>,
    //socket_agent: Box<dyn Bridge<agents::socket::Socket>>,
    link: ComponentLink<Self>,
}

pub enum Msg {
    CurrentUserResponse(Result<me::ResponseData, Error>),
    Route(Route),
    Authenticated(User),
    Logout,
}


impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        //let socket_agent = agents::socket::Socket::bridge(link.callback(|_| {}));
        let router_agent = RouteAgent::bridge(link.callback(Msg::Route));
        let route_service: RouteService = RouteService::new();
        let route = route_service.get_route();
        Self {
            auth: Auth::new(),
            current_route: AppRoute::switch(route),
            router_agent,
            current_user: None,
            current_user_response: link.callback(Msg::CurrentUserResponse),
            current_user_task: None,
            //socket_agent: socket_agent,
            link,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        // Get current user info if a token is available when mounted
        if first_render && is_authenticated() {
            let task = self.auth.current(self.current_user_response.clone());
            self.current_user_task = Some(task);
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::CurrentUserResponse(Ok(me)) => {
                self.current_user = Some(User{
                    username: me.me.username,
                    email: me.me.email,
                    role: me.me.role,
                });
                self.current_user_task = None;
            }
            Msg::CurrentUserResponse(Err(_)) => {
                self.current_user_task = None;
            }
            Msg::Route(route) => {
                self.current_route = AppRoute::switch(route)
            }
            Msg::Authenticated(user_info) => {
                self.current_user = Some(user_info);
            }
            Msg::Logout => {
                self.current_user = None;
            }
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        //type Anchor = RouterAnchor<Route>;

        let callback_login = self.link.callback(Msg::Authenticated);
        let callback_signup = self.link.callback(Msg::Authenticated);
        let callback_logout = self.link.callback(|_| Msg::Logout);

        html! {
            <>
                <Header current_user=self.current_user.clone() callback=callback_logout/>
                {
                    // Routes to render sub components
                    if let Some(route) = &self.current_route {
                        match route {
                            AppRoute::Login => html!{<Login callback=callback_login />},
                            AppRoute::Signup => html!(<Signup callback=callback_signup />),
                            AppRoute::Rules => html!{<Rules />},
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
