use crate::router::router::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SidebarProps {
    pub sidebar_open: bool,
    pub close_sidebar: Callback<MouseEvent>,
}

#[function_component(Sidebar)]
pub fn sidebar(props: &SidebarProps) -> Html {
    let sidebar_class = if props.sidebar_open {
        "sidebar active"
    } else {
        "sidebar"
    };
    html! {
        <aside class={sidebar_class}>
            <div class="sidebar-header">
                <div class="sidebar-title">{"Navigation"}</div>
                <div class="sidebar-subtitle">{"Access Panel"}</div>
            </div>
            <nav class="sidebar-nav">
                <div class="nav-section">
                    <div class="nav-section-title">{"Access Management"}</div>
                    <Link<Route> to={Route::Access} classes="nav-item">{"Request Access"}</Link<Route>>
                    <Link<Route> to={Route::ConnectedUsers} classes="nav-item">{"Connected Users"}</Link<Route>>
                    <Link<Route> to={Route::Viewers} classes="nav-item">{"Viewers"}</Link<Route>>
                    <Link<Route> to={Route::IncomingRequests} classes="nav-item">{"Incoming Requests"}</Link<Route>>
                    <Link<Route> to={Route::OutgoingRequests} classes="nav-item">{"Outgoing Requests"}</Link<Route>>
                </div>
            </nav>
        </aside>
    }
}
