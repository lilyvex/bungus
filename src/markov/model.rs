use rand::Rng;
use crate::markov::token::Token;

pub struct BungusModel {
    max_token_output: usize,
    max_token_relationships: usize,
    total_tokens: usize,
    rng: rand::rngs::ThreadRng,
    rng_weight_bias: i64,
    rng_direction_bias: f32,
    tokens: Vec<Box<dyn Token>>
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
            tokens: Vec::default(),
        }
    }
    
    pub fn start(mut self, tokens: Vec<Box<dyn Token>>) -> Self {
        todo!()
    }
}

