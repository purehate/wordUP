use word_up::WordUpConfig;

#[test]
fn test_config_creation() {
    let config = WordUpConfig {
        target: "example".into(),
        domain: "example.com".into(),
        company_name: "Example Corp".into(),
        workers: 10,
        timeout: 30,
        min_word_length: 3,
        max_word_length: 50,
        extract_emails: true,
        extract_metadata: true,
        group_size: 2,
    };

    assert_eq!(config.target, "example");
    assert_eq!(config.domain, "example.com");
    assert_eq!(config.company_name, "Example Corp");
    assert_eq!(config.workers, 10);
    assert_eq!(config.timeout, 30);
    assert_eq!(config.min_word_length, 3);
    assert_eq!(config.max_word_length, 50);
    assert!(config.extract_emails);
    assert!(config.extract_metadata);
    assert_eq!(config.group_size, 2);
}

#[test]
fn test_config_clone() {
    let config = WordUpConfig {
        target: "test".into(),
        domain: "test.com".into(),
        company_name: "Test".into(),
        workers: 5,
        timeout: 10,
        min_word_length: 2,
        max_word_length: 100,
        extract_emails: false,
        extract_metadata: false,
        group_size: 3,
    };

    let cloned = config.clone();
    assert_eq!(cloned.target, config.target);
    assert_eq!(cloned.domain, config.domain);
    assert_eq!(cloned.workers, config.workers);
}

#[test]
fn test_config_serialize() {
    let config = WordUpConfig {
        target: "test".into(),
        domain: "test.com".into(),
        company_name: "Test".into(),
        workers: 5,
        timeout: 10,
        min_word_length: 3,
        max_word_length: 50,
        extract_emails: false,
        extract_metadata: false,
        group_size: 2,
    };

    let json = serde_json::to_string(&config).unwrap();
    assert!(json.contains("\"target\":\"test\""));
    assert!(json.contains("\"domain\":\"test.com\""));

    let deserialized: WordUpConfig = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.target, "test");
    assert_eq!(deserialized.domain, "test.com");
}
