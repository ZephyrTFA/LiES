use std::fmt::Debug;

#[derive(Clone)]
pub struct TokenStore {
    pub raw: String,
    pub processed: String,
}

impl Debug for TokenStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TokenStore")
            .field("raw", &self.raw)
            .field("processed", &self.processed)
            .finish()
    }
}
