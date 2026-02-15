use word_up::stats::Statistics;

#[test]
fn test_analyze_words_integration() {
    let stats = Statistics::new();
    let words: Vec<String> = vec![
        "password".into(), "security".into(), "password".into(),
        "admin".into(), "security".into(), "password".into(),
    ];
    let result = stats.analyze_words(&words);

    assert_eq!(result.top_words["password"], 3);
    assert_eq!(result.top_words["security"], 2);
    assert_eq!(result.top_words["admin"], 1);

    // Verify frequency scores sum approximately to 1.0
    let total_score: f64 = result.frequency_scores.values().sum();
    assert!((total_score - 1.0).abs() < 0.01,
            "Frequency scores should sum to ~1.0, got {}", total_score);
}

#[test]
fn test_analyze_empty_words() {
    let stats = Statistics::new();
    let result = stats.analyze_words(&[]);
    assert!(result.top_words.is_empty());
    assert!(result.frequency_scores.is_empty());
}

#[test]
fn test_analyze_single_word() {
    let stats = Statistics::new();
    let words = vec!["unique".to_string()];
    let result = stats.analyze_words(&words);
    assert_eq!(result.top_words.len(), 1);
    assert_eq!(result.top_words["unique"], 1);
    assert!((result.frequency_scores["unique"] - 1.0).abs() < f64::EPSILON);
}

#[test]
fn test_analyze_all_same() {
    let stats = Statistics::new();
    let words: Vec<String> = std::iter::repeat("same".to_string()).take(100).collect();
    let result = stats.analyze_words(&words);
    assert_eq!(result.top_words.len(), 1);
    assert_eq!(result.top_words["same"], 100);
}
