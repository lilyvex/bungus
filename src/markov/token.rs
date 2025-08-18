use serde::{Deserialize, Serialize};

use crate::error::{BungusError, InternalError};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Token {
    pub text: String,
    pub weight: u64,
    pub bias: u64,
    pub children: Vec<Token>
}

impl Token {
    pub fn new(text: String, weight: u64, bias: u64, children: Vec<Token>) -> Self {
        Token {
            text,
            weight,
            bias,
            children
        }
    }

    pub async fn search_by_text(&mut self, text: &str) -> Result<&mut Token, BungusError> {
        Ok(Box::pin(async move {
            if self.text == text {
                return Ok(self);
            }

            for child in &mut self.children {
                if let Ok(found) = child.search_by_text(text).await {
                    return Ok(found);
                }
            }

            return Err(BungusError::InternalError(InternalError::TokenNotFound))
        }).await?)
    }

    pub fn add_to_children(&mut self, child: Token) {
        self.children.push(child);
    }

    pub fn set_token_weight(&mut self, weight: u64) {
        self.weight = weight;
    }

    pub fn generate_token_chain(&self) -> Vec<Token> {
        vec![]
    }

    pub async fn json(&mut self) -> Result<String, BungusError> {
        Ok(serde_json::to_string(self)?)
    }

    pub async fn from_json(json: String) -> Result<Token, BungusError> {
        Ok(serde_json::from_str(&json)?)
    }
}