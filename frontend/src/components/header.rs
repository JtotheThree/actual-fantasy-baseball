use yew::{html, Component, ComponentLink, Html, ShouldRender, Callback, Bridge, agent::Bridged};
use yew_router::{agent::RouteRequest::ChangeRoute, prelude::*};
use yew::services::fetch::FetchTask;
use yewtil::store::{Bridgeable, ReadOnly, StoreWrapper};

use crate::agents::{State, state::Request};
use crate::components::league_dropdown::LeagueDropdown;
use crate::error::Error;
use crate::services::{set_token, Auth};
use crate::routes::AppRoute;
use crate::types::*;

pub struct Header {
    auth: Auth,
    user: Option<User>,
    logout_response: Callback<Result<logout::ResponseData, Error>>,
    state: Box<dyn Bridge<StoreWrapper<State>>>,
    router_agent: Box<dyn Bridge<RouteAgent>>,
    task: Option<FetchTask>,
    link: ComponentLink<Self>,
}

pub enum Msg {
    Logout,
    LogoutResponse(Result<logout::ResponseData, Error>),
    StateMsg(ReadOnly<State>),
    Ignore,
}

impl Component for Header {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(Msg::StateMsg);

        Header {
            auth: Auth::new(),
            user: None,
            task: None,
            logout_response: link.callback(Msg::LogoutResponse),
            state: State::bridge(callback),
            router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Logout => {
                self.task = Some(self.auth.logout(self.logout_response.clone()));
            }
            Msg::LogoutResponse(Ok(_)) => {
                set_token(None);

                // Notify state to clear current user info
                self.state.send(Request::UpdateUser(None));

                // Redirect to home page
                self.router_agent.send(ChangeRoute(AppRoute::Home.into()));  
            }
            Msg::LogoutResponse(Err(err)) => {
                error!("{:?}", err);
            }
            Msg::StateMsg(state) => {
                let state = state.borrow();

                if state.user != self.user {
                    self.user = state.user.clone();
                } else {
                    return false;
                }
            }
            Msg::Ignore => {
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        info!("Header: view()");
        html! {
            <header class="border-solid border-b-2 border-black shadow-md">
            <nav class="flex flex-wrap items-center justify-between w-full py-4 md:py-0 px-4 text-2xl bg-paper">
                <ul class="flex flex-wrap">
                    <RouterAnchor<AppRoute> route=AppRoute::Home>
                        <a class="font-title" href="#">
                            { "Fantasy Baseball" }
                        </a>
                    </RouterAnchor<AppRoute>>
                    {
                        if let Some(_) = &self.user {
                            html!{ <LeagueDropdown /> }
                        } else {
                            html!{}
                        }
                    }
                </ul>
                {
                    if let Some(user) = &self.user {
                        self.logged_in_view(&user)
                    } else {
                        self.logged_out_view()
                    }
                }
            </nav>
            </header>
        }
    }
}

impl Header {
    fn logged_out_view(&self) -> Html {
        html! {
            <ul class="pt-4 text-base md:flex md:justify-between md:pt-0 font-normal font-bold">
                <li>
                    <RouterAnchor<AppRoute> route=AppRoute::Home classes="md:p-4 py-2 block hover:text-red-800">
                        { "News" }
                    </RouterAnchor<AppRoute>>
                </li>
                <li>
                    <RouterAnchor<AppRoute> route=AppRoute::Rules classes="md:p-4 py-2 block hover:text-red-800">
                        { "Rulebook" }
                    </RouterAnchor<AppRoute>>
                </li>
                <li>
                    <RouterAnchor<AppRoute> route=AppRoute::Signup classes="md:p-4 py-2 block hover:text-red-800">
                        { "Sign Up" }
                    </RouterAnchor<AppRoute>>
                </li>
                <li>
                    <RouterAnchor<AppRoute> route=AppRoute::Login classes="md:p-4 py-2 block hover:text-red-800">
                        { "Login" }
                    </RouterAnchor<AppRoute>>
                </li>
            </ul>
        }
    }

    fn logged_in_view(&self, user: &User) -> Html {
        let onclick = self.link.callback(|_| Msg::Logout);

        html! {
            <ul class="pt-4 text-base md:flex md:justify-between md:pt-0 font-normal font-bold">
                <li>
                    <RouterAnchor<AppRoute> route=AppRoute::Home classes="md:p-4 py-2 block hover:text-red-800">
                        { "News" }
                    </RouterAnchor<AppRoute>>
                </li>
                <li>
                    <RouterAnchor<AppRoute> route=AppRoute::Rules classes="md:p-4 py-2 block hover:text-red-800">
                        { "Rulebook" }
                    </RouterAnchor<AppRoute>>
                </li>
                <li>
                    <a class="md:p-4 py-2 block hover:text-red-800" href="#">{ user.username.clone() }</a>
                </li>
                <li>
                    <a class="md:p-4 py-2 block hover:text-red-800" href="#" onclick=onclick>{ "Logout" }</a>
                </li>
            </ul>
        }
    }
}
