use js_sys::Boolean;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

// Request/Response types
#[derive(Serialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginResponse {
    pub token: String,
}
#[derive(Serialize)]
pub struct SignupRequest {
    pub email: String,
}

#[derive(Deserialize)]
pub struct SignupResponse {
    pub message: String,
}

#[derive(Serialize)]
pub struct AuthenticatedRequest {
    pub endpoint: String,
    pub token: String,
}

#[derive(Deserialize)]
pub struct LogoutResponse {
    pub message: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct UserResponse {
    pub id: String,
    pub name: String,
    pub email: String,
}

#[derive(Serialize)]
pub struct TokenArg {
    pub token: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestAccessArgs {
    pub token: String,
    pub target_user_id: String,
}

// API service functions
pub async fn login(email: String, password: String) -> Result<LoginResponse, String> {
    let args = serde_wasm_bindgen::to_value(&LoginRequest { email, password })
        .map_err(|e| format!("Failed to serialize: {}", e))?;

    let result = invoke("login", args).await;

    serde_wasm_bindgen::from_value(result).map_err(|e| format!("Login failed: {}", e))
}

pub async fn signup(email: String) -> Result<SignupResponse, String> {
    let args = serde_wasm_bindgen::to_value(&SignupRequest { email })
        .map_err(|e| format!("Failed to serialize: {}", e))?;

    let result = invoke("signup", args).await;

    serde_wasm_bindgen::from_value(result).map_err(|e| format!("Signup failed: {}", e))
}

pub async fn logout() -> Result<LogoutResponse, String> {
    let result = invoke("logout", JsValue::NULL).await;

    serde_wasm_bindgen::from_value(result).map_err(|e| format!("Logout failed: {}", e))
}

pub async fn get_connected_users(token: String) -> Result<Vec<UserResponse>, String> {
    let args = serde_wasm_bindgen::to_value(&TokenArg { token })
        .map_err(|e| format!("Failed to serialize token arg: {}", e))?;

    let result = invoke("get_connected_users", args).await;

    serde_wasm_bindgen::from_value(result)
        .map_err(|e| format!("Failed to get connected users: {}", e))
}

pub async fn get_viewers(token: String) -> Result<Vec<UserResponse>, String> {
    let args = serde_wasm_bindgen::to_value(&TokenArg { token })
        .map_err(|e| format!("Failed to serialize token arg: {}", e))?;

    let result = invoke("get_viewers", args).await;

    serde_wasm_bindgen::from_value(result).map_err(|e| format!("Failed to get viewers: {}", e))
}

pub async fn request_access(token: String, target_user_id: String) -> Result<(), String> {
    let args = serde_wasm_bindgen::to_value(&RequestAccessArgs {
        token,
        target_user_id,
    })
    .map_err(|e| format!("Failed to serialize request_access args: {}", e))?;

    let result = invoke("request_access", args).await;

    // Check if the result is undefined or null for a successful void return
    if result.is_undefined() || result.is_null() {
        Ok(())
    } else {
        // If there's an error, the backend might return a JsValue that can be deserialized into a String error
        serde_wasm_bindgen::from_value(result)
            .map(|s: String| Err(format!("Request access failed: {}", s)))
            .unwrap_or_else(|_| Err("Request access failed with unknown error".to_string()))
    }
}

pub async fn get_incoming_requests(token: String) -> Result<Vec<UserResponse>, String> {
    let args = serde_wasm_bindgen::to_value(&TokenArg { token })
        .map_err(|e| format!("Failed to serialize token arg: {}", e))?;

    let result = invoke("get_incoming_requests", args).await;

    serde_wasm_bindgen::from_value(result)
        .map_err(|e| format!("Failed to get incoming requests: {}", e))
}

pub async fn get_outgoing_requests(token: String) -> Result<Vec<UserResponse>, String> {
    let args = serde_wasm_bindgen::to_value(&TokenArg { token })
        .map_err(|e| format!("Failed to serialize token arg: {}", e))?;

    let result = invoke("get_outgoing_requests", args).await;

    serde_wasm_bindgen::from_value(result)
        .map_err(|e| format!("Failed to get outgoing requests: {}", e))
}


#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AcceptRejectRequestArgs {
    pub token: String,
    pub access_id: String,
}

pub async fn accept_request(token: String, access_id: String) -> Result<(), String> {
    let args = serde_wasm_bindgen::to_value(&AcceptRejectRequestArgs { token, access_id })
        .map_err(|e| format!("Failed to serialize accept_request args: {}", e))?;

    let result = invoke("accept_request", args).await;

    if result.is_undefined() || result.is_null() {
        Ok(())
    } else {
        serde_wasm_bindgen::from_value(result)
            .map(|s: String| Err(format!("Accept request failed: {}", s)))
            .unwrap_or_else(|_| Err("Accept request failed with unknown error".to_string()))
    }
}

pub async fn reject_request(token: String, access_id: String) -> Result<(), String> {
    let args = serde_wasm_bindgen::to_value(&AcceptRejectRequestArgs { token, access_id })
        .map_err(|e| format!("Failed to serialize reject_request args: {}", e))?;

    let result = invoke("reject_request", args).await;

    if result.is_undefined() || result.is_null() {
        Ok(())
    } else {
        serde_wasm_bindgen::from_value(result)
            .map(|s: String| Err(format!("Reject request failed: {}", s)))
            .unwrap_or_else(|_| Err("Reject request failed with unknown error".to_string()))
    }
}
