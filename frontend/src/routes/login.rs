use yew::services::fetch::FetchTask;
use yew::{
    agent::Bridged, html, Bridge, Callback, Component, ComponentLink, FocusEvent, Html, InputData,
    Properties, ShouldRender,
};
use yew_router::{agent::RouteRequest::ChangeRoute, prelude::*};

use crate::components::list_errors::ListErrors;
use crate::error::Error;
use crate::services::{set_token, Auth};
use crate::types::*;

use crate::routes::AppRoute;

pub struct Login {
    auth: Auth,
    error: Option<crate::error::Error>,
    request: LoginInput,
    response: Callback<Result<LoginResponseWrapper, Error>>,
    task: Option<FetchTask>,
    props: Props,
    router_agent: Box<dyn Bridge<RouteAgent>>,
    link: ComponentLink<Self>,
}

#[derive(PartialEq, Properties, Clone)]
pub struct Props {
    /// Callback when user is logged in successfully
    pub callback: Callback<UserInfo>,
}

pub enum Msg {
    Request,
    Response(Result<LoginResponseWrapper, Error>),
    Ignore,
    UpdateUsername(String),
    UpdatePassword(String),
}

impl Component for Login {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Login {
            auth: Auth::new(),
            error: None,
            props,
            request: LoginInput::default(),
            response: link.callback(Msg::Response),
            router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
            task: None,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Request => {
                self.task = Some(self.auth.login(self.request.clone(), self.response.clone()));
            }
            Msg::Response(Ok(login_info)) => {
                //error!{format!{"{:?}", user_info}};
                // Set global token after logged in
                set_token(Some(login_info.login.token.clone()));

                self.props.callback.emit(UserInfo{
                    username: login_info.login.username,
                    email: login_info.login.email,
                    role: login_info.login.role,
                    token: login_info.login.token,
                });

                self.error = None;
                self.task = None;
                // Route to home page after login
                self.router_agent.send(ChangeRoute(AppRoute::Home.into()));
            }
            Msg::Response(Err(err)) => {
                //error!{"{:?}", err};
                self.error = Some(err);
                self.task = None;
            }
            Msg::UpdateUsername(username) => {
                self.request.username = username;
            }
            Msg::UpdatePassword(password) => {
                self.request.password = password;
            }
            Msg::Ignore => {
                println!("Ignore me");
            }
        }

        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let onsubmit = self.link.callback(|ev: FocusEvent| {
            ev.prevent_default(); /* Prevent event propagation */
            Msg::Request
        });
        let oninput_username = self
            .link
            .callback(|ev: InputData| Msg::UpdateUsername(ev.value));
        let oninput_password = self
            .link
            .callback(|ev: InputData| Msg::UpdatePassword(ev.value));
        html! {
            <>
            <div class="w-full justify-self-center md:w-1/2 mx-auto flex flex-col flex-wrap justify-between p-16">
            <h2 class="h-5 text-center border-b-2 border-gray-700 tracking-wid px-4 text-3xl font-title"><span class="bg-paper px-8">{ "Login" }</span></h2>
            //<h1 class="text-3xl tracking-wid text-center px-4 font-title">{ "Login" }</h1>
            //<div class="border-b-2 border-black"></div>
            <form class="md:w-1/2-screen m-0 p-12 w-full tw-h-full shadow-md" onsubmit=onsubmit>
                <ListErrors error=self.error.clone() />
                <div class="flex flex-col py-3">
                    <input
                        class="input"
                        value=self.request.username.clone()
                        oninput=oninput_username
                    />
                    <label class="px-4 pb-2 font-semibold">
                        { "Username" }
                    </label>
                </div>
                <div class="flex flex-col py-3">
                    <input
                        class="input"
                        type="password"
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
                        { "Sign In" }
                    </button>
                </div>
            </form>
            </div>
            </>
        }
    }
}
