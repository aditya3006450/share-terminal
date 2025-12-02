use crate::components::pixel_art::pixel_art::PixelArt;
use crate::router::router::Route;
use crate::services::api;
use crate::utils::auth::store_auth_token;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;
#[function_component(Login)]
pub fn login() -> Html {
    let email = use_state(|| String::new());
    let password = use_state(|| String::new());
    let error = use_state(|| String::new());

    let on_email_input = {
        let email = email.clone();
        let error = error.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            email.set(input.value());
            error.set(String::new());
        })
    };

    let on_password_input = {
        let password = password.clone();
        let error = error.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            password.set(input.value());
            error.set(String::new());
        })
    };

    let navigator = use_navigator().unwrap();

    let on_submit = {
        let email = email.clone();
        let password = password.clone();
        let error = error.clone();
        let navigator = navigator.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            if email.is_empty() || password.is_empty() {
                error.set("Please fill in all fields".to_string());
                return;
            }

            let email_val = (*email).clone();
            let password_val = (*password).clone();
            let error = error.clone();
            let navigator = navigator.clone();

            spawn_local(async move {
                match api::login(email_val, password_val).await {
                    Ok(response) => {
                        store_auth_token(&response.token);
                        navigator.push(&Route::Access);
                    }
                    Err(_) => {
                        error.set("Login failed. Please check your credentials.".to_string());
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
                    <h1 class="login-title">{"Welcome Back"}</h1>
                    <p class="login-subtitle">{"Enter your credentials to access your account"}</p>

                    {if !(*error).is_empty() {
                        html! {
                            <div class="error-message">{&*error}</div>
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
                                />
                                <span class="input-icon email-icon"></span>
                            </div>
                        </div>

                        <div class="form-group">
                            <div class="password-header">
                                <label for="password">{"Password"}</label>
                                <a href="#" class="forgot-password">{"Forgot Password?"}</a>
                            </div>
                            <div class="input-with-icon">
                                <input
                                    type="password"
                                    id="password"
                                    placeholder="Your password"
                                    value={(*password).clone()}
                                    oninput={on_password_input}
                                    class="input-field"
                                    required=true
                                />
                                <span class="input-icon password-icon"></span>
                            </div>
                        </div>

                        <button onclick={on_submit} class="submit-btn">{"LogIn"}</button>
                    </form>

                    <div class="login-footer">

                        <p>{"Don't have an account?"} <a href="#" class="signup-link"><Link<Route> to={Route::Signup}>{"Signup"}</Link<Route>></a></p>
                    </div>
                </div>
            </div>
        </div>
    }
}
