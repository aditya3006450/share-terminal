use web_sys::window;
use crate::constants::STORAGE_KEY_AUTH_TOKEN;

/// Get the authentication token from localStorage
pub fn get_auth_token() -> Option<String> {
    if let Some(window) = window() {
        if let Ok(Some(storage)) = window.local_storage() {
            if let Ok(Some(token)) = storage.get_item(STORAGE_KEY_AUTH_TOKEN) {
                return Some(token);
            }
        }
    }
    None
}

/// Store the authentication token in localStorage
pub fn store_auth_token(token: &str) {
    if let Some(window) = window() {
        if let Ok(Some(storage)) = window.local_storage() {
            let _ = storage.set_item(STORAGE_KEY_AUTH_TOKEN, token);
        }
    }
}

/// Remove the authentication token from localStorage
pub fn clear_auth_token() {
    if let Some(window) = window() {
        if let Ok(Some(storage)) = window.local_storage() {
            let _ = storage.remove_item(STORAGE_KEY_AUTH_TOKEN);
        }
    }
}

/// Check if user is authenticated (has a token)
pub fn is_authenticated() -> bool {
    get_auth_token().is_some()
}
