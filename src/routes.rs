#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::frontend::components::*;

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[route("/")]
    Home {},

    #[route("/register/:user/:password")]
    Registration { user: String, password: String },

    #[route("/auth")]
    RegistrationPage {},
}

