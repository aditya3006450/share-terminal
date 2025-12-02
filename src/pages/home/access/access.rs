use crate::components::access_layout::AccessLayout;
use crate::services::api;
use crate::utils::auth::get_auth_token;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[function_component(Access)]
pub fn access() -> Html {
    let target_user_id_input = use_state(|| String::new());
    let request_access_message = use_state(|| Option::<String>::None);
    let on_target_user_id_input = {
        let target_user_id_input = target_user_id_input.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            target_user_id_input.set(input.value());
        })
    };
    let request_access_action = {
        let target_user_id_input = target_user_id_input.clone();
        let request_access_message = request_access_message.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let target_user_id = (*target_user_id_input).clone();
            let request_access_message = request_access_message.clone();
            if target_user_id.is_empty() {
                request_access_message.set(Some("Please enter a target user ID.".to_string()));
                return;
            }
            spawn_local(async move {
                if let Some(token) = get_auth_token() {
                    match api::request_access(token, target_user_id).await {
                        Ok(_) => {
                            request_access_message
                                .set(Some("Access request sent successfully!".to_string()));
                        }
                        Err(e) => {
                            request_access_message
                                .set(Some(format!("Failed to send access request: {}", e)));
                        }
                    }
                } else {
                    request_access_message
                        .set(Some("Not authenticated. Please log in.".to_string()));
                }
            });
        })
    };
    html! {
        <AccessLayout>
            <h1>{"Access Page"}</h1>
            <section>
                <h2>{"Request Access"}</h2>
                <div class="request-access-section">
                    <input
                        type="text"
                        placeholder="Enter Target User ID"
                        value={(*target_user_id_input).clone()}
                        oninput={on_target_user_id_input}
                        class="input-field"
                    />
                    <button onclick={request_access_action} class="submit-btn">{"Request Access"}</button>
                    {if let Some(msg) = &*request_access_message {
                        html! { <p>{msg}</p> }
                    } else {
                        html! { <></> }
                    }}
                </div>
            </section>
        </AccessLayout>
    }
}
