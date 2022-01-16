use gloo::storage::{LocalStorage, Storage};
use graphql_client::GraphQLQuery;
use graphql_client::QueryBody;
use reqwasm::http::*;
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use std::rc::Rc;
use yew::prelude::*;

pub mod auth;
pub mod league;
pub mod manager;
pub mod teams;

pub use auth::*;
pub use league::*;
pub use manager::*;
pub use teams::*;

const SERVER: &str = "http://127.0.0.1:4000/";

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResponseWrapper<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<GraphQLError>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GraphQLError {
    pub message: String
}

pub async fn post<T> (variables: QueryBody<T::Variables>) -> Result<T::ResponseData, String>
where
    T: GraphQLQuery,
{
    let body = to_string(&variables).unwrap();
    let token: Option<String> = LocalStorage::get("token").unwrap_or_else(|_| None);

    let mut builder = Request::post(SERVER)
        .body(body)
        .header("Content-Type", "application/json")
        .mode(RequestMode::Cors);

    if let Some(token) = token {
        let bearer = format!{"Bearer {}", token};
        builder = builder.header("Authorization", &bearer);
    } else {
        info!("No token, not sending auth");
    }

    let resp = builder.send().await;

    match resp {
        Ok(resp) => {
            if resp.ok() {
                let data: ResponseWrapper<T::ResponseData> = resp.json().await.unwrap();

                if let Some(errors) = data.errors {
                    let error_message = errors.first().unwrap().message.clone();
                    return Err(error_message);
                } else {
                    return Ok(data.data.unwrap());
                }
            } else {
                match resp.status() {
                    401 => return Err("Unauthorized".to_string()),
                    403 => return Err("Forbidden".to_string()),
                    404 => return Err("Not Found".to_string()),
                    500 => return Err("Internal Server Error".to_string()),
                    // TODO: 422 => return Err(Error::UnprocessableEntity(resp.text())),
                    _ => return Err("Http Request Error".to_string())
                }
            }
        }
        Err(error) => {
            error!{"{}", error};
            return Err("Http Request Error".to_string())
        }
    }
}

#[derive(Debug)]
pub struct QueryResult<T>
where
    T: GraphQLQuery
{
    pub result: Result<T::ResponseData, String>,
}

pub fn use_query<T>(callback: impl Fn(QueryResult<T>) + 'static) -> Rc<dyn Fn(T::Variables)>
where
    T: GraphQLQuery + 'static,
{
    let state = use_state(|| callback);

    Rc::new(move |input: T::Variables| {
        let state = state.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let body = T::build_query(input);
            let resp = post::<T>(body).await;

            state(QueryResult::<T>{result: resp});
        });
    })
}

/*pub fn use_query<T>(callback: Rc<dyn Fn(Result<T::ResponseData, String>)>) -> Rc<dyn Fn(T::Variables)>
where
    T: GraphQLQuery + 'static,
{
    let state = use_state(|| callback);

    Rc::new(move |input: T::Variables| {
        let state = state.clone();

        wasm_bindgen_futures::spawn_local(async move {
            let body = T::build_query(input);
            let resp = post::<T>(body).await;

            state(resp);
        });
    })
}*/

/*pub fn use_query_super<T>(callback: impl FnOnce(QueryResult<T>) + 'static) -> Rc<dyn Fn(T::Variables)>
where
    T: GraphQLQuery + 'static,
    //F: FnOnce(Result<T::ResponseData, String>) -> T::Variables + 'static
{
    //let state = use_state(|| callback);
    use std::cell::RefCell;

    let callback = Rc::new(RefCell::new(callback));

    Rc::new(move |input: T::Variables| {
        //let state = state.clone();

        //let callback = callback.clone();

        wasm_bindgen_futures::spawn_local(async move {
            let body = T::build_query(input);
            let resp = post::<T>(body).await;

            /*match resp {
                Ok(resp) => {
                    state(QueryResult::<T>{
                        data: resp,
                    });
                    state(QueryResult::<T>{
                        data: Some(resp),
                        error: String::default(),
                    });
                },
                Err(err) => {
                    state(QueryResult::<T>{
                        data: None,
                        error: err,
                    });
                }
            }*/

            (callback.borrow())(QueryResult::<T>{result: resp});
        });
    })
}*/

/*pub fn use_query<F, T>(callback: F) -> Rc<dyn Fn(T::Variables)>
where
    T: GraphQLQuery + 'static,
    F: Fn(Result<T::ResponseData, String>) + 'static,
{
    let state = use_state(|| callback);

    Rc::new(move |input| {
        let state = state.clone();

        wasm_bindgen_futures::spawn_local(async move {
            let body = to_string(&input).unwrap();
            let token: Option<String> = LocalStorage::get("token").unwrap_or_else(|_| None);

            let mut builder = Request::post(SERVER)
                .body(body)
                .header("Content-Type", "application/json")
                .mode(RequestMode::Cors);

            if let Some(token) = token {
                let bearer = format!{"Bearer {}", token};
                builder = builder.header("Authorization", &bearer);
            } else {
                info!("No token, not sending auth");
            }

            let resp = builder.send().await;

            let resp = match resp {
                Ok(resp) => {
                    if resp.ok() {
                        let data: ResponseWrapper<T::ResponseData> = resp.json().await.unwrap();

                        if let Some(errors) = data.errors {
                            let error_message = errors.first().unwrap().message.clone();
                            Err(error_message)
                        } else {
                            Ok(data.data.unwrap())
                        }
                    } else {
                        match resp.status() {
                            401 => Err("Unauthorized".to_string()),
                            403 => Err("Forbidden".to_string()),
                            404 => Err("Not Found".to_string()),
                            500 => Err("Internal Server Error".to_string()),
                            // TODO: 422 => return Err(Error::UnprocessableEntity(resp.text())),
                            _ => Err("Http Request Error".to_string())
                        }
                    }
                }
                Err(error) => {
                    error!{"{}", error};
                    Err("Http Request Error".to_string())
                }
            };

            state(resp);

        });
    })
}*/

// T::Variables
// T::ResponseData
// FUCK::build_query
// C: Callback type????