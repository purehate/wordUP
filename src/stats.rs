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

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_words() -> Vec<String> {
        vec![
            "hello".into(), "world".into(), "hello".into(),
            "rust".into(), "hello".into(), "world".into(),
        ]
    }

    #[test]
    fn test_new() {
        let _stats = Statistics::new();
    }

    #[test]
    fn test_analyze_empty() {
        let stats = Statistics::new();
        let result = stats.analyze_words(&[]);
        assert!(result.top_words.is_empty());
        assert!(result.frequency_scores.is_empty());
    }

    #[test]
    fn test_word_frequency() {
        let stats = Statistics::new();
        let freq = stats.calculate_word_frequency(&sample_words());
        assert_eq!(freq["hello"], 3);
        assert_eq!(freq["world"], 2);
        assert_eq!(freq["rust"], 1);
    }

    #[test]
    fn test_frequency_scores() {
        let stats = Statistics::new();
        let result = stats.analyze_words(&sample_words());
        let hello_score = result.frequency_scores["hello"];
        // 3 out of 6 = 0.5
        assert!((hello_score - 0.5).abs() < f64::EPSILON);
        let rust_score = result.frequency_scores["rust"];
        // 1 out of 6
        assert!((rust_score - 1.0 / 6.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_top_words_matches_frequency() {
        let stats = Statistics::new();
        let result = stats.analyze_words(&sample_words());
        assert_eq!(result.top_words.len(), 3);
        assert_eq!(result.top_words["hello"], 3);
    }

    #[test]
    fn test_single_word() {
        let stats = Statistics::new();
        let words = vec!["only".to_string()];
        let result = stats.analyze_words(&words);
        assert_eq!(result.top_words["only"], 1);
        assert!((result.frequency_scores["only"] - 1.0).abs() < f64::EPSILON);
    }
}
