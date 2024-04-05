use crate::Movie;
use dioxus::prelude::*;

use crate::pages::landing::LOGIN_SIGNAL;

#[component]
pub fn Homepage() -> Element {
    let movies: Signal<Vec<Movie>> = use_signal(std::vec::Vec::new);

    rsx! {
        h1 { class: "w-full text-center",
            "Homepage landed! {LOGIN_SIGNAL():?}"
        }

        for movie in movies.iter() {
            div { class: "w-full h-fit p-2 border-2 rounded-md space-y-2",
                h2 { class: "text-3xl font-bold text-center text-white", "{movie.title}" }
                div { class: "flex flex-col w-full space-y-1 content-center",
                    p { class: "text-center", "{movie.description}" }
                    p { class: "text-center", "{movie.link}" }
                    p { class: "text-center", "{movie.rating}" }
                }
            }
        }
    }
}