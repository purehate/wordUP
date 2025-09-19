//! Statistics module
//! 
//! Handles word frequency analysis and statistical calculations.

use std::collections::HashMap;

#[derive(Debug)]
pub struct WordStatistics {
    pub top_words: HashMap<String, u32>,
    pub frequency_scores: HashMap<String, f64>,
}

pub struct Statistics;

impl Statistics {
    pub fn new() -> Self {
        Self
    }

    pub fn analyze_words(&self, words: &[String]) -> WordStatistics {
        let word_count = self.calculate_word_frequency(words);
        let frequency_scores = self.calculate_frequency_scores(&word_count, words.len());
        
        WordStatistics {
            top_words: word_count.clone(),
            frequency_scores,
        }
    }

    fn calculate_word_frequency(&self, words: &[String]) -> HashMap<String, u32> {
        let mut word_count = HashMap::new();
        
        for word in words {
            *word_count.entry(word.clone()).or_insert(0) += 1;
        }
        
        word_count
    }

    fn calculate_frequency_scores(&self, word_count: &HashMap<String, u32>, total_words: usize) -> HashMap<String, f64> {
        let mut frequency_scores = HashMap::new();
        
        for (word, count) in word_count {
            let score = *count as f64 / total_words as f64;
            frequency_scores.insert(word.clone(), score);
        }
        
        frequency_scores
    }
}
