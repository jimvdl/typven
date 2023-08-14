#![allow(dead_code)]

use std::time::Duration;

use anyhow::{bail, Context};
use serde::{Deserialize, Serialize};

const CLIENT_ID: &str = "cd13a10db89e2100f7f2";
const SCOPE: &str = "repo";

#[derive(Debug, Deserialize)]
struct AccessRequest {
    device_code: String,
    user_code: String,
    verification_uri: String,
    expires_in: u64,
    interval: u64,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Poll {
    Ready(Authenticated),
    Pending(AuthState),
}

#[derive(Debug, Serialize, Deserialize)]
struct Authenticated {
    access_token: String,
    token_type: String,
    scope: String,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "error", rename_all = "snake_case")]
enum AuthState {
    AuthorizationPending,
    IncorrectDeviceCode,
}

pub fn login() -> anyhow::Result<()> {
    let access_request = request_access()?;

    println!("device activation code: {}", access_request.user_code);
    println!("hit enter to open the device authentication url");
    std::io::stdin().read_line(&mut String::new())?;

    if let Err(_) = open::that(&access_request.verification_uri) {
        super::print_error("failed to open device authentication url. open manually: https://github.com/login/device")?;
    }

    let auth = poll_blocking(access_request)?;

    let home_dir = dirs::home_dir().expect("home directory access denied");
    let tm_config = home_dir.join(".tm_config");
    std::fs::write(tm_config, toml::to_string(&auth)?.as_bytes())?;

    println!("you've been logged in");

    Ok(())
}

fn request_access() -> anyhow::Result<AccessRequest> {
    match ureq::post("https://github.com/login/device/code")
        .set("Accept", "application/json")
        .query("client_id", CLIENT_ID)
        .query("scope", SCOPE)
        .call()
    {
        Ok(response) => Ok(response.into_json()?),
        Err(_) => bail!("network failed"),
    }
}

fn poll_blocking(access_request: AccessRequest) -> anyhow::Result<Authenticated> {
    let agent = ureq::Agent::new();

    loop {
        std::thread::sleep(Duration::from_secs(access_request.interval));

        let poll: Poll = agent
            .post("https://github.com/login/oauth/access_token")
            .set("Accept", "application/json")
            .query("client_id", CLIENT_ID)
            .query("device_code", &access_request.device_code)
            .query("grant_type", "urn:ietf:params:oauth:grant-type:device_code")
            .call()
            .context("network failed")?
            .into_json()?;

        match poll {
            Poll::Ready(auth) => return Ok(auth),
            Poll::Pending(err) => match err {
                AuthState::AuthorizationPending => {}
                AuthState::IncorrectDeviceCode => {
                    super::print_error(
                        "failed to authenticate, timed out or device code was incorrect.",
                    )?;
                }
            },
        }
    }
}
