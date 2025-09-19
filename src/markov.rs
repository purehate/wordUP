//! Markov chain word generation module
//! 
//! Generates new words using Markov chains based on extracted word patterns.

use std::collections::HashMap;
use rand::Rng;

pub struct MarkovGenerator {
    order: usize,
}

impl MarkovGenerator {
    pub fn new() -> Self {
        Self { order: 2 }
    }

    pub fn generate_words(&self, words: &[String], count: usize) -> Vec<String> {
        if words.is_empty() {
            return Vec::new();
        }

        let model = self.build_markov_chain(words);
        self.generate_from_model(&model, count)
    }

    fn build_markov_chain(&self, words: &[String]) -> HashMap<String, HashMap<char, u32>> {
        let mut model: HashMap<String, HashMap<char, u32>> = HashMap::new();

        for word in words {
            if word.len() >= self.order {
                let padded = format!("{}{}{}", 
                    "~".repeat(self.order), 
                    word, 
                    "~".repeat(self.order)
                );
                
                for i in 0..word.len() + self.order {
                    let prefix = &padded[i..i + self.order];
                    let next_char = padded.chars().nth(i + self.order).unwrap_or('~');
                    
                    model.entry(prefix.to_string())
                        .or_insert_with(HashMap::new)
                        .entry(next_char)
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                }
            }
        }

        model
    }

    fn generate_from_model(&self, model: &HashMap<String, HashMap<char, u32>>, count: usize) -> Vec<String> {
        let mut results = Vec::new();
        let mut rng = rand::thread_rng();
        let mut attempts = 0;
        let max_attempts = count * 10;

        while results.len() < count && attempts < max_attempts {
            attempts += 1;
            
            if let Some(word) = self.generate_single_word(model, &mut rng) {
                if word.len() >= 3 && word.len() <= 50 && word.chars().all(|c| c.is_alphabetic()) {
                    results.push(word.to_lowercase());
                }
            }
        }

        results
    }

    fn generate_single_word(
        &self,
        model: &HashMap<String, HashMap<char, u32>>,
        rng: &mut impl Rng,
    ) -> Option<String> {
        let mut prefix = "~".repeat(self.order);
        let mut word = String::new();

        for _ in 0..30 { // max length
            if let Some(choices) = model.get(&prefix) {
                if choices.is_empty() {
                    break;
                }

                let next_char = self.weighted_random_choice(choices, rng)?;
                if next_char == '~' {
                    break;
                }

                word.push(next_char);
                prefix = format!("{}{}", &prefix[1..], next_char);
            } else {
                break;
            }
        }

        if word.is_empty() {
            None
        } else {
            Some(word)
        }
    }

    fn weighted_random_choice(
        &self,
        choices: &HashMap<char, u32>,
        rng: &mut impl Rng,
    ) -> Option<char> {
        let total_weight: u32 = choices.values().sum();
        if total_weight == 0 {
            return None;
        }

        let random_value = rng.gen_range(0..total_weight);
        let mut current_weight = 0;

        for (&char, &weight) in choices {
            current_weight += weight;
            if random_value < current_weight {
                return Some(char);
            }
        }

        None
    }
}
