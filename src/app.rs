use crate::utils::auth::get_auth_token;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::home::access::access::Access;
use crate::pages::home::access::connected_users::ConnectedUsers;
use crate::pages::home::access::incoming_requests::IncomingRequests;
use crate::pages::home::access::outgoing_requests::OutgoingRequests;
use crate::pages::home::access::viewers::Viewers;
use crate::pages::home::login::login::Login;
use crate::pages::home::signup::signup::SignUp;
use crate::router::router::Route;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Login => html! { <Login/> },
        Route::Signup => html! { <SignUp/> },
        Route::Access => html! { <Access/> },
        Route::ConnectedUsers => html! { <ConnectedUsers/> },
        Route::Viewers => html! { <Viewers/> },
        Route::IncomingRequests => html! { <IncomingRequests/> },
        Route::OutgoingRequests => html! { <OutgoingRequests/> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let initial_route = if get_auth_token().is_some() {
        Route::Access
    } else {
        Route::Login
    };

    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
            <Redirect<Route> to={initial_route} />
        </BrowserRouter>
    }
}
