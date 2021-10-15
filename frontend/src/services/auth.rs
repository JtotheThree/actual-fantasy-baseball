use graphql_client::{GraphQLQuery, QueryBody};
use yew::callback::Callback;
use yew::services::fetch::FetchTask;

use crate::error::Error;
use crate::services::GraphQL;
use crate::types::*;

/// Apis for authentication
#[derive(Default, Debug)]
pub struct Auth {
    requests: GraphQL,   
}

impl Auth {
    pub fn new() -> Self {
        Self {
            requests: GraphQL::new(),
        }
    }

    /// Get current user info
    pub fn current(&mut self, callback: Callback<Result<me::ResponseData, Error>>) -> FetchTask {
        let body = Me::build_query(me::Variables);
        self.requests.post::<QueryBody<me::Variables>, me::ResponseData>(body, callback)
    }

    /// Login a user
    pub fn login(
        &mut self,
        login_input: login::Variables,
        callback: Callback<Result<login::ResponseData, Error>>,
    ) -> FetchTask {
        let body = Login::build_query(login_input);
        self.requests.post::<QueryBody<login::Variables>, login::ResponseData>(body, callback)
    }

    /// Logout
    pub fn logout(
        &mut self,
        callback: Callback<Result<logout::ResponseData, Error>>,
    ) -> FetchTask {
        let body = Logout::build_query(logout::Variables);
        self.requests.post::<QueryBody<logout::Variables>, logout::ResponseData>(body, callback)
    }

    /// Register a new user
    pub fn signup(
        &mut self,
        signup_input: signup::Variables,
        callback: Callback<Result<signup::ResponseData, Error>>,
    ) -> FetchTask {
        let body = Signup::build_query(signup_input);
        self.requests.post::<QueryBody<signup::Variables>, signup::ResponseData>(body, callback)
    }
}