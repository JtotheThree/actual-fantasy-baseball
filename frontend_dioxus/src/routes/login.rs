use dioxus::prelude::*;
use dioxus::router::*;

use crate::{
    api,
    components::form::*,
};

pub fn login(cx: Scope) -> Element {
    let username = use_state(&cx, String::new);
    let password = use_state(&cx, String::new);

    let service = cx.consume_context::<RouterService>()?;

    let onsubmit = move |_| {
        cx.push_future({
            let (username, password) = (username.get().clone(), password.get().clone());
            let service = service.clone();

            async move {
                let resp = api::login(username, password).await;

                match resp {
                    Ok(_) => {
                        service.push_route("/");
                    },
                    Err(err) => { error!("{}", err); }
                }
            }
        });
    };

    cx.render(rsx!{
        div { class: "w-full justify-self-center md:w-1/2 mx-auto flex flex-col flex-wrap justify-between p-16",
            form_title ( text: "Login" )
            form { class: "md:w-1/2-screen m-0 p-12 w-full tw-h-full shadow-md",
                prevent_default: "onsubmit",
                onsubmit: onsubmit,
                form_input (
                    label: "Username",
                    r#type: "input"
                    state: username,
                )
                form_input (
                   label: "Password",
                   r#type: "password",
                   state: password,
                )
                form_submit (
                    name: "Login",
                    //onclick: onlogin,
                )
            }
        }
    })
}