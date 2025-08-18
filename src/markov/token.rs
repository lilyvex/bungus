pub struct TokenWeight {
    text: String,
    weight: u64
}

pub struct TokenWeights {
    weights: Vec<TokenWeight>
}

pub trait Token: Sync {
    fn text(&mut self) -> String;
    fn weights(&mut self) -> TokenWeights;
    fn set_weights(&mut self, weights: TokenWeights);
}