
pub mod home;
pub mod login;
pub mod rules;
pub mod signup;

use yew_router::prelude::*;

/// App routes
#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/login"]
    Login,
    #[to = "/signup"]
    Signup,
    #[to = "/rules"]
    Rules,
    #[to = "/"]
    Home,
}