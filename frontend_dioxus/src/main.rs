#[macro_use]
extern crate log;

use dioxus::prelude::*;
use dioxus::router::{Router, Route};
use gloo::storage::{LocalStorage, Storage};

mod api;
mod components;
mod routes;
mod post;
mod types;

use crate::{
    components::header,
    routes::login::login,
};

fn home(cx: Scope) -> Element {
    cx.render(rsx! {
        h1 { "Home! " }
    })
}

static APP: Component = |cx| {
    let token: Option<String> = LocalStorage::get("token").unwrap_or_else(|_| None);

    if token != None {

    }

    cx.render(rsx! {
        Router {
            header ()
            div {
                Route { to: "/", home() }
                Route { to: "/login", login() }
            }
        }
    })
};

fn main() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());
    dioxus::web::launch(APP);
}
