use crate::{router::router::Route, utils::auth::clear_auth_token};
use yew::prelude::*;
use yew_icons::{Icon, IconId};
use yew_router::prelude::use_navigator; // Import use_navigator for navigation // Import clear_auth_token

#[derive(Properties, PartialEq)]
pub struct TopbarProps {
    pub dark_mode: bool,
    pub toggle_theme: Callback<MouseEvent>,
    pub is_logged_in: bool,
    pub on_sidebar_toggle: Callback<MouseEvent>,
}

#[function_component(Topbar)]
pub fn topbar(props: &TopbarProps) -> Html {
    let navigator = use_navigator().expect("Navigator not found"); // Get the navigator for redirection

    let on_logout_click = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            clear_auth_token(); // Clear the authentication token
            navigator.push(&Route::Login);
        })
    };

    html! {
        <header class="topbar">
            <div class="topbar-content">
                <div style="display: flex; align-items: center; gap: 12px;">
                    <button class="menu-toggle" onclick={props.on_sidebar_toggle.clone()} title="Open navigation">
                        {"Menu"}
                    </button>
                    <div class="topbar-brand">{"Dashboard"}</div>
                </div>
                <div class="topbar-actions">
                    <button class="theme-toggle" onclick={props.toggle_theme.clone()} title="Toggle theme">
                        {
                            if props.dark_mode {
                                html! { <Icon icon_id={IconId::BootstrapSunFill} /> }
                            } else {
                                html! { <Icon icon_id={IconId::BootstrapMoonFill} /> }
                            }
                        }
                    </button>
                    {if props.is_logged_in {
                        html! {<button class="logout-btn" onclick={on_logout_click}> {"Logout"} </button>}
                    } else {
                        html! {
                            <>
                                <a class="login-link" href="/">{"Login"}</a>
                                <a class="signup-link" href="/signup">{"Signup"}</a>
                            </>
                        }
                    }}
                </div>
            </div>
        </header>
    }
}
