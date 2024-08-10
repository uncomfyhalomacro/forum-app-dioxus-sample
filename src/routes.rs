#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

use crate::components::*;

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[route("/")]
    Home {},

    #[route("/register/:user/:password")]
    Registration { user: String, password: String },

    #[route("/auth")]
    RegistrationPage {},
}

