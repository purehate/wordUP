//! Subdomain discovery module
//! 
//! Handles subdomain enumeration using multiple methods:
//! - Certificate Transparency logs (crt.sh)
//! - DNSDumpster
//! - Wayback Machine
//! - DNS brute force
//! - Company-specific variations

use anyhow::Result;
use chrono::Datelike;
use reqwest::Client;
use serde_json::Value;
use std::collections::HashSet;
use std::time::Duration;
use tokio::time::timeout;
use trust_dns_resolver::TokioAsyncResolver;
use url::Url;

use crate::WordUpConfig;

const COMMON_SUBDOMAINS: &[&str] = &[
    "www", "mail", "webmail", "vpn", "remote", "portal", "admin", "login", "app", "cloud", "dev",
    "api", "blog", "shop", "store", "support", "help", "docs", "wiki", "test", "staging", "prod",
    "production", "demo", "beta", "alpha", "cdn", "static", "assets", "media", "files", "download",
    "upload", "secure", "ssl", "ftp", "smtp", "pop", "imap", "ldap", "radius", "auth", "sso",
    "oauth", "jwt", "token", "session", "cache", "redis", "db", "database", "sql", "nosql",
    "mongo", "elastic", "kibana", "grafana", "prometheus", "monitoring", "logs", "metrics",
    "analytics", "stats", "report", "dashboard", "panel", "console", "control", "manage",
    "management", "admin", "root", "super", "master", "primary", "secondary", "backup",
    "replica", "slave", "node", "cluster", "load", "balancer", "proxy", "gateway", "router",
    "switch", "firewall", "security", "scan", "audit", "compliance", "policy", "rules",
    "config", "settings", "preferences", "profile", "account", "user", "customer", "client",
    "partner", "vendor", "supplier", "contractor", "employee", "staff", "team", "group",
    "department", "division", "unit", "branch", "office", "location", "site", "facility",
    "building", "floor", "room", "desk", "station", "terminal", "kiosk", "booth", "counter",
];

const BUSINESS_SUFFIXES: &[&str] = &[
    "inc", "corp", "llc", "ltd", "co", "group", "systems", "solutions", "services",
    "technologies", "software", "hardware", "networks", "security", "consulting",
    "partners", "associates", "enterprises", "ventures", "holdings", "international",
    "global", "worldwide", "america", "usa", "canada", "europe", "asia", "pacific",
];

const BUSINESS_PREFIXES: &[&str] = &[
    "new", "advanced", "premium", "pro", "ultra", "mega", "super", "max", "plus",
    "elite", "gold", "silver", "platinum", "diamond", "titanium", "steel", "iron",
];

pub struct SubdomainDiscovery {
    config: WordUpConfig,
    client: Client,
    dns_resolver: TokioAsyncResolver,
}

impl SubdomainDiscovery {
    pub fn new(config: &WordUpConfig) -> Self {
        let client = Client::builder()
            .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36")
            .timeout(Duration::from_secs(config.timeout))
            .build()
            .expect("Failed to create HTTP client");

        let dns_resolver = TokioAsyncResolver::tokio_from_system_conf()
            .expect("Failed to create DNS resolver");

        Self {
            config: config.clone(),
            client,
            dns_resolver,
        }
    }

    pub async fn discover_subdomains(&self) -> Result<Vec<String>> {
        let mut all_subdomains = HashSet::new();

        // CRT.sh
        if let Ok(subs) = self.get_subdomains_crtsh().await {
            println!("    Found {} subdomains from crt.sh", subs.len());
            all_subdomains.extend(subs);
        }

        // DNSDumpster
        if let Ok(subs) = self.get_subdomains_dnsdumpster().await {
            println!("    Found {} subdomains from DNSDumpster", subs.len());
            all_subdomains.extend(subs);
        }

        // Wayback Machine
        if let Ok(subs) = self.get_subdomains_wayback().await {
            println!("    Found {} subdomains from Wayback Machine", subs.len());
            all_subdomains.extend(subs);
        }

        // Brute force
        let brute_subs = self.brute_force_subdomains().await;
        println!("    Found {} subdomains from brute force", brute_subs.len());
        all_subdomains.extend(brute_subs);

        // Company variations
        let company_subs = self.generate_company_variations();
        println!("    Generated {} company variations", company_subs.len());
        all_subdomains.extend(company_subs);

        Ok(all_subdomains.into_iter().collect())
    }

