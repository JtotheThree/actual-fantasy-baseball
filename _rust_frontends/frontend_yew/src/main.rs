#[macro_use]
extern crate log;

use bounce::*;
use gloo::storage::{LocalStorage, Storage};
use yew::prelude::*;
use yew_router::prelude::*;

mod api;
mod components;
mod state;
mod routes;

use crate::{
    state::*,
    components::Header,
    routes::{CreateLeague, Home, JoinLeague, League, Login, Signup},
};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/join_league")]
    JoinLeague,
    #[at("/league/:id")]
    League { id: String },
    #[at("/login")]
    Login,
    #[at("/signup")]
    Signup,
    #[at("/create_league")]
    CreateLeague,
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => {html! { <Home /> }}
        Route::JoinLeague => {html! { <JoinLeague />}}
        Route::League { id } => {html!{ <League id={id.clone()} /> } }
        Route::Login => {html! { <Login /> }}
        Route::Signup => {html! { <Signup /> }}
        Route::CreateLeague => {html! { <CreateLeague />}}
    }
}

#[function_component(Loader)]
fn loader() -> Html {
    let me_state = use_atom::<MeState>();

    match *me_state {
        MeState::NotStarted => {
            info!("MeState::NotStarted");

            let token: Option<String> = LocalStorage::get("token").unwrap_or_else(|_| None);

            if token != None {
                info!("Fetching me state");
                let run_query_me = use_future_notion_runner::<QueryMe>();
                run_query_me(().into());
            }
        },
        _ => {}
    }

    html! {}
}


#[function_component(App)]
fn app() -> Html {
    html! {
        <BounceRoot>
            <BrowserRouter>
                <main>
                    <Header />
                    <Switch<Route> render={Switch::render(switch)} />
                    <Loader />
                </main>
            </BrowserRouter>
        </BounceRoot>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());

    yew::start_app::<App>();
}