use bounce::*;
use gloo::storage::{LocalStorage, Storage};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{
    api,
    api::{use_query, QueryResult},
    components::form::*,
    Route,
    state::QueryMe,
};

#[function_component(Login)]
pub fn login() -> Html {
    let username = use_node_ref();
    let password = use_node_ref();
    let error = use_state(|| String::default());
    let query_me = use_future_notion_runner::<QueryMe>();

    let history = use_history().unwrap();

    let query_login = use_query({
        let history = history.clone();
        let error = error.clone();
        let query_me = query_me.clone();

        move |resp: QueryResult<api::Login> | {
            match resp.result {
                Ok(data) => {
                    LocalStorage::set("token", data.login.token).unwrap();
                    query_me(().into());
                    history.push(Route::Home);
                }
                Err(err) => {
                    error.set(err);
                }
            }
        }
    });

    let onsubmit = {
        let username = username.clone();
        let password = password.clone();

        Callback::from(move |ev: FocusEvent| {
            ev.prevent_default();

            query_login(api::login::Variables{
                username_or_email: username.cast::<HtmlInputElement>().unwrap().value(),
                password: password.cast::<HtmlInputElement>().unwrap().value(),
            });
        })
    };

    html! {
        <FullPageForm title="Login" submit_label="Login" error={error.to_string()} onsubmit={onsubmit}>
            <FormInput label="Username" value={username} />
            <FormInput label="Password" _type="password" value={password} />
        </FullPageForm>
    }
}