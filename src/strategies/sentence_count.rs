use super::{BatchingStrategy, TextBatch};

/// A strategy that splits text into batches based on complete sentences.
///
/// Sentences are identified by the presence of a period (.), question mark (?),
/// or exclamation point (!).
///
/// If the input text does not contain any sentence delimiters, the entire text
/// will be treated as a single batch.
#[derive(Debug, Clone)]
pub struct SentenceBatcher {
    /// The minimum number of characters allowed in a single batch.
    pub min_chars: usize,
}

impl SentenceBatcher {
    /// Creates a new `SentenceBatcher` with the specified minimum character count per batch.
    ///
    /// # Arguments
    ///
    /// * `min_chars` - The minimum number of characters allowed in a single batch.
    ///
    /// # Examples
    ///
    /// ```
    /// use chunk_norris::SentenceBatcher;
    ///
    /// let batcher = SentenceBatcher::new(20);
    /// ```
    pub fn new(min_chars: usize) -> Self {
        Self { min_chars }
    }
}

impl BatchingStrategy for SentenceBatcher {
    /// Splits the input text into batches, where each batch contains complete sentences
    /// and has at least `min_chars` characters (except possibly the last batch).
    ///
    /// # Arguments
    ///
    /// * `text` - The input string to be split into batches.
    ///
    /// # Returns
    ///
    /// A `Vec<TextBatch>` where each `TextBatch` contains one or more complete sentences.
    ///
    /// # Examples
    ///
    /// ```
    /// use chunk_norris::{SentenceBatcher, BatchingStrategy, TextBatch};
    ///
    /// let text = "This is a sentence. This is another. And a third one!";
    /// let batcher = SentenceBatcher::new(10);
    /// let batches = batcher.create_batches(text);
    ///
    /// assert_eq!(batches.len(), 3);
    /// assert_eq!(batches[0].content, "This is a sentence.");
    /// assert_eq!(batches[1].content, " This is another.");
    /// assert_eq!(batches[2].content, " And a third one!");
    /// ```
    fn create_batches(&self, text: &str) -> Vec<TextBatch> {
        let mut batches = Vec::new();
        let mut current_batch = String::new();

        for (index, char) in text.char_indices() {
            current_batch.push(char);
            if char == '.' || char == '?' || char == '!' {
                if current_batch.len() >= self.min_chars
                    || batches.is_empty() && index == text.len() - 1
                {
                    batches.push(TextBatch {
                        content: current_batch.clone(),
                    });
                    current_batch = String::new();
                }
            }
        }
        if !current_batch.is_empty() {
            batches.push(TextBatch {
                content: current_batch,
            });
        } else if batches.is_empty() && !text.is_empty() {
            batches.push(TextBatch {
                content: text.to_string(),
            });
        }
        batches
    }
}
