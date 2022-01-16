use bounce::*;
use gloo::storage::{LocalStorage, Storage};
use yew::prelude::*;
use yew_router::prelude::*;
use web_sys::HtmlInputElement;

use crate::{
    api,
    api::{use_query, QueryResult},
    state::QueryMyLeagues,
    components::form::*,
    Route,
};

#[function_component(CreateLeague)]
pub fn create_league() -> Html {
    let name = use_node_ref();
    let description = use_node_ref();
    let public = use_node_ref();
    let password = use_node_ref();
    let max_players = use_node_ref();
    let manual_state = use_node_ref();

    let history = use_history().unwrap();
    let error = use_state(|| "".to_string());

    let run_fetch_leagues = use_future_notion_runner::<QueryMyLeagues>();

    let query_create_league = use_query({
        let history = history.clone();
        let error = error.clone();

        move |resp: QueryResult<api::CreateLeague>| {
            match resp.result {
                Ok(resp) => {
                    LocalStorage::set("selected_league", resp.create_league.id).unwrap();
                    run_fetch_leagues(().into());
                    history.push(Route::Home)
                }
                Err(err) => {error.set(err)}
            }
        }
    });

    let onsubmit = {
        let name = name.clone();
        let description = description.clone();
        let public = public.clone();
        let password = password.clone();
        let max_players = max_players.clone();
        let manual_state = manual_state.clone();

        Callback::from(move |ev: FocusEvent| {
            ev.prevent_default();

            let password = if password.cast::<HtmlInputElement>().unwrap().value() == "" {
                None
            } else {
                Some(password.cast::<HtmlInputElement>().unwrap().value())
            };

            query_create_league(api::create_league::Variables{
                name: name.cast::<HtmlInputElement>().unwrap().value(),
                description: description.cast::<HtmlInputElement>().unwrap().value(),
                public: public.cast::<HtmlInputElement>().unwrap().checked(),
                password: password,
                max_players: max_players.cast::<HtmlInputElement>().unwrap().value_as_number() as i64,
                manual_state: manual_state.cast::<HtmlInputElement>().unwrap().checked(),
            });
        })
    };

    html! {
        <FullPageForm title="Create League" submit_label="Create League" error={error.to_string()} onsubmit={onsubmit}>
            <FormInput label="League Name" value={name} />
            <FormInput label="League Description" value={description} />
            <FormNumber label="Max Players" min="2" max="64" default="16" value={max_players} />
            <FormCheckbox label="Public" value={public} />
            <FormCheckbox label="Manual State" value={manual_state} />
            <FormInput label="Password" value={password} />
        </FullPageForm>
    }
}