use bounce::*;
use gloo::storage::{LocalStorage, Storage};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{
    components::{LeagueDropdown},
    Route,
    state::{MeState, Reset},
};

#[function_component(HeaderLeft)]
fn header_left() -> Html {
    let me_state = use_atom_value::<MeState>();

    html! {
        <ul class="flex flex-wrap">
            <Link<Route> classes="font-title" to={Route::Home}>
                {"Fantasy Baseball"}
            </Link<Route>>
            {
                match *me_state {
                    MeState::NotStarted |
                    MeState::Pending => {html!{
                    }}
                    MeState::Complete(_) => {html!{
                        <LeagueDropdown />
                    }}
                }
            }
        </ul>
    }
}

#[function_component(HeaderRight)]
fn header_right() -> Html {
    let me_state = use_atom_value::<MeState>();
    let me_reset = use_notion_applier::<Reset>();

    let history = use_history().unwrap();

    let onlogout = Callback::from(move |_| {
        LocalStorage::delete("token");
        me_reset(Reset);
        history.push(Route::Home);
    });

    html! {
        <ul class="pt-4 text-base md:flex md:justify-between md:pt-0 font-normal font-bold">
        {
            match *me_state {
                MeState::NotStarted |
                MeState::Pending => {html! {
                    <>
                    <li>
                        <Link<Route> classes="md:p-4 py-2 block hover:text-red-800" to={Route::Signup}>
                            { "Signup" }
                        </Link<Route>>
                    </li>
                    <li>
                        <Link<Route> classes="md:p-4 py-2 block hover:text-red-800" to={Route::Login}>
                            { "Login" }
                        </Link<Route>>
                    </li>
                    </>
                }}
                MeState::Complete(ref data) => {html! {
                    <>
                    <li>
                        <Link<Route> classes="md:p-4 py-2 block hover:text-red-800" to={Route::Home}>
                            { data.me.username.clone() }
                        </Link<Route>>
                    </li>
                    <li>
                        <button onclick={onlogout} class="md:p-4 py-2 block font-bold hover:text-red-800">
                            { "Logout" }
                        </button>
                    </li>
                    </>
                }}
            }
        }
        </ul>
    }
}

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header class="border-solid border-b-2 border-black shadow-md">
        <nav class="flex flex-wrap items-center justify-between w-full py-4 md:py-0 px-4 text-2xl bg-paper">
            <HeaderLeft />
            <HeaderRight />
        </nav>
        </header>
    }
}