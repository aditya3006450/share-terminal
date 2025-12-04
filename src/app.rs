use crate::constants::{STORAGE_KEY_THEME, THEME_DARK, THEME_LIGHT};
use crate::utils::auth::get_auth_token;
use web_sys::window;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::router::{switch, Route};

#[function_component(App)]
pub fn app() -> Html {
    let dark_mode = use_state(get_saved_theme);
    let theme_class = if *dark_mode { "dark-theme" } else { "" };

    let toggle_theme = {
        let dark_mode = dark_mode.clone();
        Callback::from(move |_| {
            let new_theme = !*dark_mode;
            dark_mode.set(new_theme);
            save_theme(new_theme);
        })
    };

    let initial_route = if get_auth_token().is_some() {
        Route::Access
    } else {
        Route::Login
    };

    html! {
        <div class={theme_class}>
            <BrowserRouter>
                <Switch<Route> render={move |route| switch(route, *dark_mode, toggle_theme.clone())} />
                <Redirect<Route> to={initial_route} />
            </BrowserRouter>
        </div>
    }
}

fn get_saved_theme() -> bool {
    if let Some(window) = window() {
        if let Ok(Some(storage)) = window.local_storage() {
            if let Ok(Some(theme)) = storage.get_item(STORAGE_KEY_THEME) {
                return theme == THEME_DARK;
            }
        }
    }
    false
}
fn save_theme(is_dark: bool) {
    if let Some(window) = window() {
        if let Ok(Some(storage)) = window.local_storage() {
            let _ = storage.set_item(
                STORAGE_KEY_THEME,
                if is_dark { THEME_DARK } else { THEME_LIGHT },
            );
        }
    }
}
