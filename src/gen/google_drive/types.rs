use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Token {
    pub refresh_token: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub refresh_token: String,
    pub client_id: String,
    pub client_secret: String,
}
