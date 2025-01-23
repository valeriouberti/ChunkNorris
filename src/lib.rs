// Re-export common types and traits
pub use crate::strategies::char_count::CharCountBatcher;
pub use crate::strategies::{BatchingStrategy, TextBatch};

// Declare modules (publicly if needed)
pub mod strategies;
