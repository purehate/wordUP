//! WORD UP - Wordlist Operations & Reconnaissance Data - Ultimate Profiling
//! 
//! A high-performance wordlist generator inspired by CeWL, written in Rust.
//! Extracts words from business websites and applies advanced statistical analysis.

use anyhow::Result;
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;


use word_up::subdomain::SubdomainDiscovery;
use word_up::word_extraction::WordExtractor;
use word_up::word_processing::WordProcessor;
use word_up::markov::MarkovGenerator;
use word_up::stats::Statistics;
use word_up::WordUpConfig;

#[derive(Parser)]
#[command(name = "word-up")]
#[command(about = "Wordlist Operations & Reconnaissance Data - Ultimate Profiling")]
#[command(version = "0.1.0")]
#[command(before_help = r#"
    ██╗    ██╗ ██████╗ ██████╗ ██████╗     ██╗   ██╗██████╗ 
    ██║    ██║██╔═══██╗██╔══██╗██╔══██╗    ██║   ██║██╔══██╗
    ██║ █╗ ██║██║   ██║██████╔╝██║  ██║    ██║   ██║██████╔╝
    ██║███╗██║██║   ██║██╔══██╗██║  ██║    ██║   ██║██╔═══╝ 
    ╚███╔███╔╝╚██████╔╝██║  ██║██████╔╝    ╚██████╔╝██║     
     ╚══╝╚══╝  ╚═════╝ ╚═╝  ╚═╝╚═════╝      ╚═════╝ ╚═╝     
    
    🚀 Wordlist Operations & Reconnaissance Data - Ultimate Profiling (Rust Edition)
    ⚡ High-Performance • Memory-Safe • Cross-Platform
"#)]
struct Args {
    /// Company name or domain to target
    target: String,
    
    /// Maximum number of concurrent requests
    #[arg(short = 'w', long, default_value = "20")]
    workers: usize,
    
    /// Request timeout in seconds
    #[arg(short = 't', long, default_value = "10")]
    timeout: u64,
    
    /// Minimum word length
    #[arg(short = 'm', long, default_value = "3")]
    min_word_length: usize,
    
    /// Maximum word length
    #[arg(short = 'x', long, default_value = "50")]
    max_word_length: usize,
    
    /// Enable email extraction
    #[arg(short = 'e', long)]
    extract_emails: bool,
    
    /// Enable metadata extraction
    #[arg(short = 'd', long)]
    extract_metadata: bool,
    
    /// Word group size for n-grams
    #[arg(short = 'g', long, default_value = "2")]
    group_size: usize,
    
    /// Verbose output
    #[arg(short = 'v', long)]
    verbose: bool,
}


#[derive(Debug, Serialize, Deserialize)]
struct WordUpResults {
    company_name: String,
    domain: String,
    subdomains_found: usize,
    live_hosts: usize,
    unique_words_extracted: usize,
    emails_found: usize,
    metadata_words: usize,
    word_groups: usize,
    comprehensive_words: usize,
    final_wordlist_size: usize,
    top_words: HashMap<String, u32>,
    emails: Vec<String>,
    timestamp: String,
}

