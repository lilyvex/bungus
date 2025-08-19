use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use tokio::sync::RwLock;
use log::debug;
use once_cell::sync::Lazy;
use poise::serenity_prelude::{GetMessages, GuildChannel, GuildId, Http, Message};
use rand::Rng;

use crate::error::BungusError;
use crate::markov::token::Token;

pub static MODEL: Lazy<Arc<RwLock<BungusModel>>> = Lazy::new(|| {
    Arc::new(RwLock::new(BungusModel::new(50)))
});

pub struct BungusModel {
    max_token_output: usize,
    total_tokens: usize,
    rng_weight_bias: f32,
    token_tree: Token
}

impl BungusModel {
    pub fn new(max_token_output: usize) -> Self {
        let mut rng = rand::rng();

        BungusModel {
            max_token_output,
            total_tokens: usize::default(),
            rng_weight_bias: rng.random(),
            token_tree: Token {
                text: "~BEGIN".into(),
                weight: 1,
                bias: 0.0,
                children: vec![]
            },
        }
    }

    pub fn insert_words(&mut self, words: &[&str]) {
        Self::insert_words_rec(&mut self.token_tree, words, self.rng_weight_bias);
    }
    fn insert_words_rec(node: &mut Token, words: &[&str], rng_weight_bias: f32) {
        debug!("Inserting words {:?}", words);

        if words.is_empty() {
            return;
        }

        let word = words[0];
        if let Some(child) = node.children.iter_mut().find(|c| c.text == word) {
            child.weight += 1;
            Self::insert_words_rec(child, &words[1..], rng_weight_bias);
        } else {
            node.children.push(Token {
                text: word.to_string(),
                weight: 1,
                bias: rng_weight_bias,
                children: vec![],
            });
            let len = node.children.len();
            let child = &mut node.children[len - 1];
            Self::insert_words_rec(child, &words[1..], rng_weight_bias);
        }
    }

    fn choose_weighted<'a>(&self, children: &'a [Token]) -> Option<&'a Token> {
        let mut rng = rand::rng();

        if children.is_empty() {
            return None;
        }

        let total_weight: u64 = children.iter().map(|c| c.weight).sum();
        let mut roll = rng.random_range(0..total_weight);

        for child in children {
            if roll < child.weight {
                return Some(child);
            }
            roll -= child.weight;
        }

        None
    }

    async fn index_channels(server_id: GuildId) -> Result<Vec<GuildChannel>, BungusError> {
        let http = Http::new(&std::env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN not set"));

        Ok(http.get_channels(server_id).await?)
    }

    pub async fn start(&mut self) -> Result<(), BungusError> {
        self.real_start().await?;
        Ok(())
    }

    async fn real_start(&mut self) -> Result<(), BungusError> {
        debug!("Start: read_main()");

        let brain = PathBuf::from(std::env::var("BRAIN_PATH").expect("BRAIN_PATH not set"));
        let message_fetcher = GetMessages::new().limit(10);
        let channels = Self::index_channels(GuildId::from(std::env::var("GUILD_ID").expect("GUILD_ID not set").parse::<u64>()?)).await?;
        //let mut messages: HashMap<u64, Vec<Message>> = HashMap::new();
        self.token_tree = if !brain.exists() {
            let mut token_root = Token {
                text: "~BEGIN".into(),
                weight: 1,
                bias: 0.0,
                children: vec![]
            };

            tokio::fs::write(&brain, token_root.json().await?).await?;

            token_root
        } else {
            Token::from_json(tokio::fs::read_to_string(&brain).await?).await?
        };

        for channel in channels {
            debug!("Indexing channel: {:#}", channel.name);

            let http = Http::new(&std::env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN not set"));
            let channel_messages = channel.id.messages(http, message_fetcher).await?;
            //messages.insert(channel.id.get(), channel_messages.clone());

            for message in channel_messages {
                debug!("Parsing message: {:#}", message.content);

                let words: Vec<&str> = message.content.split_whitespace().collect();
                if words.is_empty() { continue; }

                self.insert_words(&words);
            }
        }

        tokio::fs::write(brain, self.token_tree.json().await?).await?;

        debug!("Initial tokens: {:?}", self.token_tree);

        Ok(())
    }

    pub async fn add_messages(&mut self) -> Result<(), BungusError> { Ok(()) }
    pub fn generate(&self) -> Option<String> {
        let mut rng = rand::rng();
        let token_count: u32 = rng.random_range(1..=self.max_token_output as u32);
        let mut output: String = String::new();

        let mut cursor: Token = self.token_tree.clone();

        for _ in 0..token_count {
            if cursor.text != "~BEGIN" {
                output.push_str(&cursor.text);
                output.push(' ');
            }

            if cursor.children.is_empty() {
                break;
            }

            if let Some(next) = self.choose_weighted(&cursor.children) {
                cursor = next.clone();
            } else {
                break;
            }
        }

        if output.trim().is_empty() {
            None
        } else {
            Some(output)
        }
    }
}