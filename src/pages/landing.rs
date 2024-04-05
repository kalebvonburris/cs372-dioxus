use crate::{routes::Route, server::{ServerResponse, login_user, signup_user}, User};
use dioxus::prelude::*;

static POP_UP: GlobalSignal<(bool, String)> = Signal::global(|| (false, String::new()));
pub static LOGIN_SIGNAL: GlobalSignal<User> = Signal::global(User::default);

#[component]
pub fn Landing() -> Element {
    let nav = use_navigator();

    rsx! {
        // Login form
        div { class: "flex flex-col w-full h-screen justify-center items-center bg-[#833F00]",
            div { class: "w-96 h-fit p-8 border rounded-md space-y-2",
                h2 { class: "text-3xl font-bold text-center text-white", "Login/Signup" }
                div { class: "flex flex-col w-full space-y-1 content-center",
                    input {
                        class: "border text-lg text-white w-full rounded-full px-4 py-1 bg-[#833F00]",
                        r#type: "text",
                        placeholder: "Username",
                        "autofill": false,
                        value: "{LOGIN_SIGNAL().username}",
                        onchange: move |e| LOGIN_SIGNAL.write().username = e.value()
                    }
                    input {
                        class: "border text-lg text-white w-full rounded-full px-4 py-1 bg-[#833F00]",
                        r#type: "password",
                        placeholder: "Password",
                        "autofill": false,
                        value: "{LOGIN_SIGNAL().password}",
                        onchange: move |e| LOGIN_SIGNAL.write().password = e.value()
                    }
                }
                div { class: "flex flex-col justify-center w-full space-y-1 pt-6",
                    button {
                        class: "w-full p-2 border border-2 bg-white font-bold rounded-full text-bold",
                        onclick: move |_| {
                            async move {
                                let response = match login_user(LOGIN_SIGNAL()).await {
                                    Ok(response) => response,
                                    Err(e) => {
                                        *POP_UP.write() = (true, e.to_string());
                                        return;
                                    }
                                };
                                match response {
                                    ServerResponse::LoginSuccess(_) => {
                                        nav.push(Route::Homepage { });
                                    }
                                    ServerResponse::LoginFailure(reason) => {
                                        *POP_UP.write() = (true, reason);
                                    }
                                    _ => {}
                                }
                            }
                        },
                        "Login"
                    }
                    button {
                        class: "w-full p-2 border border-2 bg-white font-bold rounded-full text-bold",
                        onclick: move |_| {
                            async move {
                                let response = match signup_user(LOGIN_SIGNAL()).await {
                                    Ok(response) => response,
                                    Err(e) => {
                                        *POP_UP.write() = (true, e.to_string());
                                        return;
                                    }
                                };
                                println!("server responded with {response:?}");
                                match response {
                                    ServerResponse::SignupSuccess => {
                                        *POP_UP.write() = (true, "Signup successful!".to_string());
                                    }
                                    ServerResponse::SignupFailure(reason) => {
                                        *POP_UP.write() = (true, reason);
                                    }
                                    _ => {}
                                }
                            }
                        },
                        "Sign-Up"
                    }
                }
            }
        }
        // Pop up
        if POP_UP().0 {
            div {
                onclick: move |_| *POP_UP.write() = (false, String::new()),
                class: "absolute flex justify-center content-center top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 w-fit h-fit max-w-96 p-2 bg-gray-900 rounded",
                p { class: "text-white text-center w-full", "{POP_UP().1}" }
            }
        }
    }
}
