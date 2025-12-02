use crate::components::access_layout::AccessLayout;
use crate::services::api::{self, UserResponse};
use crate::utils::auth::get_auth_token;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[function_component(ConnectedUsers)]
pub fn connected_users() -> Html {
    let connected_users = use_state(|| Vec::<UserResponse>::new());
    let loading = use_state(|| false);
    let error = use_state(|| Option::<String>::None);

    let refresh = {
        let connected_users = connected_users.clone();
        let loading = loading.clone();
        let error = error.clone();
        Callback::from(move |_| {
            let connected_users = connected_users.clone();
            let loading = loading.clone();
            let error = error.clone();
            loading.set(true);
            spawn_local(async move {
                if let Some(token) = get_auth_token() {
                    match api::get_connected_users(token).await {
                        Ok(users) => {
                            connected_users.set(users);
                            error.set(None);
                        }
                        Err(e) => error.set(Some(format!("Failed to load connected users: {}", e))),
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

    html! {
        <AccessLayout>
            <section>
                <h2>{"Connected Users"}</h2>
                {if *loading {
                    html! { <p>{"Loading connected users..."}</p> }
                } else if let Some(err) = &*error {
                    html! { <p style="color: red;">{err}</p> }
                } else if connected_users.is_empty() {
                    html! { <p>{"No connected users found."}</p> }
                } else {
                    html! {
                        <div class="connected-users-grid">
                            { for connected_users.iter().map(|user| {
                                html! {
                                    <div class="user-card">
                                        <div class="user-card-header">
                                            <h3 class="user-name">{&user.name}</h3>
                                        </div>
                                        <div class="user-card-body">
                                            <p class="user-detail-id">{"ID: "}{&user.id}</p>
                                            <p class="user-detail-email">{"Email: "}{&user.email}</p>
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