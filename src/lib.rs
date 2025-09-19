//! WORD UP - Advanced Business Wordlist Generator
//! 
//! A high-performance wordlist generator inspired by CeWL, written in Rust.

pub mod subdomain;
pub mod word_extraction;
pub mod word_processing;
pub mod markov;
pub mod stats;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WordUpConfig {
    pub target: String,
    pub domain: String,
    pub company_name: String,
    pub workers: usize,
    pub timeout: u64,
    pub min_word_length: usize,
    pub max_word_length: usize,
    pub extract_emails: bool,
    pub extract_metadata: bool,
    pub group_size: usize,
}
