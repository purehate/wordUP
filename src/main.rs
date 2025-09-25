//! WORD UP - Wordlist Operations & Reconnaissance Data - Ultimate Profiling
//! 
//! A high-performance wordlist generator inspired by CeWL, written in Rust.
//! Extracts words from business websites and applies advanced statistical analysis.
//!
//! ## Credits & Inspiration
//! 
//! This tool draws inspiration from several excellent open-source projects:
//! - CeWL (Custom Word List generator) - https://github.com/digininja/CeWL
//! - hashcat-utils - https://github.com/hashcat/hashcat-utils
//! - evilmog/hashcat-scripts - https://github.com/evilmog/hashcat-scripts
//!
//! Special thanks to EvilMog, the Hashcat Team, and Digininja for their contributions
//! to the security community and the techniques that inspired this tool.

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
    💡 Inspired by CeWL, hashcat-utils, and evilmog/hashcat-scripts
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

async fn find_valid_domain(company_name: &str, tlds: &[&str]) -> Option<String> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(3))
        .build()
        .ok()?;
    
    for tld in tlds {
        let domain = format!("{}.{}", company_name, tld);
        
        // Try to resolve DNS first (faster check)
        if let Ok(resolver) = trust_dns_resolver::TokioAsyncResolver::tokio_from_system_conf() {
            if resolver.lookup_ip(&domain).await.is_ok() {
                // Verify with HTTP request
                let test_urls = vec![
                    format!("https://{}", domain),
                    format!("http://{}", domain),
                    format!("https://www.{}", domain),
                    format!("http://www.{}", domain),
                ];
                
                for url in test_urls {
                    if let Ok(response) = client.head(&url).send().await {
                        if response.status().is_success() || response.status().is_redirection() {
                            println!("[+] Found valid domain: {} (verified via {})", domain, url);
                            return Some(domain);
                        }
                    }
                }
                
                // If DNS resolves but HTTP fails, still consider it valid
                println!("[+] Found valid domain: {} (DNS resolution confirmed)", domain);
                return Some(domain);
            }
        }
    }
    
    None
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
        // Try common TLDs to find the actual domain
        println!("[+] Detecting TLD for company: {}", company_name);
        let common_tlds = vec!["com", "org", "net", "edu", "gov", "mil", "int", "co.uk", "co.jp", "de", "fr", "it", "es", "nl", "ca", "au"];
        let domain = find_valid_domain(&company_name, &common_tlds).await
            .unwrap_or_else(|| {
                println!("[!] No valid TLD found, defaulting to .com");
                format!("{}.com", company_name)
            });
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
    
    // Phase 5: Advanced Wordlist Generation
    println!("[+] Phase 5: Advanced Wordlist Generation");
    println!("{}", "-".repeat(40));
    
    let word_processor = WordProcessor::new(&config);
    
    // Apply expander technique
    println!("[+] Applying expander technique...");
    let expanded_words = word_processor.expander_technique(&extraction_results.words);
    println!("    Generated {} expanded words", expanded_words.len());
    
    // Apply cutb technique
    println!("[+] Applying cutb technique...");
    let cut_words = word_processor.cutb_technique(&extraction_results.words);
    println!("    Generated {} cut words", cut_words.len());
    
    // Apply prince technique
    println!("[+] Applying prince technique...");
    let prince_words = word_processor.prince_technique(&extraction_results.words);
    println!("    Generated {} prince words", prince_words.len());
    
    // Apply hybrid attack
    println!("[+] Applying hybrid attack technique...");
    let hybrid_words = word_processor.hybrid_attack(&extraction_results.words);
    println!("    Generated {} hybrid words", hybrid_words.len());
    
    // Apply iterative refinement (3 iterations)
    println!("[+] Applying iterative refinement...");
    let refined_words = word_processor.iterative_refinement(&extraction_results.words, 3);
    println!("    Generated {} refined words", refined_words.len());
    
    // Generate masks for hashcat
    println!("[+] Generating hashcat masks...");
    let masks = word_processor.generate_masks(&extraction_results.words);
    println!("    Generated {} unique masks", masks.len());
    
    // Apply combinator technique
    println!("[+] Applying combinator technique...");
    let combinator_words = word_processor.combinator_technique(&extraction_results.words, &extraction_results.words);
    println!("    Generated {} combinator words", combinator_words.len());
    
    // Generate hashcat rules
    println!("[+] Generating hashcat rules...");
    let rules = word_processor.rli2_technique(&extraction_results.words);
    println!("    Generated {} hashcat rules", rules.len());
    
    // Apply maskgen technique
    println!("[+] Applying maskgen technique...");
    let maskgen_masks = word_processor.maskgen_technique(&extraction_results.words);
    println!("    Generated {} maskgen masks", maskgen_masks.len());
    
    // Apply advanced pipeline
    println!("[+] Applying advanced pipeline...");
    let pipeline_words = word_processor.advanced_pipeline(&extraction_results.words);
    println!("    Generated {} pipeline words", pipeline_words.len());
    
    // Apply PACK techniques
    println!("[+] Applying PACK StatsGen analysis...");
    let pack_stats = word_processor.pack_statsgen(&extraction_results.words);
    println!("    Analyzed {} words with {} unique patterns", 
             pack_stats.get("total_words").unwrap_or(&0), 
             pack_stats.get("unique_patterns").unwrap_or(&0));
    
    println!("[+] Applying PACK PolicyGen analysis...");
    let pack_policy = word_processor.pack_policygen(&extraction_results.words);
    println!("    Policy analysis: min_length={}, max_length={}", 
             pack_policy.get("min_length").unwrap_or(&0),
             pack_policy.get("max_length").unwrap_or(&0));
    
    println!("[+] Applying PACK RuleGen...");
    let pack_rules = word_processor.pack_rulegen(&extraction_results.words);
    println!("    Generated {} PACK rules", pack_rules.len());
    
    println!("[+] Applying PACK MaskGen...");
    let pack_masks = word_processor.pack_maskgen(&extraction_results.words);
    println!("    Generated {} PACK masks", pack_masks.len());
    
    println!("[+] Running PACK comprehensive analysis...");
    let pack_analysis = word_processor.pack_comprehensive_analysis(&extraction_results.words);
    println!("    Comprehensive analysis complete");
    
    // Create comprehensive wordlist with all techniques
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
    
    // Save hashcat masks
    if !masks.is_empty() {
        let masks_filename = format!("{}/{}_masks.txt", project_dir, base_filename);
        save_wordlist(&masks_filename, &masks).await?;
        println!("    Hashcat masks saved: {} ({} masks)", masks_filename, masks.len());
    }
    
    // Save hashcat rules
    if !rules.is_empty() {
        let rules_filename = format!("{}/{}_rules.txt", project_dir, base_filename);
        save_wordlist(&rules_filename, &rules).await?;
        println!("    Hashcat rules saved: {} ({} rules)", rules_filename, rules.len());
    }
    
    // Save maskgen masks
    if !maskgen_masks.is_empty() {
        let maskgen_filename = format!("{}/{}_maskgen.txt", project_dir, base_filename);
        save_wordlist(&maskgen_filename, &maskgen_masks).await?;
        println!("    Maskgen masks saved: {} ({} masks)", maskgen_filename, maskgen_masks.len());
    }
    
    // Save combinator words
    if !combinator_words.is_empty() {
        let combinator_filename = format!("{}/{}_combinator.txt", project_dir, base_filename);
        let combinator_vec: Vec<String> = combinator_words.iter().cloned().collect();
        save_wordlist(&combinator_filename, &combinator_vec).await?;
        println!("    Combinator words saved: {} ({} words)", combinator_filename, combinator_words.len());
    }
    
    // Save pipeline words
    if !pipeline_words.is_empty() {
        let pipeline_filename = format!("{}/{}_pipeline.txt", project_dir, base_filename);
        let pipeline_vec: Vec<String> = pipeline_words.iter().cloned().collect();
        save_wordlist(&pipeline_filename, &pipeline_vec).await?;
        println!("    Pipeline words saved: {} ({} words)", pipeline_filename, pipeline_words.len());
    }
    
    // Save PACK rules
    if !pack_rules.is_empty() {
        let pack_rules_filename = format!("{}/{}_pack_rules.txt", project_dir, base_filename);
        save_wordlist(&pack_rules_filename, &pack_rules).await?;
        println!("    PACK rules saved: {} ({} rules)", pack_rules_filename, pack_rules.len());
    }
    
    // Save PACK masks
    if !pack_masks.is_empty() {
        let pack_masks_filename = format!("{}/{}_pack_masks.txt", project_dir, base_filename);
        save_wordlist(&pack_masks_filename, &pack_masks).await?;
        println!("    PACK masks saved: {} ({} masks)", pack_masks_filename, pack_masks.len());
    }
    
    // Save PACK analysis
    let pack_analysis_filename = format!("{}/{}_pack_analysis.json", project_dir, base_filename);
    let pack_analysis_json = serde_json::to_string_pretty(&pack_analysis)?;
    tokio::fs::write(&pack_analysis_filename, pack_analysis_json).await?;
    println!("    PACK analysis saved: {}", pack_analysis_filename);
    
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
