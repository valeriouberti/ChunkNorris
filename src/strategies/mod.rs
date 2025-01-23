pub mod char_count;
pub mod sentence_count;
#[derive(Debug, Clone, PartialEq)]
pub struct TextBatch {
    pub content: String,
}

pub trait BatchingStrategy {
    fn create_batches(&self, text: &str) -> Vec<TextBatch>;
}
