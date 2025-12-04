use crate::components::access_layout::AccessLayout;
use crate::services::api::{self, UserResponse};
use crate::utils::auth::get_auth_token;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct OutgoingRequestsProps {
    pub dark_mode: bool,
    pub toggle_theme: Callback<()>,
}

#[function_component(OutgoingRequests)]
pub fn outgoing_requests(props: &OutgoingRequestsProps) -> Html {
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
                    match api::get_outgoing_requests(token).await {
                        Ok(rs) => {
                            requests.set(rs);
                            error.set(None);
                        }
                        Err(e) => error.set(Some(format!("Failed to load outgoing requests: {}", e))),
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

    let on_cancel = {
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
                    match api::cancel_request(token, id.clone()).await {
                        Ok(_) => {
                            refresh.emit(());
                        }
                        Err(e) => error.set(Some(format!("Failed to cancel request: {}", e))),
                    }
                } else {
                    error.set(Some("Not authenticated".to_string()));
                }
                loading.set(false);
            });
        })
    };

    html! {
        <AccessLayout dark_mode={props.dark_mode} toggle_theme={props.toggle_theme.reform(|_| ())}>
        <section>
            <h2>{"Outgoing Requests"}</h2>
            {if *loading {
                html! { <p>{"Loading outgoing requests..."}</p> }
            } else if let Some(err) = &*error {
                html! { <p style="color: red;">{err}</p> }
            } else if requests.is_empty() {
                html! { <p>{"No outgoing requests found."}</p> }
            } else {
                html! {
                    <div class="connected-users-grid"> // Reusing the same grid style
                        { for requests.iter().map(|req| {
                            let req_id = req.id.clone();
                            let on_cancel = on_cancel.clone();
                            html! {
                                <div class="user-card"> // Reusing the same card style
                                    <div class="user-card-header">
                                        <h3 class="user-name">{&req.name}</h3>
                                    </div>
                                    <div class="user-card-body">
                                        <p class="user-detail-id">{"ID: "}{&req.id}</p>
                                        <p class="user-detail-email">{"Email: "}{&req.email}</p>
                                    </div>
                                    <div class="user-card-footer">
                                        <button
                                            onclick={Callback::from(move |_| on_cancel.emit(req_id.clone()))}
                                            class="btn-cancel"
                                        >
                                            {"Cancel Request"}
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
    }
}