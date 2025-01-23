use super::{BatchingStrategy, TextBatch};
//
/// This struct implements the [`BatchingStrategy`] trait to provide a simple way
/// to break down text into smaller chunks of a specified maximum size.
///
/// # Examples
///
/// ```
/// use chunk_norris::{CharCountBatcher, BatchingStrategy};
///
/// let text = "This is a test string.";
/// let batcher = CharCountBatcher::new(5);
/// let batches = batcher.create_batches(text);
///
/// assert_eq!(batches.len(), 4);
/// assert_eq!(batches[0].content, "This ");
/// assert_eq!(batches[1].content, "is a ");
/// assert_eq!(batches[2].content, "test ");
/// assert_eq!(batches[3].content, "strin"); // Last batch might be shorter
/// ```
#[derive(Debug, Clone)]
pub struct CharCountBatcher {
    /// The maximum number of characters allowed in a single batch.
    pub max_chars: usize,
}

impl CharCountBatcher {
    /// Creates a new `CharCountBatcher` with the specified maximum character count per batch.
    ///
    /// # Arguments
    ///
    /// * `max_chars` - The maximum number of characters allowed in a single batch.
    ///
    /// # Examples
    ///
    /// ```
    /// use chunk_norris::CharCountBatcher;
    ///
    /// let batcher = CharCountBatcher::new(20);
    /// ```
    pub fn new(max_chars: usize) -> Self {
        Self { max_chars }
    }
}

impl BatchingStrategy for CharCountBatcher {
    /// Splits the input text into batches, each with a maximum length specified by `max_chars`.
    ///
    /// # Arguments
    ///
    /// * `text` - The input string to be split into batches.
    ///
    /// # Returns
    ///
    /// A `Vec<TextBatch>` where each `TextBatch` contains a portion of the original text.
    ///
    /// # Examples
    ///
    /// ```
    /// use chunk_norris::{CharCountBatcher, BatchingStrategy, TextBatch};
    ///
    /// let text = "This is a longer text to demonstrate batch creation.";
    /// let batcher = CharCountBatcher::new(15);
    /// let batches = batcher.create_batches(text);
    ///
    /// assert_eq!(batches.len(), 4);
    /// assert_eq!(batches[0].content, "This is a longe");
    /// assert_eq!(batches[1].content, "r text to demon");
    /// assert_eq!(batches[2].content, "strate batch cr");
    /// assert_eq!(batches[3].content, "eation.")
    /// ```
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
