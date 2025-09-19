//! Word extraction module
//! 
//! Handles extraction of words, emails, and metadata from web pages
//! with advanced text processing inspired by CeWL.

use anyhow::Result;
use regex::Regex;
use reqwest::Client;
use scraper::{Html, Selector};
use std::collections::HashSet;
use std::time::Duration;
use tokio::time::timeout;
use url::Url;

use crate::WordUpConfig;

const EMAIL_REGEX: &str = r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b";
const IGNORE_EXTENSIONS: &[&str] = &[
    ".zip", ".gz", ".bz2", ".png", ".gif", ".jpg", ".jpeg", ".css", ".js", ".ico", ".svg"
];

#[derive(Debug)]
pub struct ExtractionResults {
    pub words: Vec<String>,
    pub emails: Vec<String>,
    pub metadata: Vec<String>,
    pub word_groups: Vec<String>,
}

pub struct WordExtractor {
    config: WordUpConfig,
    client: Client,
    email_regex: Regex,
    word_regex: Regex,
}

impl WordExtractor {
    pub fn new(config: &WordUpConfig) -> Self {
        let client = Client::builder()
            .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36")
            .timeout(Duration::from_secs(config.timeout))
            .build()
            .expect("Failed to create HTTP client");

        let email_regex = Regex::new(EMAIL_REGEX).expect("Invalid email regex");
        let word_regex = Regex::new(&format!(
            r"\b[a-zA-Z]{{{},{}}}\b",
            config.min_word_length, config.max_word_length
        )).expect("Invalid word regex");

        Self {
            config: config.clone(),
            client,
            email_regex,
            word_regex,
        }
    }

    pub async fn extract_from_urls(&self, urls: &[String]) -> Result<ExtractionResults> {
        let mut all_words = HashSet::new();
        let mut all_emails = HashSet::new();
        let mut all_metadata = HashSet::new();
        let mut all_word_groups = HashSet::new();

        let mut handles = Vec::new();

        for url in urls {
            let client = self.client.clone();
            let url = url.clone();
            let config = self.config.clone();
            let email_regex = self.email_regex.clone();
            let word_regex = self.word_regex.clone();

            let handle = tokio::spawn(async move {
                Self::extract_from_url(&client, &url, &config, &email_regex, &word_regex).await
            });

            handles.push(handle);
        }

        for handle in handles {
            if let Ok(Ok((words, emails, metadata, groups))) = handle.await {
                all_words.extend(words);
                all_emails.extend(emails);
                all_metadata.extend(metadata);
                all_word_groups.extend(groups);
            }
        }

        Ok(ExtractionResults {
            words: all_words.into_iter().collect(),
            emails: all_emails.into_iter().collect(),
            metadata: all_metadata.into_iter().collect(),
            word_groups: all_word_groups.into_iter().collect(),
        })
    }

    async fn extract_from_url(
        client: &Client,
        url: &str,
        config: &WordUpConfig,
        email_regex: &Regex,
        word_regex: &Regex,
    ) -> Result<(Vec<String>, Vec<String>, Vec<String>, Vec<String>)> {
        println!("    [.] Scraping {}", url);

        // Check if we should ignore this file
        if Self::should_ignore_file(url) {
            println!("    [!] Ignoring file type: {}", url);
            return Ok((Vec::new(), Vec::new(), Vec::new(), Vec::new()));
        }

        let response = timeout(
            Duration::from_secs(config.timeout),
            client.get(url).send()
        ).await??;

        let content_type = response.headers()
            .get("content-type")
            .and_then(|h| h.to_str().ok())
            .unwrap_or("")
            .to_lowercase();

        let text = response.text().await?;

        // Extract emails
        let emails = Self::extract_emails(&text, email_regex);

        // Process HTML content
        if content_type.contains("text/html") || content_type.is_empty() {
            let html = Html::parse_document(&text);
            let (words, metadata, groups) = Self::extract_from_html(&html, config, word_regex);
            Ok((words, emails, metadata, groups))
        } else {
            // Handle plain text
            let words = Self::extract_words_from_text(&text, word_regex);
            Ok((words, emails, Vec::new(), Vec::new()))
        }
    }

