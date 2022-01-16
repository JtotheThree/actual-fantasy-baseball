use bounce::*;
use gloo::storage::{LocalStorage, Storage};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{
    api::League,
    state::{MyLeaguesState, QueryMyLeagues},
    components::base::{Dropdown, DropdownItem},
    Route,
};

#[function_component(LeagueDropdownLoader)]
fn league_dropdown_loader() -> Html {
    let my_leagues_state = use_atom::<MyLeaguesState>();

    match *my_leagues_state {
        MyLeaguesState::NotStarted => {
            info!("MyLeaguesState::NotStarted - Fetching");
            let run_query_leagues = use_future_notion_runner::<QueryMyLeagues>();
            run_query_leagues(().into());
        },
        MyLeaguesState::Pending => {
            info!("MyLeaguesState::Pending");
        },
        MyLeaguesState::Complete(ref my_leagues) => {
            info!("{:?}", my_leagues);
        },
    }

    html! {}
}

#[function_component(LeagueDropdownFilled)]
fn league_dropdown_filled() -> Html {
    let my_leagues = use_atom_value::<MyLeaguesState>();

    let selected_league_storage: UseStateHandle<Option<String>> = use_state(
        || LocalStorage::get("selected_league").unwrap_or(None));

    let leagues = match &*my_leagues {
        MyLeaguesState::NotStarted |
        MyLeaguesState::Pending => Vec::<League>::new(),
        MyLeaguesState::Complete(leagues) => leagues.to_vec()
    };

    let selected_league = if let Some(selected_league_storage) = &*selected_league_storage {
        leagues.iter().find(|&league| league.id == *selected_league_storage)
    } else {
        None
    };

    let history = use_history().unwrap();

    let onclick = {       //leagues.borrow_mut().send(Request::SelectLeague(id));
        let selected_league_storage = selected_league_storage.clone();
        let history = history.clone();

        Callback::from(move |id: String| {
            selected_league_storage.set(Some(id.clone()));
            LocalStorage::set("selected_league", id.clone()).unwrap();
            history.push(Route::League{ id: id});
        })
    };

    let leagues_html = leagues.iter().map(|league| {
        html! {
            <DropdownItem
                class="block p-4 text-lg font-normal font-bold hover:text-red-800"
                data={league.id.clone()}
                onclick={onclick.clone()}
            >
                <Link<Route> to={Route::League { id: league.id.clone() }}>
                    { league.name.clone() }
                </Link<Route>>
            </DropdownItem>
        }
    }).collect::<Html>();

    let dropdown_link = if let Some(selected_league) = selected_league {
        html! {
            <Link<Route> to={Route::League { id: selected_league.id.clone() }}>
                { selected_league.name.clone() }
            </Link<Route>>
        }
    } else {
        html! {
            <Link<Route> to={Route::League { id: leagues[0].id.clone() }}>
                { leagues[0].name.clone() }
            </Link<Route>>
        }
    };

    html! {
        <Dropdown class="md:px-6 px-6" parent={dropdown_link}>
            { leagues_html }
            <DropdownItem
                class="block p-4 text-lg font-normal font-bold hover:text-red-800 border-t-2"
                data=""
            >
                <Link<Route> to={Route::CreateLeague}>
                    { "Create a League" }
                </Link<Route>>
            </DropdownItem>

            <DropdownItem
                class="block p-4 text-lg font-normal font-bold hover:text-red-800"
                data=""
            >
                <Link<Route> to={Route::JoinLeague}>
                    { "Join a League" }
                </Link<Route>>
            </DropdownItem>
        </Dropdown>
    }
}

#[function_component(LeagueDropdownEmpty)]
fn leage_dropdown_empty() -> Html {
    html! {
        <>
        <span class="px-24 text-lg text-center font-normal font-bold">
        <Link<Route>
            classes="underline"
            to={Route::CreateLeague}>
            { "Create" }
        </Link<Route>>

        { " or "}

        <Link<Route>
            classes="underline"
            to={Route::JoinLeague}>
            { "Join" }
        </Link<Route>>

        { " a league" }

        </span>
        </>
    }
}

#[function_component(LeagueDropdown)]
pub fn league_dropdown() -> Html {
    let my_leagues = use_atom_value::<MyLeaguesState>();

    let empty = match &*my_leagues {
        MyLeaguesState::NotStarted |
        MyLeaguesState::Pending => {true}
        MyLeaguesState::Complete(leagues) => {
            if leagues.len() == 0 { true } else { false }
        }
    };

    html! {
        <>
        {
            if empty {
                html!{<LeagueDropdownEmpty />}
            } else {
                html! {<LeagueDropdownFilled />}
            }
        }
        <LeagueDropdownLoader />
        </>
    }
}