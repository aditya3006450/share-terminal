use crate::{
    components::pixel_art::pixel_art::PixelArt, router::router::Route, services::api,
};
use gloo::console::info;
use gloo::timers::callback::Timeout;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(SignUp)]
pub fn signup() -> Html {
    let email = use_state(|| String::new());
    let error = use_state(|| String::new());
    let success_message = use_state(|| String::new());
    let is_loading = use_state(|| false);

    let on_email_input = {
        let email = email.clone();
        let error = error.clone();
        let success_message = success_message.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            email.set(input.value());
            error.set(String::new());
            success_message.set(String::new());
        })
    };

    let navigator = use_navigator().unwrap();

    let on_submit = {
        let email = email.clone();
        let error = error.clone();
        let success_message = success_message.clone();
        let is_loading = is_loading.clone();
        let navigator = navigator.clone();

        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let email_value = (*email).clone();
            let error = error.clone();
            let success_message = success_message.clone();
            let is_loading = is_loading.clone();

            if email.is_empty() {
                error.set("Please fill in all fields".to_string());
                return;
            }
            
            is_loading.set(true);
            let navigator = navigator.clone();

            spawn_local(async move {
                match api::signup(email_value.to_string()).await {
                    Ok(response) => {
                        info!(response.message);
                        error.set(String::new());
                        success_message
                            .set("Email sent successfully! Login to continue.".to_string());
                        let navigator = navigator.clone();
                        let timeout = Timeout::new(3000, move || {
                            navigator.push(&Route::Login);
                        });
                        timeout.forget();
                        is_loading.set(false);
                    }
                    Err(_) => {
                        error.set("Login failed. Please check your credentials.".to_string());
                        is_loading.set(false);
                    }
                }
            });
        })
    };

    html! {
        <div class="login-container">
            <div class="login-left">
                <PixelArt/>
            </div>
            <div class="login-right">
                <div class="login-card">
                    <div class="logo-container">
                        <div class="logo"></div>
                    </div>
                    <h1 class="login-title">{"Welcome"}</h1>
                    <p class="login-subtitle">{"Enter your email to sign up"}</p>

                    {if !(*error).is_empty() {
                        html! {
                            <div class="error-message">{&*error}</div>
                        }
                    } else if !(*success_message).is_empty() {
                        html! {
                            <div class="success-message">{&*success_message}</div>
                        }
                    } else {
                        html! {}
                    }}

                    <form class="login-form">
                        <div class="form-group">
                            <label for="email">{"Email"}</label>
                            <div class="input-with-icon">
                                <input
                                    type="email"
                                    id="email"
                                    placeholder="Your email address"
                                    value={(*email).clone()}
                                    oninput={on_email_input}
                                    class="input-field"
                                    required=true
                                    disabled={*is_loading}
                                />
                                <span class="input-icon email-icon"></span>
                            </div>
                        </div>

                        <button onclick={on_submit} class="submit-btn" disabled={*is_loading}>
                            {if *is_loading { "Loading..." } else { "SignUp" }}
                        </button>
                    </form>

                    <div class="login-footer">

                        <p>{"Have an account? "} <a href="#" class="signup-link"><Link<Route> to={Route::Login}>{"Login"}</Link<Route>></a></p>
                    </div>
                </div>
            </div>
        </div>
    }
}
