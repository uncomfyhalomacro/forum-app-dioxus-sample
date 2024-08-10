#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

use forum_app_dioxus_sample::routes::Route;


fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

