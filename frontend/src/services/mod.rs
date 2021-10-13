mod auth;
mod graphql;

pub use auth::Auth;
pub use graphql::{get_token, is_authenticated, set_token, GraphQLRequests};