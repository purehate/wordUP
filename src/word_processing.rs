//! Word processing module
//! 
//! Handles advanced word processing including leetspeak conversion,
//! permutations, expander techniques, cut-based processing, and
//! comprehensive wordlist generation inspired by hashcat-scripts.
//!
//! ## Credits
//! 
//! This module implements techniques inspired by:
//! - hashcat-utils (https://github.com/hashcat/hashcat-utils)
//! - evilmog/hashcat-scripts (https://github.com/evilmog/hashcat-scripts)
//! - PACK (Password Analysis and Cracking Kit) (https://github.com/iphelix/pack)
//! - princeprocessor functionality
//! - EvilMog's comprehensive attack methodology

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

    /// Expander technique - generates word variations similar to hashcat-utils expander.bin
    /// This creates multiple variations of each word by applying different transformations
    pub fn expander_technique(&self, words: &[String]) -> HashSet<String> {
        let mut expanded = HashSet::new();
        
        for word in words {
            if word.len() > 20 { continue; } // Skip very long words
            
            // Add original word
            expanded.insert(word.clone());
            
            // Add common variations
            let variations = self.generate_expander_variations(word);
            expanded.extend(variations);
        }
        
        expanded
    }

    /// Generate expander-style variations for a single word
    fn generate_expander_variations(&self, word: &str) -> Vec<String> {
        let mut variations = Vec::new();
        let word_lower = word.to_lowercase();
        
        // Common expander patterns
        let patterns = [
            // Capitalization variations
            word_lower.clone(),
            word.to_uppercase(),
            capitalize_first(&word_lower),
            
            // Common suffixes
            format!("{}123", word_lower),
            format!("{}2024", word_lower),
            format!("{}2025", word_lower),
            format!("{}!", word_lower),
            format!("{}@", word_lower),
            format!("{}#", word_lower),
            format!("{}$", word_lower),
            format!("{}%", word_lower),
            format!("{}&", word_lower),
            format!("{}*", word_lower),
            format!("{}+", word_lower),
            format!("{}-", word_lower),
            format!("{}.", word_lower),
            format!("{}/", word_lower),
            format!("{}:", word_lower),
            format!("{};", word_lower),
            format!("{}<", word_lower),
            format!("{}==", word_lower),
            format!("{}>", word_lower),
            format!("{}?", word_lower),
            format!("{}@", word_lower),
            format!("{}[", word_lower),
            format!("{}\\", word_lower),
            format!("{}]", word_lower),
            format!("{}^{}", word_lower, word_lower),
            format!("{}_{}", word_lower, word_lower),
            format!("{}`", word_lower),
            format!("{}{{", word_lower),
            format!("{}{}", word_lower, word_lower),
            format!("{}|", word_lower),
            format!("{}~", word_lower),
            
            // Common prefixes
            format!("admin{}", word_lower),
            format!("user{}", word_lower),
            format!("test{}", word_lower),
            format!("temp{}", word_lower),
            format!("new{}", word_lower),
            format!("old{}", word_lower),
            format!("backup{}", word_lower),
            format!("{}_admin", word_lower),
            format!("{}_user", word_lower),
            format!("{}_test", word_lower),
            format!("{}_temp", word_lower),
            format!("{}_new", word_lower),
            format!("{}_old", word_lower),
            format!("{}_backup", word_lower),
            
            // Number variations
            format!("{}0", word_lower),
            format!("{}1", word_lower),
            format!("{}2", word_lower),
            format!("{}3", word_lower),
            format!("{}4", word_lower),
            format!("{}5", word_lower),
            format!("{}6", word_lower),
            format!("{}7", word_lower),
            format!("{}8", word_lower),
            format!("{}9", word_lower),
            format!("0{}", word_lower),
            format!("1{}", word_lower),
            format!("2{}", word_lower),
            format!("3{}", word_lower),
            format!("4{}", word_lower),
            format!("5{}", word_lower),
            format!("6{}", word_lower),
            format!("7{}", word_lower),
            format!("8{}", word_lower),
            format!("9{}", word_lower),
        ];
        
        for pattern in &patterns {
            if pattern.len() <= self.config.max_word_length {
                variations.push(pattern.clone());
            }
        }
        
        variations
    }

    /// Cut-based processing technique - cuts words at different positions
    /// Similar to hashcat-utils cutb functionality
    pub fn cutb_technique(&self, words: &[String]) -> HashSet<String> {
        let mut cut_words = HashSet::new();
        
        for word in words {
            if word.len() < 3 { continue; }
            
            // Add original word
            cut_words.insert(word.clone());
            
            // Cut from beginning (remove first 1-3 characters)
            for i in 1..=std::cmp::min(3, word.len() - 1) {
                if let Some(cut) = word.get(i..) {
                    if cut.len() >= self.config.min_word_length {
                        cut_words.insert(cut.to_string());
                    }
                }
            }
            
            // Cut from end (remove last 1-3 characters)
            for i in 1..=std::cmp::min(3, word.len() - 1) {
                if let Some(cut) = word.get(..word.len() - i) {
                    if cut.len() >= self.config.min_word_length {
                        cut_words.insert(cut.to_string());
                    }
                }
            }
            
            // Cut from both ends
            for i in 1..=std::cmp::min(2, word.len() / 2) {
                for j in 1..=std::cmp::min(2, word.len() / 2) {
                    if i + j < word.len() {
                        if let Some(cut) = word.get(i..word.len() - j) {
                            if cut.len() >= self.config.min_word_length {
                                cut_words.insert(cut.to_string());
                            }
                        }
                    }
                }
            }
        }
        
        cut_words
    }

    /// Prince processor technique - generates word combinations and mutations
    /// Inspired by princeprocessor functionality
    pub fn prince_technique(&self, words: &[String]) -> HashSet<String> {
        let mut prince_words = HashSet::new();
        
        // Add original words
        prince_words.extend(words.iter().cloned());
        
        // Generate combinations of words
        for (i, word1) in words.iter().enumerate() {
            for word2 in words.iter().skip(i + 1) {
                // Concatenate words
                let combined = format!("{}{}", word1, word2);
                if combined.len() <= self.config.max_word_length {
                    prince_words.insert(combined);
                }
                
                // Concatenate with separator
                let combined_sep = format!("{}_{}", word1, word2);
                if combined_sep.len() <= self.config.max_word_length {
                    prince_words.insert(combined_sep);
                }
                
                let combined_dash = format!("{}-{}", word1, word2);
                if combined_dash.len() <= self.config.max_word_length {
                    prince_words.insert(combined_dash);
                }
                
                let combined_dot = format!("{}.{}", word1, word2);
                if combined_dot.len() <= self.config.max_word_length {
                    prince_words.insert(combined_dot);
                }
            }
        }
        
        prince_words
    }

    /// Hybrid attack technique - combines multiple word processing methods
    /// Similar to hashcat hybrid attacks
    pub fn hybrid_attack(&self, words: &[String]) -> HashSet<String> {
        let mut hybrid_words = HashSet::new();
        
        // Start with expander technique
        let expanded = self.expander_technique(words);
        hybrid_words.extend(expanded);
        
        // Apply cutb to expanded words
        let expanded_vec: Vec<String> = hybrid_words.iter().cloned().collect();
        let cut_words = self.cutb_technique(&expanded_vec);
        hybrid_words.extend(cut_words);
        
        // Apply prince technique
        let prince_words = self.prince_technique(&expanded_vec);
        hybrid_words.extend(prince_words);
        
        // Apply leetspeak to all words
        let mut leet_words = HashSet::new();
        for word in &hybrid_words {
            let leet_variations = self.apply_leetspeak(word);
            leet_words.extend(leet_variations);
        }
        hybrid_words.extend(leet_words);
        
        hybrid_words
    }

    /// Iterative refinement - repeatedly processes wordlist to find new variations
    /// Based on EvilMog's iterative methodology
    pub fn iterative_refinement(&self, words: &[String], iterations: usize) -> HashSet<String> {
        let mut current_words: HashSet<String> = words.iter().cloned().collect();
        
        for i in 0..iterations {
            println!("    [+] Iteration {}/{}: {} words", i + 1, iterations, current_words.len());
            
            // Convert to vector for processing
            let word_vec: Vec<String> = current_words.iter().cloned().collect();
            
            // Apply hybrid attack
            let new_words = self.hybrid_attack(&word_vec);
            
            // Merge with existing words
            let previous_size = current_words.len();
            current_words.extend(new_words);
            
            let new_count = current_words.len() - previous_size;
            if new_count == 0 {
                println!("    [+] No new words found in iteration {}, stopping", i + 1);
                break;
            }
            
            println!("    [+] Found {} new words (total: {})", new_count, current_words.len());
        }
        
        current_words
    }

    /// Generate mask patterns for hashcat attacks
    /// Based on PACK mask generation techniques
    pub fn generate_masks(&self, words: &[String]) -> Vec<String> {
        let mut masks = Vec::new();
        
        for word in words {
            if word.len() < 3 || word.len() > 16 { continue; }
            
            // Generate character class masks
            let mut mask = String::new();
            for ch in word.chars() {
                match ch {
                    'a'..='z' => mask.push_str("?l"),
                    'A'..='Z' => mask.push_str("?u"),
                    '0'..='9' => mask.push_str("?d"),
                    '!'..='/' => mask.push_str("?s"),
                    ':'..='@' => mask.push_str("?s"),
                    '['..='`' => mask.push_str("?s"),
                    '{'..='~' => mask.push_str("?s"),
                    _ => mask.push_str("?a"),
                }
            }
            
            if !masks.contains(&mask) {
                masks.push(mask);
            }
        }
        
        masks
    }

    /// Combinator technique - combines words from two lists
    /// Based on hashcat-utils combinator.bin functionality
    pub fn combinator_technique(&self, words1: &[String], words2: &[String]) -> HashSet<String> {
        let mut combined = HashSet::new();
        
        for word1 in words1 {
            for word2 in words2 {
                // Direct concatenation
                let combined_word = format!("{}{}", word1, word2);
                if combined_word.len() <= self.config.max_word_length {
                    combined.insert(combined_word);
                }
                
                // With separator
                let combined_sep = format!("{}_{}", word1, word2);
                if combined_sep.len() <= self.config.max_word_length {
                    combined.insert(combined_sep);
                }
            }
        }
        
        combined
    }

    /// RLI2 technique - generates rules for hashcat based on word patterns
    /// Based on hashcat-utils rli2.bin functionality
    pub fn rli2_technique(&self, words: &[String]) -> Vec<String> {
        let mut rules = Vec::new();
        
        for word in words {
            if word.len() < 3 { continue; }
            
            // Generate common rule patterns
            let word_lower = word.to_lowercase();
            
            // Capitalization rules
            rules.push(format!("c {}", word_lower));
            rules.push(format!("u {}", word_lower));
            rules.push(format!("l {}", word_lower));
            
            // Prefix rules
            rules.push(format!("^0 {}", word_lower));
            rules.push(format!("^1 {}", word_lower));
            rules.push(format!("^2 {}", word_lower));
            rules.push(format!("^3 {}", word_lower));
            rules.push(format!("^4 {}", word_lower));
            rules.push(format!("^5 {}", word_lower));
            rules.push(format!("^6 {}", word_lower));
            rules.push(format!("^7 {}", word_lower));
            rules.push(format!("^8 {}", word_lower));
            rules.push(format!("^9 {}", word_lower));
            
            // Suffix rules
            rules.push(format!("$0 {}", word_lower));
            rules.push(format!("$1 {}", word_lower));
            rules.push(format!("$2 {}", word_lower));
            rules.push(format!("$3 {}", word_lower));
            rules.push(format!("$4 {}", word_lower));
            rules.push(format!("$5 {}", word_lower));
            rules.push(format!("$6 {}", word_lower));
            rules.push(format!("$7 {}", word_lower));
            rules.push(format!("$8 {}", word_lower));
            rules.push(format!("$9 {}", word_lower));
            
            // Special character rules
            rules.push(format!("$! {}", word_lower));
            rules.push(format!("$@ {}", word_lower));
            rules.push(format!("$# {}", word_lower));
            rules.push(format!("$$ {}", word_lower));
            rules.push(format!("$% {}", word_lower));
            rules.push(format!("$& {}", word_lower));
            rules.push(format!("$* {}", word_lower));
            rules.push(format!("$+ {}", word_lower));
            rules.push(format!("$- {}", word_lower));
            rules.push(format!("$. {}", word_lower));
            rules.push(format!("$/ {}", word_lower));
            rules.push(format!("$: {}", word_lower));
            rules.push(format!("$; {}", word_lower));
            rules.push(format!("$< {}", word_lower));
            rules.push(format!("$= {}", word_lower));
            rules.push(format!("$> {}", word_lower));
            rules.push(format!("$? {}", word_lower));
            rules.push(format!("$[ {}", word_lower));
            rules.push(format!("$\\ {}", word_lower));
            rules.push(format!("$] {}", word_lower));
            rules.push(format!("$^ {}", word_lower));
            rules.push(format!("$_ {}", word_lower));
            rules.push(format!("$` {}", word_lower));
            rules.push(format!("${{ {}", word_lower));
            rules.push(format!("$| {}", word_lower));
            rules.push(format!("$~ {}", word_lower));
        }
        
        rules
    }

    /// Maskgen technique - generates masks based on word patterns
    /// Based on hashcat-utils maskgen.bin functionality
    pub fn maskgen_technique(&self, words: &[String]) -> Vec<String> {
        let mut masks = Vec::new();
        
        // Generate length-based masks
        let mut length_counts: HashMap<usize, usize> = HashMap::new();
        for word in words {
            *length_counts.entry(word.len()).or_insert(0) += 1;
        }
        
        // Create masks for common lengths
        for (length, count) in length_counts {
            if count > 5 && length >= 3 && length <= 16 {
                // Generate different mask patterns for this length
                masks.push(format!("?l?l?l{}", "?l".repeat(length - 3)));
                masks.push(format!("?u?u?u{}", "?u".repeat(length - 3)));
                masks.push(format!("?d?d?d{}", "?d".repeat(length - 3)));
                masks.push(format!("?s?s?s{}", "?s".repeat(length - 3)));
                masks.push(format!("?a?a?a{}", "?a".repeat(length - 3)));
                
                // Mixed patterns
                if length >= 6 {
                    masks.push(format!("?l?l?l?d?d{}", "?l".repeat(length - 5)));
                    masks.push(format!("?u?u?u?d?d{}", "?u".repeat(length - 5)));
                    masks.push(format!("?l?l?l?l?d{}", "?l".repeat(length - 5)));
                }
            }
        }
        
        masks
    }

    /// Lenfilter technique - filters words by length ranges
    /// Based on hashcat-utils lenfilter.bin functionality
    pub fn lenfilter_technique(&self, words: &[String], min_len: usize, max_len: usize) -> Vec<String> {
        words.iter()
            .filter(|word| word.len() >= min_len && word.len() <= max_len)
            .cloned()
            .collect()
    }

    /// Cap2bin technique - converts words to binary patterns
    /// Based on hashcat-utils cap2bin.bin functionality
    pub fn cap2bin_technique(&self, words: &[String]) -> Vec<String> {
        let mut patterns = Vec::new();
        
        for word in words {
            let mut pattern = String::new();
            for ch in word.chars() {
                match ch {
                    'a'..='z' => pattern.push('0'),
                    'A'..='Z' => pattern.push('1'),
                    _ => pattern.push('2'),
                }
            }
            patterns.push(pattern);
        }
        
        patterns
    }

    /// Advanced wordlist processing pipeline
    /// Combines multiple hashcat-utils techniques
    pub fn advanced_pipeline(&self, words: &[String]) -> HashSet<String> {
        let mut result = HashSet::new();
        
        // Start with original words
        result.extend(words.iter().cloned());
        
        // Apply expander technique
        let expanded = self.expander_technique(words);
        result.extend(expanded);
        
        // Apply cutb technique
        let cut_words = self.cutb_technique(words);
        result.extend(cut_words);
        
        // Apply prince technique
        let prince_words = self.prince_technique(words);
        result.extend(prince_words);
        
        // Apply combinator technique (combine with itself)
        let word_vec: Vec<String> = result.iter().cloned().collect();
        let combinator_words = self.combinator_technique(&word_vec, &word_vec);
        result.extend(combinator_words);
        
        // Apply hybrid attack
        let hybrid_words = self.hybrid_attack(words);
        result.extend(hybrid_words);
        
        result
    }

    /// PACK StatsGen technique - analyzes password statistics and patterns
    /// Based on PACK statsgen.py functionality
    pub fn pack_statsgen(&self, words: &[String]) -> HashMap<String, usize> {
        let mut stats = HashMap::new();
        
        // Length distribution
        let mut length_dist: HashMap<usize, usize> = HashMap::new();
        for word in words {
            *length_dist.entry(word.len()).or_insert(0) += 1;
        }
        
        // Character set analysis
        let mut charset_dist: HashMap<String, usize> = HashMap::new();
        for word in words {
            let charset = self.analyze_charset(word);
            *charset_dist.entry(charset).or_insert(0) += 1;
        }
        
        // Pattern analysis
        let mut pattern_dist: HashMap<String, usize> = HashMap::new();
        for word in words {
            let pattern = self.analyze_pattern(word);
            *pattern_dist.entry(pattern).or_insert(0) += 1;
        }
        
        // Combine statistics
        stats.insert("total_words".to_string(), words.len());
        stats.insert("unique_lengths".to_string(), length_dist.len());
        stats.insert("unique_charsets".to_string(), charset_dist.len());
        stats.insert("unique_patterns".to_string(), pattern_dist.len());
        
        // Add most common lengths
        let mut length_vec: Vec<_> = length_dist.iter().collect();
        length_vec.sort_by(|a, b| b.1.cmp(a.1));
        for (i, (length, count)) in length_vec.iter().take(10).enumerate() {
            stats.insert(format!("length_{}_count", i + 1), **count);
            stats.insert(format!("length_{}_value", i + 1), **length);
        }
        
        stats
    }

    /// Analyze character set of a word (PACK-inspired)
    fn analyze_charset(&self, word: &str) -> String {
        let mut has_lower = false;
        let mut has_upper = false;
        let mut has_digit = false;
        let mut has_special = false;
        
        for ch in word.chars() {
            match ch {
                'a'..='z' => has_lower = true,
                'A'..='Z' => has_upper = true,
                '0'..='9' => has_digit = true,
                _ => has_special = true,
            }
        }
        
        let mut charset = String::new();
        if has_lower { charset.push('l'); }
        if has_upper { charset.push('u'); }
        if has_digit { charset.push('d'); }
        if has_special { charset.push('s'); }
        
        charset
    }

    /// Analyze pattern of a word (PACK-inspired)
    fn analyze_pattern(&self, word: &str) -> String {
        let mut pattern = String::new();
        
        for ch in word.chars() {
            match ch {
                'a'..='z' => pattern.push('l'),
                'A'..='Z' => pattern.push('u'),
                '0'..='9' => pattern.push('d'),
                _ => pattern.push('s'),
            }
        }
        
        pattern
    }

    /// PACK PolicyGen technique - analyzes password policies
    /// Based on PACK policygen.py functionality
    pub fn pack_policygen(&self, words: &[String]) -> HashMap<String, usize> {
        let mut policy_stats = HashMap::new();
        
        // Minimum length analysis
        let min_length = words.iter().map(|w| w.len()).min().unwrap_or(0);
        let max_length = words.iter().map(|w| w.len()).max().unwrap_or(0);
        policy_stats.insert("min_length".to_string(), min_length);
        policy_stats.insert("max_length".to_string(), max_length);
        
        // Character requirements
        let mut requires_lower = 0;
        let mut requires_upper = 0;
        let mut requires_digit = 0;
        let mut requires_special = 0;
        
        for word in words {
            let charset = self.analyze_charset(word);
            if charset.contains('l') { requires_lower += 1; }
            if charset.contains('u') { requires_upper += 1; }
            if charset.contains('d') { requires_digit += 1; }
            if charset.contains('s') { requires_special += 1; }
        }
        
        policy_stats.insert("has_lowercase".to_string(), requires_lower);
        policy_stats.insert("has_uppercase".to_string(), requires_upper);
        policy_stats.insert("has_digits".to_string(), requires_digit);
        policy_stats.insert("has_special".to_string(), requires_special);
        
        // Common patterns
        let mut common_patterns: HashMap<String, usize> = HashMap::new();
        for word in words {
            let pattern = self.analyze_pattern(word);
            *common_patterns.entry(pattern).or_insert(0) += 1;
        }
        
        // Add top patterns
        let mut pattern_vec: Vec<_> = common_patterns.iter().collect();
        pattern_vec.sort_by(|a, b| b.1.cmp(a.1));
        for (i, (pattern, count)) in pattern_vec.iter().take(5).enumerate() {
            policy_stats.insert(format!("pattern_{}_count", i + 1), **count);
            policy_stats.insert(format!("pattern_{}_value", i + 1), pattern.len());
        }
        
        policy_stats
    }

    /// PACK RuleGen technique - generates rules with edit distance
    /// Based on PACK rulegen.py functionality
    pub fn pack_rulegen(&self, words: &[String]) -> Vec<String> {
        let mut rules = Vec::new();
        
        for word in words {
            if word.len() < 3 { continue; }
            
            // Generate rules based on word characteristics
            let word_lower = word.to_lowercase();
            
            // Basic transformation rules
            rules.push(format!("c {}", word_lower));
            rules.push(format!("u {}", word_lower));
            rules.push(format!("l {}", word_lower));
            
            // Prefix rules with common patterns
            let prefixes = ["!", "@", "#", "$", "%", "&", "*", "(", ")", "-", "_", "=", "+"];
            for prefix in &prefixes {
                rules.push(format!("^{} {}", prefix, word_lower));
            }
            
            // Suffix rules with common patterns
            for suffix in &prefixes {
                rules.push(format!("${} {}", suffix, word_lower));
            }
            
            // Number suffix rules
            for i in 0..=99 {
                rules.push(format!("${} {}", i, word_lower));
            }
            
            // Year suffix rules
            for year in 1900..=2030 {
                rules.push(format!("${} {}", year, word_lower));
            }
            
            // Leetspeak rules
            let leet_word = self.apply_leetspeak_simple(&word_lower);
            if leet_word != word_lower {
                rules.push(format!("so0 se3 si1 sg9 {}", word_lower));
            }
        }
        
        rules
    }

    /// Simple leetspeak conversion for rule generation
    fn apply_leetspeak_simple(&self, word: &str) -> String {
        word.replace('o', "0")
            .replace('e', "3")
            .replace('i', "1")
            .replace('a', "@")
            .replace('s', "5")
            .replace('t', "7")
            .replace('l', "1")
            .replace('g', "9")
            .replace('b', "6")
            .replace('z', "2")
    }

    /// PACK MaskGen technique - advanced mask generation
    /// Based on PACK maskgen.py functionality
    pub fn pack_maskgen(&self, words: &[String]) -> Vec<String> {
        let mut masks = Vec::new();
        
        // Analyze word patterns
        let mut pattern_counts: HashMap<String, usize> = HashMap::new();
        for word in words {
            let pattern = self.analyze_pattern(word);
            *pattern_counts.entry(pattern).or_insert(0) += 1;
        }
        
        // Generate masks for common patterns
        let mut pattern_vec: Vec<_> = pattern_counts.iter().collect();
        pattern_vec.sort_by(|a, b| b.1.cmp(a.1));
        
        for (pattern, count) in pattern_vec {
            if *count > 5 && pattern.len() >= 3 && pattern.len() <= 16 {
                let mut mask = String::new();
                for ch in pattern.chars() {
                    match ch {
                        'l' => mask.push_str("?l"),
                        'u' => mask.push_str("?u"),
                        'd' => mask.push_str("?d"),
                        's' => mask.push_str("?s"),
                        _ => mask.push_str("?a"),
                    }
                }
                masks.push(mask);
            }
        }
        
        // Generate length-based masks
        let mut length_counts: HashMap<usize, usize> = HashMap::new();
        for word in words {
            *length_counts.entry(word.len()).or_insert(0) += 1;
        }
        
        for (length, count) in length_counts {
            if count > 10 && length >= 3 && length <= 16 {
                // Generate various mask patterns for this length
                masks.push(format!("?l?l?l{}", "?l".repeat(length - 3)));
                masks.push(format!("?u?u?u{}", "?u".repeat(length - 3)));
                masks.push(format!("?d?d?d{}", "?d".repeat(length - 3)));
                masks.push(format!("?s?s?s{}", "?s".repeat(length - 3)));
                masks.push(format!("?a?a?a{}", "?a".repeat(length - 3)));
                
                // Mixed patterns
                if length >= 6 {
                    masks.push(format!("?l?l?l?d?d{}", "?l".repeat(length - 5)));
                    masks.push(format!("?u?u?u?d?d{}", "?u".repeat(length - 5)));
                    masks.push(format!("?l?l?l?l?d{}", "?l".repeat(length - 5)));
                    masks.push(format!("?l?l?l?l?l?d{}", "?l".repeat(length - 6)));
                }
            }
        }
        
        masks
    }

    /// PACK comprehensive analysis - combines all PACK techniques
    /// Based on PACK's comprehensive analysis approach
    pub fn pack_comprehensive_analysis(&self, words: &[String]) -> HashMap<String, String> {
        let mut analysis = HashMap::new();
        
        // Run all PACK analyses
        let stats = self.pack_statsgen(words);
        let policy = self.pack_policygen(words);
        let rules = self.pack_rulegen(words);
        let masks = self.pack_maskgen(words);
        
        // Combine results
        analysis.insert("total_words".to_string(), stats.get("total_words").unwrap_or(&0).to_string());
        analysis.insert("unique_lengths".to_string(), stats.get("unique_lengths").unwrap_or(&0).to_string());
        analysis.insert("unique_charsets".to_string(), stats.get("unique_charsets").unwrap_or(&0).to_string());
        analysis.insert("unique_patterns".to_string(), stats.get("unique_patterns").unwrap_or(&0).to_string());
        analysis.insert("min_length".to_string(), policy.get("min_length").unwrap_or(&0).to_string());
        analysis.insert("max_length".to_string(), policy.get("max_length").unwrap_or(&0).to_string());
        analysis.insert("rules_generated".to_string(), rules.len().to_string());
        analysis.insert("masks_generated".to_string(), masks.len().to_string());
        
        analysis
    }
}

/// Helper function to capitalize first letter
fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}
