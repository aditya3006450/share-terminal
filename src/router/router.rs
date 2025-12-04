use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::home::access::access::Access;
use crate::pages::home::access::connected_users::ConnectedUsers;
use crate::pages::home::access::incoming_requests::IncomingRequests;
use crate::pages::home::access::outgoing_requests::OutgoingRequests;
use crate::pages::home::access::viewers::Viewers;
use crate::pages::home::login::login::Login;
use crate::pages::home::signup::signup::SignUp;
use crate::pages::canvas::CanvasPage;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Login,
    #[at("/signup")]
    Signup,
    #[at("/access")]
    Access,
    #[at("/access/connected-users")]
    ConnectedUsers,
    #[at("/access/viewers")]
    Viewers,
    #[at("/access/incoming-requests")]
    IncomingRequests,
    #[at("/access/outgoing-requests")]
    OutgoingRequests,
    #[at("/canvas/:id")]
    Canvas { id: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route, dark_mode: bool, toggle_theme: Callback<()>) -> Html {
    match routes {
        Route::Login => html! { <Login /> },
        Route::Signup => html! { <SignUp /> },
        Route::Access => html! { <Access dark_mode={dark_mode} toggle_theme={toggle_theme} /> },
        Route::ConnectedUsers => html! { <ConnectedUsers dark_mode={dark_mode} toggle_theme={toggle_theme} /> },
        Route::Viewers => html! { <Viewers dark_mode={dark_mode} toggle_theme={toggle_theme} /> },
        Route::IncomingRequests => html! { <IncomingRequests dark_mode={dark_mode} toggle_theme={toggle_theme} /> },
        Route::OutgoingRequests => html! { <OutgoingRequests dark_mode={dark_mode} toggle_theme={toggle_theme} /> },
        Route::Canvas { id } => html! { <CanvasPage id={id} dark_mode={dark_mode} toggle_theme={toggle_theme} /> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}
