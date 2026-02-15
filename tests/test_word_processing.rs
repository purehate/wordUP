use std::collections::HashMap;
use word_up::word_processing::WordProcessor;
use word_up::WordUpConfig;

fn config() -> WordUpConfig {
    WordUpConfig {
        target: "acme".into(),
        domain: "acme.com".into(),
        company_name: "Acme".into(),
        workers: 1,
        timeout: 5,
        min_word_length: 3,
        max_word_length: 50,
        extract_emails: false,
        extract_metadata: false,
        group_size: 2,
    }
}

fn words() -> Vec<String> {
    vec!["password".into(), "security".into(), "admin".into()]
}

// --- convert_umlauts ---

#[test]
fn test_convert_umlauts_integration() {
    assert_eq!(WordProcessor::convert_umlauts("München"), "Muenchen");
    assert_eq!(WordProcessor::convert_umlauts("Düsseldorf"), "Duesseldorf");
    assert_eq!(WordProcessor::convert_umlauts("noumlauts"), "noumlauts");
}

// --- expander_technique ---

#[test]
fn test_expander_grows_wordlist() {
    let wp = WordProcessor::new(&config());
    let result = wp.expander_technique(&words());
    // Each word should produce many variations (70+ per word)
    assert!(result.len() > words().len() * 10,
            "Expander should produce many variations, got {}", result.len());
}

#[test]
fn test_expander_empty_input() {
    let wp = WordProcessor::new(&config());
    let result = wp.expander_technique(&[]);
    assert!(result.is_empty());
}

// --- cutb_technique ---

#[test]
fn test_cutb_grows_wordlist() {
    let wp = WordProcessor::new(&config());
    let result = wp.cutb_technique(&words());
    assert!(result.len() > words().len());
}

// --- prince_technique ---

#[test]
fn test_prince_combinations() {
    let wp = WordProcessor::new(&config());
    let result = wp.prince_technique(&words());
    // With 3 words: 3 originals + 3 pairs * 4 separators = 15
    assert!(result.len() >= 3 + 3);
    assert!(result.contains(&"passwordsecurity".to_string()));
}

// --- lenfilter_technique ---

#[test]
fn test_lenfilter_range() {
    let wp = WordProcessor::new(&config());
    let words = vec!["ab".into(), "abc".into(), "abcdef".into(), "abcdefghij".into()];
    let filtered = wp.lenfilter_technique(&words, 3, 7);
    assert!(filtered.contains(&"abc".to_string()));
    assert!(filtered.contains(&"abcdef".to_string()));
    assert!(!filtered.contains(&"ab".to_string()));
    assert!(!filtered.contains(&"abcdefghij".to_string()));
}

// --- cap2bin_technique ---

#[test]
fn test_cap2bin_patterns() {
    let wp = WordProcessor::new(&config());
    let words = vec!["Hello".into(), "world".into(), "ABC123".into()];
    let patterns = wp.cap2bin_technique(&words);
    assert_eq!(patterns[0], "10000"); // Hello
    assert_eq!(patterns[1], "00000"); // world
    assert_eq!(patterns[2], "111222"); // ABC123
}

// --- generate_masks ---

#[test]
fn test_generate_masks_produces_patterns() {
    let wp = WordProcessor::new(&config());
    let masks = wp.generate_masks(&words());
    assert!(!masks.is_empty());
    // All masks should contain only valid hashcat mask chars
    for mask in &masks {
        assert!(mask.contains("?l") || mask.contains("?u") ||
                mask.contains("?d") || mask.contains("?s") ||
                mask.contains("?a"),
                "Invalid mask: {}", mask);
    }
}

// --- combinator_technique ---

#[test]
fn test_combinator_cross_product() {
    let wp = WordProcessor::new(&config());
    let list1 = vec!["user".into()];
    let list2 = vec!["pass".into(), "admin".into()];
    let result = wp.combinator_technique(&list1, &list2);
    assert!(result.contains(&"userpass".to_string()));
    assert!(result.contains(&"useradmin".to_string()));
    assert!(result.contains(&"user_pass".to_string()));
    assert!(result.contains(&"user_admin".to_string()));
}

// --- rli2_technique ---

#[test]
fn test_rli2_produces_rules() {
    let wp = WordProcessor::new(&config());
    let rules = wp.rli2_technique(&words());
    assert!(!rules.is_empty());
    // Should contain capitalization rules for each word
    assert!(rules.contains(&"c password".to_string()));
    assert!(rules.contains(&"u security".to_string()));
}

// --- maskgen_technique ---

#[test]
fn test_maskgen_produces_masks() {
    let wp = WordProcessor::new(&config());
    // Need enough words of same length to trigger (count > 5)
    let many_words: Vec<String> = (0..10).map(|i| format!("word{}", i)).collect();
    let masks = wp.maskgen_technique(&many_words);
    // With 10 words of length 5, should produce masks
    assert!(!masks.is_empty());
}

// --- pack_statsgen ---

#[test]
fn test_pack_statsgen_integration() {
    let wp = WordProcessor::new(&config());
    let stats = wp.pack_statsgen(&words());
    assert_eq!(*stats.get("total_words").unwrap(), 3);
    assert!(*stats.get("unique_lengths").unwrap() > 0);
    assert!(*stats.get("unique_charsets").unwrap() > 0);
}

// --- pack_policygen ---

#[test]
fn test_pack_policygen_integration() {
    let wp = WordProcessor::new(&config());
    let policy = wp.pack_policygen(&words());
    let min_len = *policy.get("min_length").unwrap();
    let max_len = *policy.get("max_length").unwrap();
    assert_eq!(min_len, 5); // "admin" = 5
    assert_eq!(max_len, 8); // "password"/"security" = 8
}

// --- pack_comprehensive_analysis ---

#[test]
fn test_pack_comprehensive_has_all_keys() {
    let wp = WordProcessor::new(&config());
    let analysis = wp.pack_comprehensive_analysis(&words());
    let expected_keys = ["total_words", "unique_lengths", "unique_charsets",
                         "unique_patterns", "min_length", "max_length",
                         "rules_generated", "masks_generated"];
    for key in expected_keys {
        assert!(analysis.contains_key(key), "Missing key: {}", key);
    }
}

// --- create_comprehensive_wordlist ---

#[test]
fn test_comprehensive_wordlist_integration() {
    let wp = WordProcessor::new(&config());
    let freq: HashMap<String, f64> = HashMap::new();
    let metadata = vec!["metadata_word".to_string()];
    let result = wp.create_comprehensive_wordlist(&words(), &metadata, &freq);

    // Should include base words
    for w in &words() {
        assert!(result.contains(w), "Missing base word: {}", w);
    }
    // Should include metadata
    assert!(result.contains(&"metadata_word".to_string()));
    // Should include company variations
    assert!(result.contains(&"acme".to_string()));
}

// --- hybrid_attack ---

#[test]
fn test_hybrid_attack_integration() {
    let wp = WordProcessor::new(&config());
    let result = wp.hybrid_attack(&["test".to_string()]);
    // Should be much larger than input
    assert!(result.len() > 50);
}

// --- advanced_pipeline ---

#[test]
fn test_advanced_pipeline_integration() {
    let wp = WordProcessor::new(&config());
    let input = vec!["test".to_string()];
    let result = wp.advanced_pipeline(&input);
    assert!(result.len() > input.len());
    assert!(result.contains(&"test".to_string()));
}
