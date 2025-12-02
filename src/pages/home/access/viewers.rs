use crate::components::access_layout::AccessLayout;
use crate::services::api::{self, UserResponse};
use crate::utils::auth::get_auth_token;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[function_component(Viewers)]
pub fn viewers() -> Html {
    let viewers = use_state(|| Vec::<UserResponse>::new());
    let loading = use_state(|| false);
    let error = use_state(|| Option::<String>::None);

    let refresh = {
        let viewers = viewers.clone();
        let loading = loading.clone();
        let error = error.clone();
        Callback::from(move |_| {
            let viewers = viewers.clone();
            let loading = loading.clone();
            let error = error.clone();
            loading.set(true);
            spawn_local(async move {
                if let Some(token) = get_auth_token() {
                    match api::get_viewers(token).await {
                        Ok(vs) => {
                            viewers.set(vs);
                            error.set(None);
                        }
                        Err(e) => error.set(Some(format!("Failed to load viewers: {}", e))),
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
                <h2>{"Current Viewers"}</h2>
                {"Users who can use your device"}
                {if *loading {
                    html! { <p>{"Loading viewers..."}</p> }
                } else if let Some(err) = &*error {
                    html! { <p style="color: red;">{err}</p> }
                } else if viewers.is_empty() {
                    html! { <p>{"No viewers found."}</p> }
                } else {
                    html! {
                        <div class="connected-users-grid"> // Reusing the same grid style
                            { for viewers.iter().map(|viewer| {
                                html! {
                                    <div class="user-card"> // Reusing the same card style
                                        <div class="user-card-header">
                                            <h3 class="user-name">{&viewer.name}</h3>
                                        </div>
                                        <div class="user-card-body">
                                            <p class="user-detail-id">{"ID: "}{&viewer.id}</p>
                                            <p class="user-detail-email">{"Email: "}{&viewer.email}</p>
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

