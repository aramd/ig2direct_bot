use crate::error::Error;
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct Secrets {
    pub tg_token: String,
    pub ig_username: String,
    pub ig_password: String,
}

impl Secrets {
    pub fn new() -> Result<Self, Error> {
        let file_content = fs::read_to_string("./Secrets.toml")?;
        let secrets: Secrets = toml::from_str(&file_content)?;
        Ok(secrets)
    }
}
