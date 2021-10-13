use graphql_client::{GraphQLQuery, QueryBody};
use yew::callback::Callback;
use yew::services::fetch::FetchTask;

use crate::error::Error;
use crate::services::GraphQLRequests;
use crate::types::*;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/auth/login.graphql",
    response_derives = "Debug"
)]
struct Login;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/auth/logout.graphql",
    response_derives = "Debug"
)]
struct Logout;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/auth/signup.graphql",
    response_derives = "Debug"
)]
struct Signup;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/auth/me.graphql",
    response_derives = "Debug"
)]
struct Me;

/// Apis for authentication
#[derive(Default, Debug)]
pub struct Auth {
    requests: GraphQLRequests,   
}

impl Auth {
    pub fn new() -> Self {
        Self {
            requests: GraphQLRequests::new(),
        }
    }

    /// Get current user info
    pub fn current(&mut self, callback: Callback<Result<MeResponseWrapper, Error>>) -> FetchTask {
        let body = Me::build_query(me::Variables);
        self.requests.post::<QueryBody<me::Variables>, MeResponseWrapper>(body, callback)
    }

    /// Login a user
    pub fn login(
        &mut self,
        login_input: LoginInput,
        callback: Callback<Result<LoginResponseWrapper, Error>>,
    ) -> FetchTask {
        let body = Login::build_query(login::Variables {
            username_or_email: login_input.username,
            password: login_input.password,
        });

        self.requests.post::<QueryBody<login::Variables>, LoginResponseWrapper>(body, callback)
    }

    /// Logout
    pub fn logout(
        &mut self,
        callback: Callback<Result<LogoutResponseWrapper, Error>>,
    ) -> FetchTask {
        let body = Logout::build_query(logout::Variables);
        self.requests.post::<QueryBody<logout::Variables>, LogoutResponseWrapper>(body, callback)
    }

    /// Register a new user
    pub fn signup(
        &mut self,
        signup_input: SignupInput,
        callback: Callback<Result<SignupResponseWrapper, Error>>,
    ) -> FetchTask {
        let body = Signup::build_query(signup::Variables {
            username: signup_input.username,
            email: signup_input.email,
            password: signup_input.password,
        });

        self.requests.post::<QueryBody<signup::Variables>, SignupResponseWrapper>(body, callback)
    }
}