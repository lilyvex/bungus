use serde::{Deserialize, Serialize};

use crate::error::BungusError;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Token {
    pub text: String,
    pub weight: u64,
    pub bias: f32,
    pub children: Vec<Token>
}

impl Token {
    pub async fn json(&mut self) -> Result<String, BungusError> {
        Ok(serde_json::to_string(self)?)
    }

    pub async fn from_json(json: String) -> Result<Token, BungusError> {
        Ok(serde_json::from_str(&json)?)
    }
}