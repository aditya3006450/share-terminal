use crate::components::access_layout::AccessLayout;
use crate::services::api::{fetch_inbox, get_connected_users, send_signal};
use crate::utils::auth::get_auth_token;
use gloo::timers::callback::Interval;
use std::collections::HashMap;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ScreenShareRequestsProps {
    pub dark_mode: bool,
    pub toggle_theme: Callback<()>,
}

#[derive(Clone, PartialEq)]
struct ScreenShareRequest {
    from_user_id: String,
    from_user_name: String,
}

#[function_component(ScreenShareRequests)]
pub fn screen_share_requests(props: &ScreenShareRequestsProps) -> Html {
    let requests = use_state(|| Vec::<ScreenShareRequest>::new());
    let connected_users = use_state(|| HashMap::<String, String>::new()); // ID -> Name map
    let loading = use_state(|| false);
    let error = use_state(|| Option::<String>::None);

    // Fetch connected users on mount
    {
        let connected_users = connected_users.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                if let Some(token) = get_auth_token() {
                    match get_connected_users(token).await {
                        Ok(users) => {
                            let mut map = HashMap::new();
                            for user in users {
                                map.insert(user.id, user.name);
                            }
                            connected_users.set(map);
                        }
                        Err(e) => log::error!("Failed to fetch connected users: {}", e),
                    }
                }
            });
            || ()
        });
    }

    // Poll for incoming screen share requests
    {
        let requests = requests.clone();
        let connected_users = connected_users.clone();
        let error = error.clone();

        use_effect_with((), move |_| {
            let requests = requests.clone();
            let connected_users = connected_users.clone();
            let error = error.clone();

            let interval = Interval::new(2000, move || {
                let requests = requests.clone();
                let connected_users = connected_users.clone();
                let error = error.clone();

                spawn_local(async move {
                    if let Some(token) = get_auth_token() {
                        match fetch_inbox(token).await {
                            Ok(messages) => {
                                let mut screen_requests = Vec::new();

                                for msg in messages {
                                    // Look for "request_screen_share" signals
                                    if msg.signal_type == "request_screen_share" {
                                        // Look up name from connected users map
                                        let name = connected_users
                                            .get(&msg.from_user_id)
                                            .cloned()
                                            .unwrap_or_else(|| "Unknown User".to_string());

                                        screen_requests.push(ScreenShareRequest {
                                            from_user_id: msg.from_user_id,
                                            from_user_name: name,
                                        });
                                    }
                                }

                                if !screen_requests.is_empty() {
                                    requests.set(screen_requests);
                                }
                            }
                            Err(e) => {
                                log::error!("Failed to fetch inbox: {:?}", e);
                                error.set(Some(format!("Failed to check for requests: {}", e)));
                            }
                        }
                    }
                });
            });

            move || {
                drop(interval);
            }
        });
    }

    let on_approve = {
        let requests = requests.clone();
        let error = error.clone();

        Callback::from(move |user_id: String| {
            let requests = requests.clone();
            let error = error.clone();
            let user_id_clone = user_id.clone();

            spawn_local(async move {
                if let Some(token) = get_auth_token() {
                    // Send approval signal
                    let mut payload = HashMap::new();
                    payload.insert("approved".to_string(), serde_json::json!(true));

                    match send_signal(
                        token,
                        user_id_clone.clone(),
                        "screen_share_approved".to_string(),
                        serde_json::json!(payload),
                    )
                    .await
                    {
                        Ok(_) => {
                            log::info!("Screen share approved for user: {}", user_id_clone);
                            // Remove the request from the list
                            requests.set(
                                (*requests)
                                    .iter()
                                    .filter(|r| r.from_user_id != user_id_clone)
                                    .cloned()
                                    .collect(),
                            );
                        }
                        Err(e) => {
                            log::error!("Failed to send approval: {:?}", e);
                            error.set(Some(format!("Failed to approve: {}", e)));
                        }
                    }
                }
            });
        })
    };

    let on_reject = {
        let requests = requests.clone();
        let error = error.clone();

        Callback::from(move |user_id: String| {
            let requests = requests.clone();
            let error = error.clone();
            let user_id_clone = user_id.clone();

            spawn_local(async move {
                if let Some(token) = get_auth_token() {
                    // Send rejection signal
                    let mut payload = HashMap::new();
                    payload.insert("approved".to_string(), serde_json::json!(false));

                    match send_signal(
                        token,
                        user_id_clone.clone(),
                        "screen_share_rejected".to_string(),
                        serde_json::json!(payload),
                    )
                    .await
                    {
                        Ok(_) => {
                            log::info!("Screen share rejected for user: {}", user_id_clone);
                            // Remove the request from the list
                            requests.set(
                                (*requests)
                                    .iter()
                                    .filter(|r| r.from_user_id != user_id_clone)
                                    .cloned()
                                    .collect(),
                            );
                        }
                        Err(e) => {
                            log::error!("Failed to send rejection: {:?}", e);
                            error.set(Some(format!("Failed to reject: {}", e)));
                        }
                    }
                }
            });
        })
    };

    html! {
        <AccessLayout dark_mode={props.dark_mode} toggle_theme={props.toggle_theme.reform(|_| ())}>
            <section>
                <h2>{"Screen Share Requests"}</h2>
                <p>{"Users requesting to view your screen"}</p>

                {if *loading {
                    html! { <p>{"Loading requests..."}</p> }
                } else if let Some(err) = &*error {
                    html! { <p style="color: red;">{err}</p> }
                } else if requests.is_empty() {
                    html! {
                        <div class="no-requests">
                            <p>{"No pending screen share requests."}</p>
                            <p class="hint">{"Requests will appear here when someone wants to view your screen."}</p>
                        </div>
                    }
                } else {
                    html! {
                        <div class="connected-users-grid">
                            { for requests.iter().map(|req| {
                                let user_id_approve = req.from_user_id.clone();
                                let user_id_reject = req.from_user_id.clone();
                                let on_approve = on_approve.clone();
                                let on_reject = on_reject.clone();

                                html! {
                                    <div class="user-card screen-share-request">
                                        <div class="user-card-header">
                                            <h3 class="user-name">{&req.from_user_name}</h3>
                                            <span class="request-badge">{"🔔 Screen Share Request"}</span>
                                        </div>
                                        <div class="user-card-body">
                                            <p class="user-detail-id">{"User ID: "}{&req.from_user_id}</p>
                                            <p class="request-message">{"wants to view your screen"}</p>
                                        </div>
                                        <div class="user-card-footer button-group">
                                            <button
                                                onclick={Callback::from(move |_| on_approve.emit(user_id_approve.clone()))}
                                                class="btn-accept"
                                            >
                                                {"✓ Approve & Share"}
                                            </button>
                                            <button
                                                onclick={Callback::from(move |_| on_reject.emit(user_id_reject.clone()))}
                                                class="btn-reject"
                                            >
                                                {"✗ Reject"}
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
