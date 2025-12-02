use crate::components::access_layout::AccessLayout;
use crate::services::api::{self, UserResponse};
use crate::utils::auth::get_auth_token;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[function_component(IncomingRequests)]
pub fn incoming_requests() -> Html {
    let requests = use_state(|| Vec::<UserResponse>::new());
    let loading = use_state(|| false);
    let error = use_state(|| Option::<String>::None);

    let refresh = {
        let requests = requests.clone();
        let loading = loading.clone();
        let error = error.clone();
        Callback::from(move |_| {
            let requests = requests.clone();
            let loading = loading.clone();
            let error = error.clone();
            loading.set(true);
            spawn_local(async move {
                if let Some(token) = get_auth_token() {
                    match api::get_incoming_requests(token).await {
                        Ok(rs) => {
                            requests.set(rs);
                            error.set(None);
                        }
                        Err(e) => error.set(Some(format!("Failed to load: {}", e))),
                    }
                } else {
                    error.set(Some("Not authenticated".to_string()));
                }
                loading.set(false);
            });
        })
    };

    use_effect_with((), {
        let refresh = refresh.clone();
        move |_| {
            refresh.emit(());
            || ()
        }
    });

    let on_accept = {
        let loading = loading.clone();
        let error = error.clone();
        let refresh = refresh.clone();
        Callback::from(move |id: String| {
            let loading = loading.clone();
            let error = error.clone();
            let refresh = refresh.clone();
            loading.set(true);
            spawn_local(async move {
                if let Some(token) = get_auth_token() {
                    match api::accept_request(token, id.clone()).await {
                        Ok(_) => {
                            refresh.emit(());
                        }
                        Err(e) => error.set(Some(format!("Failed to accept: {}", e))),
                    }
                } else {
                    error.set(Some("Not authenticated".to_string()));
                }
                loading.set(false);
            });
        })
    };
    let on_reject = {
        let loading = loading.clone();
        let error = error.clone();
        let refresh = refresh.clone();
        Callback::from(move |id: String| {
            let loading = loading.clone();
            let error = error.clone();
            let refresh = refresh.clone();
            loading.set(true);
            spawn_local(async move {
                if let Some(token) = get_auth_token() {
                    match api::reject_request(token, id.clone()).await {
                        Ok(_) => {
                            refresh.emit(());
                        }
                        Err(e) => error.set(Some(format!("Failed to reject: {}", e))),
                    }
                } else {
                    error.set(Some("Not authenticated".to_string()));
                }
                loading.set(false);
            });
        })
    };

    html! {
        <AccessLayout>
        <section>
            <h2>{"Incoming Requests"}</h2>
            {if *loading {
                html! { <p>{"Loading incoming requests..."}</p> }
            } else if let Some(err) = &*error {
                html! { <p style="color: red;">{err}</p> }
            } else if requests.is_empty() {
                html! { <p>{"No incoming requests found."}</p> }
            } else {
                html! {
                    <div class="connected-users-grid"> // Reusing the same grid style
                        { for requests.iter().map(|req| {
                            let id_accept = req.id.clone();
                            let id_reject = req.id.clone();
                            let on_accept = on_accept.clone();
                            let on_reject = on_reject.clone();
                            html! {
                                <div class="user-card"> // Reusing the same card style
                                    <div class="user-card-header">
                                        <h3 class="user-name">{&req.name}</h3>
                                    </div>
                                    <div class="user-card-body">
                                        <p class="user-detail-id">{"ID: "}{&req.id}</p>
                                        <p class="user-detail-email">{"Email: "}{&req.email}</p>
                                    </div>
                                    <div class="user-card-footer button-group"> // Added button-group for styling
                                        <button
                                            onclick={Callback::from(move |_| on_accept.emit(id_accept.clone()))}
                                            class="btn-accept"
                                        >
                                            {"Accept"}
                                        </button>
                                        <button
                                            onclick={Callback::from(move |_| on_reject.emit(id_reject.clone()))}
                                            class="btn-reject"
                                        >
                                            {"Reject"}
                                        </button>
                                    </div>
                                </div>
                            }
                        }) }
                    </div>
                }
            }}
        </section>
        </AccessLayout>
    }}

