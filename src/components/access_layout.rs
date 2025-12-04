use yew::prelude::*;
use crate::components::topbar::Topbar;
use crate::components::sidebar::Sidebar;

#[derive(Properties, PartialEq)]
pub struct AccessLayoutProps {
    pub children: Children,
    pub dark_mode: bool,
    pub toggle_theme: Callback<MouseEvent>,
}


#[function_component(AccessLayout)]
pub fn access_layout(props: &AccessLayoutProps) -> Html {
    let sidebar_open = use_state(|| false);
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
    let overlay_class = if *sidebar_open { "sidebar-overlay active" } else { "sidebar-overlay" };
    let theme_class = if props.dark_mode { "dark-theme" } else { "" };
    html! {
        <div class={theme_class}>
            <Topbar
                dark_mode={props.dark_mode}
                toggle_theme={props.toggle_theme.clone()}
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
