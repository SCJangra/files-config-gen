mod server;
mod types;

use crate::scan;
use anyhow::Context;
use reqwest::{blocking::Client, Url};
use types::*;

const AUTH_URI: &str = "https://accounts.google.com/o/oauth2/auth";
const TOKEN_URI: &str = "https://oauth2.googleapis.com/token";
const RESPONSE_TYPE: &str = "code";
const SCOPE: &str = "https://www.googleapis.com/auth/drive";
const PORT: u16 = 8000;
const REDIRECT_URI: &str = "http://localhost:8000";

pub fn gd() -> anyhow::Result<String> {
    let client_id = scan!("Client ID: ", String)?;
    let client_secret = scan!("Client Secret: ", String)?;

    open::that_in_background(get_auth_uri(&client_id)?);

    let (s, r) = std::sync::mpsc::sync_channel::<String>(1);

    server::start(s, PORT)?;

    let code = r.recv().with_context(|| "Could not get auth code")?;

    let res = Client::new()
        .post(TOKEN_URI)
        .form(&[
            ("grant_type", "authorization_code"),
            ("code", code.as_str()),
            ("client_id", client_id.as_str()),
            ("client_secret", client_secret.as_str()),
            ("redirect_uri", REDIRECT_URI),
        ])
        .send()
        .with_context(|| "Could not send request to get access token")?
        .text()
        .with_context(|| "Could not get access token")?;

    let token: Token =
        serde_json::from_str(&res).with_context(|| "Could not deserialize access token")?;

    let config = serde_json::to_string_pretty(&Config {
        refresh_token: token.refresh_token,
        client_id,
        client_secret,
    })
    .with_context(|| "Could not serialize config")?;

    Ok(config)
}

fn get_auth_uri(client_id: &str) -> anyhow::Result<String> {
    let u = Url::parse_with_params(
        AUTH_URI,
        &[
            ("client_id", client_id),
            ("redirect_uri", REDIRECT_URI),
            ("response_type", RESPONSE_TYPE),
            ("scope", SCOPE),
        ],
    )
    .with_context(|| "Could not create auth url!")?
    .to_string();

    Ok(u)
}
