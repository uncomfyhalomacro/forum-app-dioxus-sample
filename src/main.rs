#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/register/:user/:password")]
    Registration { user: String, password: String },

    #[route("/auth")]
    RegistrationPage {},
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

#[component]
fn Registration(user: String, password: String) -> Element {
    rsx! {
        div {
            "{user} now registered!"
        }
    }
}

#[component]
fn RegistrationPage() -> Element {
    let mut user = use_signal(|| "".to_string());
    let mut password = use_signal(|| "".to_string());
    rsx! {
    div {
            display: "flex", flex_direction: "column",
            h1 { "Site Registration" }
            form {
                onsubmit: move |event| { info!("Submitted! {event:?}")  },
            label { "Username:" input { margin: "0.5rem", r#type: "text", value: "{user}", oninput: move |event| user.set(event.value()) } }
            label { "Password:" input { margin: "0.5rem", r#type: "password", value: "{password}", oninput: move |event| password.set(event.value()) } }
        button { label { "Register!" } }
            }
    }
    }
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        div {
        Link {
            to: Route::RegistrationPage {},
            "Register here!"
        }
        }
    }
}
