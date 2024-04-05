use dioxus::prelude::*;
use crate::pages::{homepage::Homepage, landing::Landing};

/// An enum of all of the possible routes in the app.
#[derive(Routable, Clone)]
pub enum Route {
    // The home page is at the / route
    #[route("/")]
    Landing {},
    #[route("/homepage")]
    Homepage { }
}
