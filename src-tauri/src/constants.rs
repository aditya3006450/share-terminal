// API Configuration
pub const API_BASE_URL: &str =
    "https://share-termial-hcdmbfd8gwb8ehb6.centralindia-01.azurewebsites.net";

// API Endpoints
pub mod endpoints {
    pub const AUTH_LOGIN: &str = "/auth/login";
    pub const AUTH_SIGNUP: &str = "/auth/sign-up";
    pub const ACCESS_CONNECTIONS: &str = "/access/connections";
    pub const ACCESS_VIEWERS: &str = "/access/viewers";
    pub const ACCESS_REQUEST: &str = "/access/request";
    pub const ACCESS_REQUESTS_INCOMING: &str = "/access/requests/incoming";
    pub const ACCESS_REQUESTS_OUTGOING: &str = "/access/requests/outgoing";
    pub const ACCESS_REQUESTS_ACCEPT: &str = "/access/requests"; // accessId and /accept will be appended
    pub const ACCESS_REQUESTS_REJECT: &str = "/access/requests"; // accessId and /reject will be appended
    pub const ACCESS_CANCEL_REQUEST: &str = "/access/requests"; // accessId and /cancel will be appended
    pub const SIGNAL_SEND: &str = "/signal/send";
    pub const SIGNAL_INBOX: &str = "/signal/inbox";
}

pub fn build_url(endpoint: &str) -> String {
    format!("{}{}", API_BASE_URL, endpoint)
}
