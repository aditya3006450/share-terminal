use crate::constants::{build_url, endpoints};
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct UserResponse {
    pub id: String,
    pub name: String,
    pub email: String, // Added email field for display
}

pub async fn get_connected_users(token: String) -> Result<Vec<UserResponse>, String> {
    let client = reqwest::Client::new();
    let response = client
        .get(&build_url(endpoints::ACCESS_CONNECTIONS))
        .header(AUTHORIZATION, format!("Bearer {}", token))
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if !response.status().is_success() {
        return Err(format!(
            "Failed to get connected users: {}",
            response.status()
        ));
    }

    let users_map: std::collections::HashMap<String, Vec<UserResponse>> = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    // The Java backend returns Map<String, List<UserResponse>>, so extract the "connections" key
    users_map
        .get("connections")
        .cloned() // Clone the Vec<UserResponse>
        .ok_or_else(|| "Response did not contain 'connections' key".to_string())
}

pub async fn get_viewers(token: String) -> Result<Vec<UserResponse>, String> {
    let client = reqwest::Client::new();
    let response = client
        .get(&build_url(endpoints::ACCESS_VIEWERS))
        .header(AUTHORIZATION, format!("Bearer {}", token))
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Failed to get viewers: {}", response.status()));
    }

    let viewers_map: std::collections::HashMap<String, Vec<UserResponse>> =
        response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

    // The Java backend returns Map<String, List<UserResponse>>, so extract the "viewers" key
    viewers_map
        .get("viewers")
        .cloned() // Clone the Vec<UserResponse>
        .ok_or_else(|| "Response did not contain 'viewers' key".to_string())
}

pub async fn request_access(token: String, target_user_id: String) -> Result<(), String> {
    let client = reqwest::Client::new();
    let url = format!(
        "{}/{}",
        build_url(endpoints::ACCESS_REQUEST),
        target_user_id
    );
    let response = client
        .post(&url)
        .header(AUTHORIZATION, format!("Bearer {}", token))
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Failed to request access: {}", response.status()));
    }
    // No specific JSON response is expected for success, just status 2xx
    Ok(())
}

pub async fn get_incoming_requests(token: String) -> Result<Vec<UserResponse>, String> {
    let client = reqwest::Client::new();
    let response = client
        .get(&build_url(endpoints::ACCESS_REQUESTS_INCOMING))
        .header(AUTHORIZATION, format!("Bearer {}", token))
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if !response.status().is_success() {
        return Err(format!(
            "Failed to get incoming requests: {}",
            response.status()
        ));
    }

    let requests_map: std::collections::HashMap<String, Vec<UserResponse>> = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    // The Java backend returns Map<String, List<UserResponse>>, so extract the "incomingRequests" key
    requests_map
        .get("incomingRequests")
        .cloned()
        .ok_or_else(|| "Response did not contain 'incomingRequests' key".to_string())
}

pub async fn get_outgoing_requests(token: String) -> Result<Vec<UserResponse>, String> {
    let client = reqwest::Client::new();
    let response = client
        .get(&build_url(endpoints::ACCESS_REQUESTS_OUTGOING))
        .header(AUTHORIZATION, format!("Bearer {}", token))
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if !response.status().is_success() {
        return Err(format!(
            "Failed to get outgoing requests: {}",
            response.status()
        ));
    }

    let requests_map: std::collections::HashMap<String, Vec<UserResponse>> = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    // The Java backend returns Map<String, List<UserResponse>>, so extract the "outgoingRequests" key
    requests_map
        .get("outgoingRequests")
        .cloned()
        .ok_or_else(|| "Response did not contain 'outgoingRequests' key".to_string())
}

pub async fn accept_request(token: String, access_id: String) -> Result<(), String> {
    let client = reqwest::Client::new();
    let url = format!(
        "{}/{}/accept",
        build_url(endpoints::ACCESS_REQUESTS_ACCEPT),
        access_id
    );
    let response = client
        .post(&url)
        .header(AUTHORIZATION, format!("Bearer {}", token))
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Failed to accept request: {}", response.status()));
    }
    Ok(())
}

pub async fn reject_request(token: String, access_id: String) -> Result<(), String> {
    let client = reqwest::Client::new();
    let url = format!(
        "{}/{}/reject",
        build_url(endpoints::ACCESS_REQUESTS_REJECT),
        access_id
    );
    let response = client
        .post(&url)
        .header(AUTHORIZATION, format!("Bearer {}", token))
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Failed to reject request: {}", response.status()));
    }
    Ok(())
}



