use yew::prelude::*;
use crate::components::topbar::Topbar;
use crate::components::sidebar::Sidebar;
use web_sys::window;
use crate::constants::{STORAGE_KEY_THEME, THEME_DARK, THEME_LIGHT};

#[derive(Properties, PartialEq)]
pub struct AccessLayoutProps {
    pub children: Children,
}


#[function_component(AccessLayout)]
pub fn access_layout(props: &AccessLayoutProps) -> Html {
    let sidebar_open = use_state(|| false);
    let dark_mode = use_state(|| get_saved_theme());
    let close_sidebar = {
        let sidebar_open = sidebar_open.clone();
        Callback::from(move |_: MouseEvent| {
            sidebar_open.set(false);
        })
    };
    let open_sidebar = {
        let sidebar_open = sidebar_open.clone();
        Callback::from(move |_: MouseEvent| {
            sidebar_open.set(true);
        })
    };
    let toggle_theme = {
        let dark_mode = dark_mode.clone();
        Callback::from(move |_| {
            let new_theme = !*dark_mode;
            dark_mode.set(new_theme);
            save_theme(new_theme);
        })
    };
    let overlay_class = if *sidebar_open { "sidebar-overlay active" } else { "sidebar-overlay" };
    let theme_class = if *dark_mode { "dark-theme" } else { "" };
    html! {
        <div class={theme_class}>
            <Topbar
                dark_mode={*dark_mode}
                toggle_theme={toggle_theme.reform(|e: MouseEvent| e)}
                is_logged_in={true}
                on_sidebar_toggle={open_sidebar}
            />
            <div class={overlay_class} onclick={close_sidebar.clone()}></div>
            <Sidebar sidebar_open={*sidebar_open} close_sidebar={close_sidebar} />
            <main class="main-content">
                { for props.children.iter() }
            </main>
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