    fn should_ignore_file(url: &str) -> bool {
        if let Ok(parsed_url) = Url::parse(url) {
            let path = parsed_url.path().to_lowercase();
            IGNORE_EXTENSIONS.iter().any(|ext| path.ends_with(ext))
        } else {
            false
        }
    }

    fn extract_emails(text: &str, email_regex: &Regex) -> Vec<String> {
        email_regex
            .find_iter(text)
            .map(|m| m.as_str().to_lowercase())
            .collect()
    }

    fn extract_from_html(
        html: &Html,
        config: &WordUpConfig,
        word_regex: &Regex,
    ) -> (Vec<String>, Vec<String>, Vec<String>) {
        let mut words = HashSet::new();
        let metadata = Vec::new();
        let mut groups = Vec::new();

        // Extract from title
        if let Some(title) = html.select(&Selector::parse("title").unwrap()).next() {
            let title_text = title.text().collect::<String>();
            words.extend(Self::extract_words_from_text(&title_text, word_regex));
        }

        // Extract from meta tags
        for meta in html.select(&Selector::parse("meta").unwrap()) {
            if let Some(content) = meta.value().attr("content") {
                words.extend(Self::extract_words_from_text(content, word_regex));
            }
        }

        // Extract from alt attributes
        for img in html.select(&Selector::parse("img").unwrap()) {
            if let Some(alt) = img.value().attr("alt") {
                words.extend(Self::extract_words_from_text(alt, word_regex));
            }
        }

        // Extract from title attributes
        for element in html.select(&Selector::parse("[title]").unwrap()) {
            if let Some(title) = element.value().attr("title") {
                words.extend(Self::extract_words_from_text(title, word_regex));
            }
        }

        // Extract from placeholder attributes
        for element in html.select(&Selector::parse("[placeholder]").unwrap()) {
            if let Some(placeholder) = element.value().attr("placeholder") {
                words.extend(Self::extract_words_from_text(placeholder, word_regex));
            }
        }

        // Extract from aria-label attributes
        for element in html.select(&Selector::parse("[aria-label]").unwrap()) {
            if let Some(aria_label) = element.value().attr("aria-label") {
                words.extend(Self::extract_words_from_text(aria_label, word_regex));
            }
        }

        // Extract from main content
        let body_text = html.root_element().text().collect::<String>();
        words.extend(Self::extract_words_from_text(&body_text, word_regex));

        // Generate word groups if requested
        if config.group_size > 0 {
            let word_list: Vec<String> = words.iter().cloned().collect();
            groups = Self::generate_word_groups(&word_list, config.group_size);
        }

        // Filter out common words
        let filtered_words: Vec<String> = words
            .into_iter()
            .filter(|word| !Self::is_common_word(word))
            .collect();

        (filtered_words, metadata, groups)
    }

    fn extract_words_from_text(text: &str, word_regex: &Regex) -> Vec<String> {
        word_regex
            .find_iter(text)
            .map(|m| m.as_str().to_lowercase())
            .collect()
    }

    fn generate_word_groups(words: &[String], group_size: usize) -> Vec<String> {
        let mut groups = Vec::new();
        
        for i in 0..=words.len().saturating_sub(group_size) {
            let group = words[i..i + group_size].join(" ");
            groups.push(group);
        }
        
        groups
    }

    fn is_common_word(word: &str) -> bool {
        const COMMON_WORDS: &[&str] = &[
            "the", "and", "for", "are", "but", "not", "you", "all", "can", "had", "her", "was",
            "one", "our", "out", "day", "get", "has", "him", "his", "how", "its", "may", "new",
            "now", "old", "see", "two", "who", "boy", "did", "man", "men", "put", "say", "she",
            "too", "use", "will", "with", "this", "that", "they", "have", "from", "been", "than",
            "what", "some", "time", "very", "when", "come", "here", "just", "like", "long", "make",
            "many", "over", "such", "take", "than", "them", "well", "were", "been", "good", "much",
            "some", "time", "very", "when", "come", "here", "just", "like", "long", "make",
        ];
        
        COMMON_WORDS.contains(&word)
    }
}
