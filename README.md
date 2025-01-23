# ChunkNorris

A simple and efficient Rust library for splitting large text into smaller batches based on character count. This is particularly useful when working with large language models (LLMs) that have input size limitations.

## Features

- **`CharCountBatcher`:** Splits text into batches of a specified maximum character length.
- **Easy to Use:** Simple and intuitive API.
- **Lightweight:** No external dependencies for the core `CharCountBatcher`.
- **Extensible:** Designed to accommodate other batching strategies in the future (e.g., sentence-based, semantic unit-based).
- **Well-Tested:** Includes unit tests to ensure correctness.

## Installation

Add `chunk_norris` to your `Cargo.toml`:

```toml
[dependencies]
chunk_norris = "0.1.0" # Replace with the latest version
```

## Usage

### CharCountBatcher

```rust
use chunk_norris::{BatchingStrategy, CharCountBatcher, TextBatch};

fn main() {
    let text = "This is an example text. It will be split into smaller batches.";

    // Create a batcher with a maximum of 25 characters per batch
    let batcher = CharCountBatcher::new(25);

    // Generate the batches
    let batches: Vec<TextBatch> = batcher.create_batches(text);

    // Print the batches
    for (i, batch) in batches.iter().enumerate() {
        println!("Batch {}: {}", i + 1, batch.content);
    }
}
```

#### Output

```
Batch 1: This is an example text.
Batch 2:  It will be split into
Batch 3:  smaller batches.
```

### SentenceBatcher

```rust
use chunk_norris::{BatchingStrategy, SentenceBatcher, TextBatch};

fn main() {
    let text = "This is a sentence. This is another. And a third one!";

    // Create a batcher with a minimum batch size of 10 characters
    let batcher = SentenceBatcher::new(10);

    // Generate the batches
    let batches: Vec<TextBatch> = batcher.create_batches(text);

    // Print the batches
    for (i, batch) in batches.iter().enumerate() {
        println!("Batch {}: {}", i + 1, batch.content);
    }
}
```

#### Output

```
Batch 1: This is an example text.
Batch 2:  It will be split into
Batch 3:  smaller batches.
```

### Explanation

1. **Import**: use text_batcher::{CharCountBatcher, TextBatch}; brings the necessary structs into scope.
2. **Create a Batcher**: CharCountBatcher::new(25) creates a new CharCountBatcher instance with a maximum batch size of 25 characters.
3. **Batch the Text**: batcher.create_batches(text) splits the input text into a Vec<TextBatch>.
4. **Iterate and Process**: The code then iterates through the batches vector and prints the content of each TextBatch.

## Advanced Usage (Future Extensions)

The library is designed to be extensible. Although the current version only provides CharCountBatcher, you can implement the BatchingStrategy trait to create custom batching logic:

```rust
use chunk_norris::{BatchingStrategy, TextBatch};

// Example: A hypothetical SentenceBatcher (not yet implemented in the library)
struct SentenceBatcher {
    max_sentences: usize,
}

impl BatchingStrategy for SentenceBatcher {
    fn create_batches(&self, text: &str) -> Vec<TextBatch> {
        // ... your implementation to split text into batches by sentences ...
    }
}
```

You could then use your custom batcher similarly to the CharCountBatcher.

## Contributing

Contributions are welcome! If you'd like to add new batching strategies, improve the existing code, or fix any issues, please feel free to open an issue or submit a pull request.

## License

This project is licensed under either the MIT License or the Apache License, Version 2.0 at your option.
