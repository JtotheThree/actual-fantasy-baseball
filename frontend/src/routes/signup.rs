use yew::services::fetch::FetchTask;
use yew::{
    agent::Bridged, html, Bridge, Callback, Component, ComponentLink, FocusEvent, Html, InputData,
    Properties, ShouldRender,
};
use yew_router::{agent::RouteRequest::ChangeRoute, prelude::*};

use crate::components::list_errors::ListErrors;
use crate::error::Error;
use crate::services::Auth;
use crate::types::*;

use crate::routes::AppRoute;

pub struct Signup {
    auth: Auth,
    error: Option<crate::error::Error>,
    request: SignupInput,
    response: Callback<Result<SignupResponseWrapper, Error>>,
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
    Response(Result<SignupResponseWrapper, Error>),
    Ignore,
    UpdateUsername(String),
    UpdateEmail(String),
    UpdatePassword(String),
    UpdateConfirmPassword(String),
}

impl Component for Signup {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Signup {
            auth: Auth::new(),
            error: None,
            props,
            request: SignupInput::default(),
            response: link.callback(Msg::Response),
            router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
            task: None,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Request => {
                if self.request.password != self.request.confirm_password {
                    self.error = Some(Error::ConfirmPasswordError)
                } else {
                    self.task = Some(self.auth.signup(self.request.clone(), self.response.clone()));
                }
            }
            Msg::Response(Ok(signup_info)) => {
                //error!{format!{"{:?}", user_info}};
                // Set global token after logged in
                if signup_info.signup.status == "success" {
                    self.error = None;
                    self.task = None;
                    self.router_agent.send(ChangeRoute(AppRoute::Login.into()));
                } else {
                    self.task = None;
                    self.error = Some(Error::InternalServerError);
                }
            }
            Msg::Response(Err(err)) => {
                //error!{"{:?}", err};
                self.error = Some(err);
                self.task = None;
            }
            Msg::UpdateUsername(username) => {
                self.request.username = username;
            }
            Msg::UpdateEmail(email) => {
                self.request.email = email;
            }
            Msg::UpdatePassword(password) => {
                self.request.password = password;
            }
            Msg::UpdateConfirmPassword(confirm_password) => {
                self.request.confirm_password = confirm_password;
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
        let oninput_email = self
            .link
            .callback(|ev: InputData| Msg::UpdateEmail(ev.value));
        let oninput_password = self
            .link
            .callback(|ev: InputData| Msg::UpdatePassword(ev.value));
        let oninput_confirm_password = self
            .link
            .callback(|ev: InputData| Msg::UpdateConfirmPassword(ev.value));
        html! {
            <>
            <div class="w-full justify-self-center md:w-1/2 mx-auto flex flex-col flex-wrap justify-between p-16">
            <h2 class="h-5 text-center border-b-2 border-gray-700 tracking-wid px-4 text-3xl font-title"><span class="bg-paper px-8">{ "Sign Up" }</span></h2>
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
                        value=self.request.email.clone()
                        oninput=oninput_email
                    />
                    <label class="px-4 pb-2 font-semibold">
                        { "E-mail" }
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
                <div class="flex flex-col py-3">
                    <input
                        class="input"
                        type="password"
                        value=self.request.confirm_password.clone()
                        oninput=oninput_confirm_password
                    />
                    <label class="px-4 pb-2 font-semibold">
                        { "Confirm Password" }
                    </label>
                </div>
                <div class="mt-2">
                    <button
                        class="btn p-3 my-2 bg-gray-700 text-paper rounded-sm border-b-4 border-paper w-full font-bold hover:bg-red-800"
                        type="submit"
                        disabled=false>
                        { "Sign Up" }
                    </button>
                </div>
            </form>
            </div>
            </>
        }
    }
}
