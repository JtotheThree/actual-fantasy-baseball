use gloo::storage::{LocalStorage, Storage};
use graphql_client::GraphQLQuery;

use crate::{
    types::*,
    post::post,
};

pub async fn login(username: String, password: String) -> Result<login::ResponseData, String> {
    let body = Login::build_query(login::Variables{
        username_or_email: username,
        password: password,
    });

    let resp = post::<login::Variables, login::ResponseData>(body).await;

    match resp {
        Ok(data) => {
            // TODO: Handle unwrap yeah?
            LocalStorage::set("token", data.login.token.clone()).unwrap();
            Ok(data)
        },
        Err(err) => Err(err),
    }
}