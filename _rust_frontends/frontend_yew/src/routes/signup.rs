use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{
    api,
    api::{QueryResult, use_query},
    components::form::*,
    Route,
};


#[function_component(Signup)]
pub fn sign_up() -> Html {
    let username = use_node_ref();
    let email = use_node_ref();
    let password = use_node_ref();
    let confirm_password = use_node_ref();

    let history = use_history().unwrap();
    let error = use_state(|| "".to_string());

    let query_signup = use_query({
        let history = history.clone();
        let error = error.clone();

        move |resp: QueryResult<api::Signup>| {
            match resp.result {
                Ok(_) => {history.push(Route::Home)}
                Err(err) => {error.set(err)}
            }
        }
    });

    let onsubmit = {
        let (username, email) = (username.clone(), email.clone());
        let (password, confirm_password) = (password.clone(), confirm_password.clone());

        let error = error.clone();

        Callback::from(move |ev: FocusEvent| {
            ev.prevent_default();

            let password = password.cast::<HtmlInputElement>().unwrap().value();
            let confirm_password = confirm_password.cast::<HtmlInputElement>().unwrap().value();

            if password != confirm_password {
                error.set("Passwords do not match".to_string());
                return;
            }

            query_signup(api::signup::Variables{
                username: username.cast::<HtmlInputElement>().unwrap().value(),
                email: email.cast::<HtmlInputElement>().unwrap().value(),
                password: password,
            });
        })
    };

    html! {
        <FullPageForm title="Sign Up" submit_label="Sign Up" error={error.to_string()} onsubmit={onsubmit}>
            <FormInput label="Username" value={username} />
            <FormInput label="E-mail" value={email} />
            <FormInput label="Password" _type="password" value={password} />
            <FormInput label="Confirm Password" _type="password" value={confirm_password} />
        </FullPageForm>
    }
}