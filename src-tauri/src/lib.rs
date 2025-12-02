mod api;
mod constants; // <-- Added this line

use api::access::UserResponse;
use api::auth::{LoginResponse, LogoutResponse, SignupResponse}; // Import UserResponse

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn login(email: String, password: String) -> Result<LoginResponse, String> {
    api::auth::login(email, password).await
}

#[tauri::command]
async fn signup(email: String) -> Result<SignupResponse, String> {
    api::auth::signup(email).await
}

#[tauri::command]
async fn logout() -> Result<LogoutResponse, String> {
    api::auth::logout().await
}

#[tauri::command]
async fn get_connected_users(token: String) -> Result<Vec<UserResponse>, String> {
    api::access::get_connected_users(token).await
}

#[tauri::command]
async fn get_viewers(token: String) -> Result<Vec<UserResponse>, String> {
    api::access::get_viewers(token).await
}

#[tauri::command]
async fn request_access(token: String, target_user_id: String) -> Result<(), String> {
    api::access::request_access(token, target_user_id).await
}

#[tauri::command]
async fn get_incoming_requests(token: String) -> Result<Vec<UserResponse>, String> {
    api::access::get_incoming_requests(token).await
}

#[tauri::command]
async fn get_outgoing_requests(token: String) -> Result<Vec<UserResponse>, String> {
    api::access::get_outgoing_requests(token).await
}

#[tauri::command]
async fn accept_request(token: String, access_id: String) -> Result<(), String> {
    api::access::accept_request(token, access_id).await
}

#[tauri::command]
async fn reject_request(token: String, access_id: String) -> Result<(), String> {
    api::access::reject_request(token, access_id).await
}

#[tauri::command]
async fn cancel_request(token: String, access_id: String) -> Result<(), String> {
    api::access::cancel_request(token, access_id).await
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            login,
            signup,
            logout,
            get_connected_users,
            get_viewers,
            request_access,
            get_incoming_requests,
            get_outgoing_requests,
            accept_request,
            reject_request,
            cancel_request
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
