use super::{BatchingStrategy, TextBatch};

#[derive(Debug, Clone)]
pub struct CharCountBatcher {
    pub max_chars: usize,
}

impl CharCountBatcher {
    pub fn new(max_chars: usize) -> Self {
        Self { max_chars }
    }
}

impl BatchingStrategy for CharCountBatcher {
    fn create_batches(&self, text: &str) -> Vec<TextBatch> {
        let mut batches = Vec::new();
        let mut current_batch = String::new();

        for char in text.chars() {
            current_batch.push(char);
            if current_batch.len() >= self.max_chars {
                batches.push(TextBatch {
                    content: current_batch.clone(),
                });
                current_batch = String::new();
            }
        }

        if !current_batch.is_empty() {
            batches.push(TextBatch {
                content: current_batch,
            });
        }
        batches
    }
}
