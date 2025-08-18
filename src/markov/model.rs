use std::collections::HashMap;
use std::path::PathBuf;
use poise::serenity_prelude::{GetMessages, Guild, GuildChannel, GuildId, Http, Message};
use rand::Rng;

use crate::error::BungusError;
use crate::markov::scanner::index_channels;
use crate::markov::token::Token;

pub struct BungusModel {
    max_token_output: usize,
    max_token_relationships: usize,
    total_tokens: usize,
    rng: rand::rngs::ThreadRng,
    rng_weight_bias: i64,
    rng_direction_bias: f32,
    token_tree: Token
}

impl BungusModel {
    pub fn new(max_token_output: usize, max_token_relationships: usize) -> Self {
        let mut rng = rand::rng();

        BungusModel {
            max_token_output,
            max_token_relationships,
            total_tokens: usize::default(),
            rng: rng.clone(),
            rng_weight_bias: rng.random(),
            rng_direction_bias: rng.random_range(-1.0..=1.0),
            token_tree: Token {
                text: "~BEGIN".into(),
                weight: 1,
                bias: 1,
                children: vec![]
            },
        }
    }
    
    pub async fn start(mut self) -> Result<(), BungusError> {
        let brain = PathBuf::from(std::env::var("BRAIN_PATH").expect("BRAIN_PATH not set"));
        let message_fetcher = GetMessages::new();
        let channels = index_channels(GuildId::from(std::env::var("GUILD_ID").expect("GUILD_ID not set").parse::<u64>()?)).await?;
        let mut messages: HashMap<u64, Vec<Message>> = HashMap::new();

        if !brain.exists() {
            std::fs::write(brain, self.token_tree.json().await?)?;
        } else {
            self.token_tree = Token::from_json(std::fs::read_to_string(&brain)?).await?;
        }

        for channel in channels {
            let http = Http::new(&std::env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN not set"));

            messages.insert(channel.id.get(),channel.messages(http, message_fetcher).await?);
        }
        
        // TODO: Implement Markov parsing logic
        todo!();

        Ok(())
    }
}

