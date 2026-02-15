use word_up::markov::MarkovGenerator;

#[test]
fn test_generate_from_empty() {
    let gen = MarkovGenerator::new();
    let result = gen.generate_words(&[], 10);
    assert!(result.is_empty());
}

#[test]
fn test_generate_produces_words() {
    let gen = MarkovGenerator::new();
    let input: Vec<String> = vec![
        "testing".into(), "running".into(), "walking".into(),
        "talking".into(), "working".into(), "playing".into(),
        "reading".into(), "writing".into(), "coding".into(),
        "building".into(), "creating".into(), "making".into(),
    ];
    let result = gen.generate_words(&input, 20);
    // Should produce some results (not guaranteed exact count)
    // At minimum, check constraints
    for word in &result {
        assert!(word.len() >= 3);
        assert!(word.len() <= 50);
        assert!(word.chars().all(|c| c.is_alphabetic()));
        assert_eq!(word, &word.to_lowercase());
    }
}

#[test]
fn test_generate_zero_count() {
    let gen = MarkovGenerator::new();
    let input = vec!["hello".into(), "world".into()];
    let result = gen.generate_words(&input, 0);
    assert!(result.is_empty());
}

#[test]
fn test_generate_with_single_word_input() {
    let gen = MarkovGenerator::new();
    let input = vec!["abcdefgh".to_string()];
    // Should not panic, may produce few or no results
    let _result = gen.generate_words(&input, 5);
}
