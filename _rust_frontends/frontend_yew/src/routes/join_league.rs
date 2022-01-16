use bounce::*;
use gloo::storage::{LocalStorage, Storage};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{
    api,
    api::{use_query, QueryResult},
    components::base::{Button, Card},
    Route,
    state::{MyLeaguesState, Reset},
};

#[function_component(JoinLeague)]
pub fn join_league() -> Html {
    let league_data: UseStateHandle<Option<api::public_leagues::ResponseData>> = use_state(|| None);

    let my_leagues_reset = use_notion_applier::<Reset>();

    let history = use_history().unwrap();
    let error = use_state(|| String::default());

    let query_leagues = use_query({
        let error = error.clone();
        let league_data = league_data.clone();

        move |resp: QueryResult<api::PublicLeagues>| {
            match resp.result {
                Ok(data) => league_data.set(Some(data)),
                Err(err) => error.set(err),
            }
        }
    });

    let join_league = use_query({
        let history = history.clone();
        let error = error.clone();

        move |resp: QueryResult<api::JoinLeague>| {
            match resp.result {
                Ok(data) => {
                    LocalStorage::set("selected_league", data.join_league.id.clone()).unwrap();
                    my_leagues_reset(Reset);
                    history.push(Route::League{ id: data.join_league.id.clone() });
                },
                Err(err) => error.set(err),
            }
        }
    });

    let onjoin = {       //leagues.borrow_mut().send(Request::SelectLeague(id));
        Callback::from(move |id: String| {
            join_league(api::join_league::Variables{
                id: id.clone(),
            })
        })
    };

    let view = if let Some(league_data) = &*league_data {
        let leagues_html = league_data.leagues.iter().map(|league| {
            html! {
                <Card
                    header={{ html!{ league.name.clone() } }}
                    body={{ html!{
                        <>{" Players: "}{"1/"}{ league.max_players }<br/>
                        {" Owner: "}{ league.owner.username.clone() }
                        /*{ "State: "}{ &league.state }*/
                        </>
                    } }}
                    footer={{ html!{
                        <Button
                            data={league.id.clone()}
                            onclick={onjoin.clone()}
                            text="Join"
                        />
                    } }}
                />
                }
        }).collect::<Html>();

        html! {
            { leagues_html }
        }
    } else {
        query_leagues(api::public_leagues::Variables{});

        html! {
        }
    };

    html! {
        <div class="flex flex-col md:w-1/3 mx-auto p-8 space-y-8">
            { error.to_string() }
            { view }
        </div>
    }
}