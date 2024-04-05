#![allow(non_snake_case)]
#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery
)]

use cs372_dioxus::routes::Route;
use dioxus::prelude::*;

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    rsx! { Router::<Route> {} }
}
