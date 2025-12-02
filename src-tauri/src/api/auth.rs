use crate::constants::{build_url, endpoints};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct SignupRequest {
    pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct SignupResponse {
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct LogoutResponse {
    pub message: String,
}

pub async fn login(email: String, password: String) -> Result<LoginResponse, String> {
    let client = reqwest::Client::new();
    let login_data = LoginRequest { email, password };

    let response = client
        .post(&build_url(endpoints::AUTH_LOGIN))
        .json(&login_data)
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Login failed with status: {}", response.status()));
    }

    let login_response = response
        .json::<LoginResponse>()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    Ok(login_response)
}

pub async fn signup(email: String) -> Result<SignupResponse, String> {
    let client = reqwest::Client::new();
    let signup_data = SignupRequest { email };

    let response = client
        .post(&build_url(endpoints::AUTH_SIGNUP))
        .json(&signup_data)
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Signup failed with status: {}", response.status()));
    }

    let signup_response = response
        .json::<SignupResponse>()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    Ok(signup_response)
}

pub async fn logout() -> Result<LogoutResponse, String> {
    // Simulate a successful logout
    Ok(LogoutResponse {
        message: "Logged out successfully!".to_string(),
    })
}
