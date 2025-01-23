//! # Chunk Norris - A Rust Text Batching Library
//!
//! `chunk-norris` is a powerful and efficient Rust library designed to split large texts into
//! smaller, manageable batches. This is particularly useful when working with large language
//! models (LLMs) that often have limitations on the size of input they can process at once.
//!
//! ## Features
//!
//! *   **`CharCountBatcher`:** The core batching strategy that splits text based on a fixed
//!     character count.
//! *   **Simple and Intuitive API:** Easy to use and integrate into your projects.
//! *   **Lightweight:** No external dependencies for the core `CharCountBatcher`.
//! *   **Extensible:** Designed to accommodate other batching strategies in the future.
//! *   **Well-Tested:** Comprehensive unit tests to ensure correctness and reliability.
//!
//! ## Getting Started
//!
//! Add `chunk-norris` to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! chunk-norris = "0.1.0" # Replace with the latest version
//! ```
//!
//! Then, use the `CharCountBatcher` in your code:
//!
//! ```
//! use chunk_norris::{BatchingStrategy, CharCountBatcher, TextBatch};
//!
//! fn main() {
//!     let text = "This is a long text that needs to be split into smaller batches.";
//!
//!     // Create a new CharCountBatcher with a maximum of 20 characters per batch
//!     let batcher = CharCountBatcher::new(20);
//!
//!     // Create the batches
//!     let batches: Vec<TextBatch> = batcher.create_batches(text);
//!
//!     // Print the batches
//!     for (i, batch) in batches.iter().enumerate() {
//!         println!("Batch {}: {}", i + 1, batch.content);
//!     }
//! }
//!
//! ```
//! or use the `SentenceBatcher`:
//!
//! ```
//! use chunk_norris::{BatchingStrategy, SentenceBatcher, TextBatch};
//!
//! fn main() {
//!     let text = "This is a long text. That needs to be split into smaller batches.";
//!
//!     // Create a new CharCountBatcher with a maximum of 20 characters per batch
//!     let batcher = SentenceBatcher::new(20);
//!
//!     // Create the batches
//!     let batches: Vec<TextBatch> = batcher.create_batches(text);
//!
//!     // Print the batches
//!     for (i, batch) in batches.iter().enumerate() {
//!         println!("Batch {}: {}", i + 1, batch.content);
//!     }
//! }
//!
//! ```
//! ## Future Development
//!
//! The library is designed to be easily extensible. Future versions might include:
//!
//! *   **`SentenceCountBatcher`:** Batching based on the number of sentences.
//! *   **`SemanticBatcher`:** Batching based on semantic units (paragraphs, sections, etc.).
//! *   **Integration with Tokenizers:**  Support for integration with popular tokenizers for more
//!     advanced batching strategies.
//!
//! ## Contributing
//!
//! Contributions are welcome! Please see the [CONTRIBUTING.md](CONTRIBUTING.md) file for guidelines.
//!
//! ## License
//!
//! This project is licensed under either the [MIT License](LICENSE-MIT) or the [Apache License,
//! Version 2.0](LICENSE-APACHE) at your option.

pub mod strategies;

// Re-export the CharCountBatcher and TextBatch for easier access
pub use crate::strategies::char_count::CharCountBatcher;
pub use crate::strategies::sentence_count::SentenceBatcher;
pub use crate::strategies::BatchingStrategy;
pub use crate::strategies::TextBatch;
