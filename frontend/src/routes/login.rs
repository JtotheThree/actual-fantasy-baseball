use yew::prelude::*;
use yew::services::fetch::FetchTask;

use yew_router::{agent::RouteRequest::ChangeRoute, prelude::*};
use yewtil::store::{Bridgeable, ReadOnly, StoreWrapper};

use crate::agents::{State, state::Request};
use crate::components::list_errors::ListErrors;
use crate::error::Error;
use crate::services::{set_token, Auth};
use crate::types::*;

use crate::routes::AppRoute;

pub struct Login {
    auth: Auth,
    user: Option<User>,
    error: Option<crate::error::Error>,
    request: login::Variables,
    response: Callback<Result<login::ResponseData, Error>>,
    task: Option<FetchTask>,
    state: Box<dyn Bridge<StoreWrapper<State>>>,
    router_agent: Box<dyn Bridge<RouteAgent>>,
    link: ComponentLink<Self>,
}

pub enum Msg {
    Request,
    Response(Result<login::ResponseData, Error>),
    Ignore,
    UpdateUsername(String),
    UpdatePassword(String),
    StateMsg(ReadOnly<State>),
}

impl Component for Login {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(Msg::StateMsg);

        Login {
            auth: Auth::new(),
            user: None,
            error: None,
            request: login::Variables {
                username_or_email: "".to_string(),
                password: "".to_string(),
            },
            response: link.callback(Msg::Response),
            state: State::bridge(callback),
            router_agent: RouteAgent::bridge(link.callback(|_| Msg::Ignore)),
            task: None,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Request => {
                self.task = Some(self.auth.login(login::Variables{
                    username_or_email: self.request.username_or_email.clone(),
                    password: self.request.password.clone(),
                }, 
                self.response.clone()));
            }
            Msg::Response(Ok(response)) => {
                // Set global token after logged in
                set_token(Some(response.login.token.clone()));

                self.state.send(Request::UpdateUser(Some(User {
                    id: response.login.id,
                    username: response.login.username,
                    email: response.login.email,
                    role: response.login.role,
                })));

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
                self.request.username_or_email = username;
            }
            Msg::UpdatePassword(password) => {
                self.request.password = password;
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
                println!("Ignore me");
            }
        }

        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
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
                        value=self.request.username_or_email.clone()
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
