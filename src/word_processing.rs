//! Word processing module
//! 
//! Handles advanced word processing including leetspeak conversion,
//! permutations, and comprehensive wordlist generation.

use std::collections::{HashMap, HashSet};
use chrono::Datelike;

use crate::WordUpConfig;

const LEETSPEAK_MAP: &[(char, &[char])] = &[
    ('a', &['4', '@']),
    ('e', &['3']),
    ('i', &['1', '!']),
    ('o', &['0']),
    ('s', &['5', '$']),
    ('t', &['7']),
    ('l', &['1']),
    ('g', &['9']),
    ('b', &['6']),
    ('z', &['2']),
];

const UMLAUT_MAP: &[(char, &str)] = &[
    ('ä', "ae"), ('ö', "oe"), ('ü', "ue"), ('ß', "ss"),
    ('Ä', "Ae"), ('Ö', "Oe"), ('Ü', "Ue"),
];

pub struct WordProcessor {
    config: WordUpConfig,
}

impl WordProcessor {
    pub fn new(config: &WordUpConfig) -> Self {
        Self {
            config: config.clone(),
        }
    }

    pub fn create_comprehensive_wordlist(
        &self,
        words: &[String],
        metadata: &[String],
        frequency_scores: &HashMap<String, f64>,
    ) -> HashSet<String> {
        let mut wordlist = HashSet::new();
        
        // Add base words
        wordlist.extend(words.iter().cloned());
        
        // Add metadata words
        wordlist.extend(metadata.iter().cloned());
        
        // Add company-specific terms
        let company_terms = self.extract_company_terms(words);
        wordlist.extend(company_terms);
        
        // Generate variations for high-frequency words
        let high_freq_words: Vec<&String> = frequency_scores
            .iter()
            .filter(|(_, score)| **score > 0.01)
            .map(|(word, _)| word)
            .take(100) // Limit to top 100 words
            .collect();
        
        for word in high_freq_words {
            // Leetspeak variations
            let leet_variations = self.apply_leetspeak(word);
            wordlist.extend(leet_variations);
            
            // Permutation variations
            let perm_variations = self.generate_word_permutations(word);
            wordlist.extend(perm_variations);
        }
        
        // Add company name variations
        let company_variations = self.generate_company_variations();
        wordlist.extend(company_variations);
        
        wordlist
    }

    fn extract_company_terms(&self, words: &[String]) -> HashSet<String> {
        let mut company_terms = HashSet::new();
        let company_lower = self.config.company_name.to_lowercase();
        
        // Direct company name matches
        for word in words {
            if word.contains(&company_lower) || company_lower.contains(word) {
                company_terms.insert(word.clone());
            }
        }
        
        // Extract potential product names (capitalized words)
        for word in words {
            if word.chars().next().map_or(false, |c| c.is_uppercase()) && word.len() > 3 {
                company_terms.insert(word.to_lowercase());
            }
        }
        
        company_terms
    }

    fn apply_leetspeak(&self, word: &str) -> HashSet<String> {
        let mut variations = HashSet::new();
        variations.insert(word.to_string()); // Original word
        
        for (original, replacements) in LEETSPEAK_MAP {
            if word.contains(*original) {
                for replacement in *replacements {
                    let leet_word = word.replace(*original, &replacement.to_string());
                    variations.insert(leet_word);
                }
            }
        }
        
        variations
    }

    fn generate_word_permutations(&self, word: &str) -> HashSet<String> {
        let mut variations = HashSet::new();
        variations.insert(word.to_string());
        
        // Add common separators
        let separators = ['-', '_', '.', ' '];
        for sep in separators {
            if word.len() > 4 {
                // Split word and add separator
                for i in 1..word.len() {
                    let variation = format!("{}{}{}", &word[..i], sep, &word[i..]);
                    variations.insert(variation);
                }
            }
        }
        
        // Add numbers
        for i in 0..10 {
            variations.insert(format!("{}{}", word, i));
            variations.insert(format!("{}{}", i, word));
            variations.insert(format!("{}{}{}", word, i, i));
        }
        
        // Add common suffixes
        let common_suffixes = ["s", "ing", "ed", "er", "est", "ly", "tion", "sion", "ness", "ment"];
        for suffix in common_suffixes {
            variations.insert(format!("{}{}", word, suffix));
        }
        
        variations
    }

    fn generate_company_variations(&self) -> HashSet<String> {
        let mut variations = HashSet::new();
        let company_lower = self.config.company_name.to_lowercase().replace(' ', "");
        
        // Add original
        variations.insert(company_lower.clone());
        
        // Add with business suffixes
        let business_suffixes = [
            "inc", "corp", "llc", "ltd", "co", "group", "systems", "solutions", "services",
            "technologies", "software", "hardware", "networks", "security", "consulting",
        ];
        
        for suffix in business_suffixes {
            variations.insert(format!("{}{}", company_lower, suffix));
            variations.insert(format!("{}-{}", company_lower, suffix));
            variations.insert(format!("{}_{}", company_lower, suffix));
        }
        
        // Add with business prefixes
        let business_prefixes = [
            "new", "advanced", "premium", "pro", "ultra", "mega", "super", "max", "plus",
            "elite", "gold", "silver", "platinum", "diamond", "titanium", "steel", "iron",
        ];
        
        for prefix in business_prefixes {
            variations.insert(format!("{}{}", prefix, company_lower));
            variations.insert(format!("{}-{}", prefix, company_lower));
            variations.insert(format!("{}_{}", prefix, company_lower));
        }
        
        // Add year variations
        let current_year = chrono::Utc::now().year();
        for year in (current_year - 5)..(current_year + 2) {
            variations.insert(format!("{}{}", company_lower, year));
            variations.insert(format!("{}-{}", company_lower, year));
            variations.insert(format!("{}_{}", company_lower, year));
        }
        
        variations
    }

    pub fn convert_umlauts(text: &str) -> String {
        let mut result = text.to_string();
        
        for (umlaut, replacement) in UMLAUT_MAP {
            result = result.replace(*umlaut, replacement);
        }
        
        result
    }
}