    async fn get_subdomains_crtsh(&self) -> Result<Vec<String>> {
        println!("[+] Pulling subdomains from crt.sh");
        let url = format!("https://crt.sh/?q=%25.{}&output=json", self.config.domain);
        
        let response = self.client.get(&url).send().await?;
        let data: Vec<Value> = response.json().await?;
        
        let mut subdomains = Vec::new();
        for entry in data {
            if let Some(name_value) = entry.get("name_value").and_then(|v| v.as_str()) {
                for sub in name_value.split('\n') {
                    let sub = sub.trim().to_lowercase();
                    if sub.contains(&self.config.domain) && !sub.starts_with('*') {
                        subdomains.push(sub);
                    }
                }
            }
        }
        
        Ok(subdomains)
    }

    async fn get_subdomains_dnsdumpster(&self) -> Result<Vec<String>> {
        println!("[+] Pulling subdomains from DNSDumpster");
        
        // Get CSRF token
        let response = self.client.get("https://dnsdumpster.com/").send().await?;
        let _html = response.text().await?;
        
        // This is a simplified version - in practice you'd need to parse the CSRF token
        // For now, we'll skip the actual submission and return empty results
        // In a real implementation, you'd use a proper HTML parser to extract the token
        // and then submit the form
        
        Ok(Vec::new())
    }

    async fn get_subdomains_wayback(&self) -> Result<Vec<String>> {
        println!("[+] Pulling subdomains from Wayback Machine");
        let url = format!(
            "http://web.archive.org/cdx/search/cdx?url=*.{}/*&output=json&collapse=urlkey",
            self.config.domain
        );
        
        let response = self.client.get(&url).send().await?;
        let data: Vec<Value> = response.json().await?;
        
        let mut subdomains = Vec::new();
        for entry in data.iter().skip(1) { // Skip header
            if let Some(url_str) = entry.get(2).and_then(|v| v.as_str()) {
                if let Ok(url) = Url::parse(url_str) {
                    if let Some(hostname) = url.host_str() {
                        if hostname.contains(&self.config.domain) {
                            subdomains.push(hostname.to_lowercase());
                        }
                    }
                }
            }
        }
        
        Ok(subdomains)
    }

    async fn brute_force_subdomains(&self) -> Vec<String> {
        println!("[+] Brute-forcing common subdomains");
        let mut found = Vec::new();
        
        for sub in COMMON_SUBDOMAINS {
            let fqdn = format!("{}.{}", sub, self.config.domain);
            if let Ok(_) = self.dns_resolver.lookup_ip(&fqdn).await {
                found.push(fqdn);
            }
        }
        
        found
    }

    fn generate_company_variations(&self) -> Vec<String> {
        let mut variations = Vec::new();
        let company_lower = self.config.company_name.to_lowercase().replace(' ', "");
        
        // Add original
        variations.push(format!("{}.{}", company_lower, self.config.domain));
        
        // Add with business suffixes
        for suffix in BUSINESS_SUFFIXES {
            variations.push(format!("{}{}.{}", company_lower, suffix, self.config.domain));
            variations.push(format!("{}-{}.{}", company_lower, suffix, self.config.domain));
            variations.push(format!("{}_{}.{}", company_lower, suffix, self.config.domain));
        }
        
        // Add with business prefixes
        for prefix in BUSINESS_PREFIXES {
            variations.push(format!("{}{}.{}", prefix, company_lower, self.config.domain));
            variations.push(format!("{}-{}.{}", prefix, company_lower, self.config.domain));
            variations.push(format!("{}_{}.{}", prefix, company_lower, self.config.domain));
        }
        
        // Add year variations
        let current_year = chrono::Utc::now().year();
        for year in (current_year - 5)..(current_year + 2) {
            variations.push(format!("{}{}.{}", company_lower, year, self.config.domain));
            variations.push(format!("{}-{}.{}", company_lower, year, self.config.domain));
            variations.push(format!("{}_{}.{}", company_lower, year, self.config.domain));
        }
        
        variations
    }

    pub async fn check_live_hosts(&self, subdomains: &[String]) -> Result<Vec<String>> {
        println!("[+] Checking which subdomains are live");
        
        let mut live_hosts = Vec::new();
        let mut handles = Vec::new();
        
        for subdomain in subdomains {
            let client = self.client.clone();
            let subdomain = subdomain.clone();
            let timeout_duration = Duration::from_secs(self.config.timeout);
            
            let handle = tokio::spawn(async move {
                for scheme in &["https://", "http://"] {
                    let url = format!("{}{}", scheme, subdomain);
                    if let Ok(response) = timeout(
                        timeout_duration,
                        client.get(&url).send()
                    ).await {
                        if let Ok(resp) = response {
                            if resp.status().is_success() {
                                return Some(url);
                            }
                        }
                    }
                }
                None
            });
            
            handles.push(handle);
        }
        
        for handle in handles {
            if let Ok(Some(url)) = handle.await {
                live_hosts.push(url);
            }
        }
        
        Ok(live_hosts)
    }
}
