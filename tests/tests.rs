#[cfg(test)]
mod tests {
    use chunk_norris::strategies::char_count::CharCountBatcher;
    use chunk_norris::strategies::sentence_count::SentenceBatcher;
    use chunk_norris::strategies::BatchingStrategy;

    #[test]
    fn test_empty_input() {
        let batcher = CharCountBatcher::new(10);
        let text = "";
        let batches = batcher.create_batches(text);
        assert_eq!(batches.len(), 0);
    }

    #[test]
    fn test_exact_multiple() {
        let batcher = CharCountBatcher::new(5);
        let text = "abcdefghij"; // 10 chars, exactly 2 batches
        let batches = batcher.create_batches(text);
        assert_eq!(batches.len(), 2);
        assert_eq!(batches[0].content, "abcde");
        assert_eq!(batches[1].content, "fghij");
    }

    #[test]
    fn test_remainder() {
        let batcher = CharCountBatcher::new(5);
        let text = "abcdefghijxyz"; // 13 chars, 2 full batches and a remainder
        let batches = batcher.create_batches(text);
        assert_eq!(batches.len(), 3);
        assert_eq!(batches[0].content, "abcde");
        assert_eq!(batches[1].content, "fghij");
        assert_eq!(batches[2].content, "xyz");
    }

    #[test]
    fn test_single_char_batch() {
        let batcher = CharCountBatcher::new(1);
        let text = "abc";
        let batches = batcher.create_batches(text);
        assert_eq!(batches.len(), 3);
        assert_eq!(batches[0].content, "a");
        assert_eq!(batches[1].content, "b");
        assert_eq!(batches[2].content, "c");
    }

    #[test]
    fn test_input_shorter_than_max_chars() {
        let batcher = CharCountBatcher::new(10);
        let text = "short";
        let batches = batcher.create_batches(text);
        assert_eq!(batches.len(), 1);
        assert_eq!(batches[0].content, "short");
    }

    #[test]
    fn test_sentence_batcher_empty() {
        let batcher = SentenceBatcher::new(10);
        let text = "";
        let batches = batcher.create_batches(text);
        assert_eq!(batches.len(), 0);
    }

    #[test]
    fn test_sentence_batcher_basic() {
        let batcher = SentenceBatcher::new(10);
        let text = "This is a sentence. This is another. And a third one!";
        let batches = batcher.create_batches(text);
        assert_eq!(batches.len(), 3);
        assert_eq!(batches[0].content, "This is a sentence.");
        assert_eq!(batches[1].content, " This is another.");
        assert_eq!(batches[2].content, " And a third one!");
    }

    #[test]
    fn test_sentence_batcher_min_chars() {
        let batcher = SentenceBatcher::new(20);
        let text = "Short sentence. A bit longer one. And a very long sentence here.";
        let batches = batcher.create_batches(text);
        assert_eq!(batches.len(), 3); // First two sentences are combined
        assert_eq!(batches[0].content, "Short sentence. A bi");
        assert_eq!(batches[1].content, "t longer one. And a ");
        assert_eq!(batches[2].content, "very long sentence here.");
    }
    #[test]
    fn test_sentence_batcher_no_delimiters() {
        let batcher = SentenceBatcher::new(10);
        let text = "No sentence delimiters here";
        let batches = batcher.create_batches(text);
        assert_eq!(batches.len(), 1);
        assert_eq!(batches[0].content, "No sentence delimiters here");
    }
}
