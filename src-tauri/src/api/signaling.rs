use crate::constants::{build_url, endpoints};
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SignalMessage {
    #[serde(rename = "fromUserId")]
    pub from_user_id: String,
    #[serde(rename = "toUserId")]
    pub to_user_id: String,
    #[serde(rename = "type")]
    pub signal_type: String,
    pub payload: HashMap<String, serde_json::Value>,
}

pub async fn send_signal(
    token: String,
    to_user_id: String,
    signal_type: String,
    payload: serde_json::Value,
) -> Result<(), String> {
    let client = reqwest::Client::new();

    let response = client
        .post(&build_url(endpoints::SIGNAL_SEND))
        .header(AUTHORIZATION, format!("Bearer {}", token))
        .header(CONTENT_TYPE, "application/json")
        .json(&serde_json::json!({
            "toUserId": to_user_id,
            "type": signal_type,
            "payload": payload
        }))
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(format!("Failed to send signal: {} - {}", status, body));
    }

    Ok(())
}

pub async fn fetch_inbox(token: String) -> Result<Vec<SignalMessage>, String> {
    let client = reqwest::Client::new();

    let response = client
        .get(&build_url(endpoints::SIGNAL_INBOX))
        .header(AUTHORIZATION, format!("Bearer {}", token))
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(format!("Failed to fetch inbox: {} - {}", status, body));
    }

    // Get the response text first for debugging
    let response_text = response
        .text()
        .await
        .map_err(|e| format!("Failed to read response: {}", e))?;

    // Try to parse as a map with "messages" key
    let response_json: serde_json::Value = serde_json::from_str(&response_text)
        .map_err(|e| format!("Failed to parse JSON: {} - Response: {}", e, response_text))?;

    // Extract the messages array
    let messages = response_json
        .get("messages")
        .ok_or_else(|| format!("Missing 'messages' field in response: {}", response_text))?;

    // Deserialize the messages
    let messages_vec: Vec<SignalMessage> =
        serde_json::from_value(messages.clone()).map_err(|e| {
            format!(
                "Failed to deserialize messages: {} - Messages: {}",
                e, messages
            )
        })?;

    Ok(messages_vec)
}