async fn get_unique_project_dir(base_dir: &str) -> Result<String> {
    let mut project_dir = base_dir.to_string();
    let mut counter = 1;
    
    while tokio::fs::metadata(&project_dir).await.is_ok() {
        project_dir = format!("{}_{}", base_dir, counter);
        counter += 1;
    }
    
    Ok(project_dir)
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    
    let args = Args::parse();
    
    println!("{}", r#"
    ██╗    ██╗ ██████╗ ██████╗ ██████╗     ██╗   ██╗██████╗ 
    ██║    ██║██╔═══██╗██╔══██╗██╔══██╗    ██║   ██║██╔══██╗
    ██║ █╗ ██║██║   ██║██████╔╝██║  ██║    ██║   ██║██████╔╝
    ██║███╗██║██║   ██║██╔══██╗██║  ██║    ██║   ██║██╔═══╝ 
    ╚███╔███╔╝╚██████╔╝██║  ██║██████╔╝    ╚██████╔╝██║     
     ╚══╝╚══╝  ╚═════╝ ╚═╝  ╚═╝╚═════╝      ╚═════╝ ╚═╝     
    "#);
    println!("{}", "=".repeat(60));
    println!("🚀 Wordlist Operations & Reconnaissance Data - Ultimate Profiling (Rust Edition)");
    println!("⚡ High-Performance • Memory-Safe • Cross-Platform");
    println!("{}", "=".repeat(60));
    
    // Parse target - could be domain or company name
    let (company_name, domain) = if args.target.contains('.') {
        let domain = args.target.clone();
        let company_name = domain.split('.').next().unwrap_or(&args.target).to_string();
        (company_name, domain)
    } else {
        let company_name = args.target.clone();
        let domain = format!("{}.com", company_name);
        (company_name, domain)
    };
    
    println!("[+] Target: {}", company_name);
    println!("[+] Domain: {}", domain);
    println!();
    
    let config = WordUpConfig {
        target: args.target,
        domain: domain.clone(),
        company_name: company_name.clone(),
        workers: args.workers,
        timeout: args.timeout,
        min_word_length: args.min_word_length,
        max_word_length: args.max_word_length,
        extract_emails: args.extract_emails,
        extract_metadata: args.extract_metadata,
        group_size: args.group_size,
    };
    
    // Phase 1: Subdomain Discovery
    println!("[+] Phase 1: Subdomain Discovery");
    println!("{}", "-".repeat(40));
    
    let subdomain_discovery = SubdomainDiscovery::new(&config);
    let subdomains = subdomain_discovery.discover_subdomains().await?;
    
    println!("[+] Total subdomains discovered: {}", subdomains.len());
    println!();
    
    // Phase 2: Live Host Detection
    println!("[+] Phase 2: Live Host Detection");
    println!("{}", "-".repeat(40));
    
    let live_hosts = subdomain_discovery.check_live_hosts(&subdomains).await?;
    println!("[+] Found {} live hosts", live_hosts.len());
    println!();
    
    // Phase 3: Word Extraction
    println!("[+] Phase 3: Word Extraction");
    println!("{}", "-".repeat(40));
    
    let word_extractor = WordExtractor::new(&config);
    let extraction_results = word_extractor.extract_from_urls(&live_hosts).await?;
    
    println!("[+] Extracted {} unique words", extraction_results.words.len());
    println!("[+] Found {} email addresses", extraction_results.emails.len());
    println!("[+] Extracted {} metadata words", extraction_results.metadata.len());
    println!("[+] Generated {} word groups", extraction_results.word_groups.len());
    println!();
    
    // Phase 4: Statistical Analysis
    println!("[+] Phase 4: Statistical Analysis");
    println!("{}", "-".repeat(40));
    
    let statistics = Statistics::new();
    let word_stats = statistics.analyze_words(&extraction_results.words);
    
    println!("Top 20 most frequent words:");
    for (word, count) in word_stats.top_words.iter().take(20) {
        println!("    {}: {}", word, count);
    }
    println!();
    
    // Phase 5: Wordlist Generation
    println!("[+] Phase 5: Wordlist Generation");
    println!("{}", "-".repeat(40));
    
    let word_processor = WordProcessor::new(&config);
    let comprehensive_wordlist = word_processor.create_comprehensive_wordlist(
        &extraction_results.words,
        &extraction_results.metadata,
        &word_stats.frequency_scores,
    );
    
    // Generate Markov-based words
    let markov_generator = MarkovGenerator::new();
    let markov_words = markov_generator.generate_words(
        &extraction_results.words,
        extraction_results.words.len() * 50, // 50x expansion
    );
    
    // Combine all wordlists
    let mut final_wordlist = comprehensive_wordlist.clone();
    final_wordlist.extend(markov_words);
    
    // Phase 6: Save Results
    println!("[+] Phase 6: Saving Results");
    println!("{}", "-".repeat(40));
    
    // Create project directory with collision handling
    let base_project_dir = format!("wordup_{}", company_name.to_lowercase().replace(" ", "_"));
    let project_dir = get_unique_project_dir(&base_project_dir).await?;
    tokio::fs::create_dir_all(&project_dir).await?;
    println!("[+] Created project directory: {}", project_dir);
    
    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S").to_string();
    let base_filename = format!("{}_{}", company_name, timestamp);
    
    // Save raw wordlist
    let raw_filename = format!("{}/{}_raw.txt", project_dir, base_filename);
    save_wordlist(&raw_filename, &extraction_results.words).await?;
    println!("    Raw wordlist saved: {} ({} words)", raw_filename, extraction_results.words.len());
    
    // Save comprehensive wordlist
    let comp_filename = format!("{}/{}_comprehensive.txt", project_dir, base_filename);
    let comprehensive_words: Vec<String> = comprehensive_wordlist.iter().cloned().collect();
    save_wordlist(&comp_filename, &comprehensive_words).await?;
    println!("    Comprehensive wordlist saved: {} ({} words)", comp_filename, comprehensive_words.len());
    
    // Save final wordlist
    let final_filename = format!("{}/{}_final.txt", project_dir, base_filename);
    let final_words: Vec<String> = final_wordlist.iter().cloned().collect();
    save_wordlist(&final_filename, &final_words).await?;
    println!("    Final wordlist saved: {} ({} words)", final_filename, final_words.len());
    
    // Save email addresses
    if !extraction_results.emails.is_empty() {
        let email_filename = format!("{}/{}_emails.txt", project_dir, base_filename);
        save_wordlist(&email_filename, &extraction_results.emails).await?;
        println!("    Email addresses saved: {} ({} emails)", email_filename, extraction_results.emails.len());
    }
    
    // Save word groups
    if !extraction_results.word_groups.is_empty() {
        let groups_filename = format!("{}/{}_groups.txt", project_dir, base_filename);
        save_wordlist(&groups_filename, &extraction_results.word_groups).await?;
        println!("    Word groups saved: {} ({} groups)", groups_filename, extraction_results.word_groups.len());
    }
    
    // Save metadata words
    if !extraction_results.metadata.is_empty() {
        let metadata_filename = format!("{}/{}_metadata.txt", project_dir, base_filename);
        save_wordlist(&metadata_filename, &extraction_results.metadata).await?;
        println!("    Metadata words saved: {} ({} words)", metadata_filename, extraction_results.metadata.len());
    }
    
    // Save statistics
    let stats_filename = format!("{}/{}_stats.json", project_dir, base_filename);
    let results = WordUpResults {
        company_name: company_name.clone(),
        domain: domain.clone(),
        subdomains_found: subdomains.len(),
        live_hosts: live_hosts.len(),
        unique_words_extracted: extraction_results.words.len(),
        emails_found: extraction_results.emails.len(),
        metadata_words: extraction_results.metadata.len(),
        word_groups: extraction_results.word_groups.len(),
        comprehensive_words: comprehensive_words.len(),
        final_wordlist_size: final_words.len(),
        top_words: word_stats.top_words,
        emails: extraction_results.emails,
        timestamp,
    };
    
    let stats_json = serde_json::to_string_pretty(&results)?;
    tokio::fs::write(&stats_filename, stats_json).await?;
    println!("    Statistics saved: {}", stats_filename);
    
    println!();
    println!("{}", r#"
    ╔══════════════════════════════════════════════════════════╗
    ║  ██████╗ ██████╗ ███╗   ███╗██████╗ ██╗     ███████╗    ║
    ║ ██╔════╝██╔═══██╗████╗ ████║██╔══██╗██║     ██╔════╝    ║
    ║ ██║     ██║   ██║██╔████╔██║██████╔╝██║     █████╗      ║
    ║ ██║     ██║   ██║██║╚██╔╝██║██╔═══╝ ██║     ██╔══╝      ║
    ║ ╚██████╗╚██████╔╝██║ ╚═╝ ██║██║     ███████╗███████╗    ║
    ║  ╚═════╝ ╚═════╝ ╚═╝     ╚═╝╚═╝     ╚══════╝╚══════╝    ║
    ╚══════════════════════════════════════════════════════════╝
    "#);
    println!("{}", "=".repeat(60));
    println!("🎉 WORD UP COMPLETE! 🎉");
    println!("📊 Generated {} words for {}", final_words.len(), company_name);
    println!("📁 Project directory: {}", project_dir);
    println!("📄 Main wordlist: {}", final_filename);
    println!("⚡ High-performance Rust • Memory-safe • Cross-platform");
    println!("{}", "=".repeat(60));
    
    Ok(())
}

async fn save_wordlist(filename: &str, words: &[String]) -> Result<()> {
    let mut file = File::create(filename).await?;
    for word in words {
        file.write_all(format!("{}\n", word).as_bytes()).await?;
    }
    file.flush().await?;
    Ok(())
}
