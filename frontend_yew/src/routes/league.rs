
use yew::prelude::*;

use crate::{
    api,
    api::{QueryResult, use_query},
    components::base::Card,
};

#[derive(Properties, PartialEq)]
pub struct LeagueProps {
    pub id: String,
}

#[function_component(League)]
pub fn league(props: &LeagueProps) -> Html {
    let league_data: UseStateHandle<Option<api::get_league::ResponseData>> = use_state(|| None);
    let error = use_state(|| String::default());

    let query_league = use_query({
        let error = error.clone();
        let league_data = league_data.clone();

        move |resp: QueryResult<api::GetLeague>| {
            match resp.result {
                Ok(data) => league_data.set(Some(data)),
                Err(err) => error.set(err),
            }
        }
    });

    let view = if let Some(league_data) = &*league_data {
        if league_data.league.id == props.id {
            html! {
                <Card
                    header={{
                        html! {
                            <h1>{league_data.league.name.clone()}</h1>
                        }
                    }}
                    body={{
                        html! {
                            <>
                            {"Owner: "}{ league_data.league.owner.username.clone() }<br/>
                            {"State: "}{ format!{"{:?}", league_data.league.state } }<br/>
                            </>
                        }
                    }}
                />
            }
        } else {
            query_league(api::get_league::Variables{
                id: props.id.clone(),
            });

            html! {
                <h1>{"Loading..."}</h1>
            }
        }
    } else {
        query_league(api::get_league::Variables{
            id: props.id.clone(),
        });

        html! {
            <h1>{"Loading..."}</h1>
        }
    };

    html! {
        <div class="flex flex-col md:w-1/3 mx-auto p-8 space-y-8">
            { view }
        </div>
    }
}