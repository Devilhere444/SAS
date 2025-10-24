use anyhow::Result;
use colored::Colorize;
use reqwest::Client;
use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::config::KeywordGenerator as KeywordConfig;

pub struct KeywordGenerator {
    config: KeywordConfig,
    client: Client,
    keywords: Arc<Mutex<HashSet<String>>>,
}

impl KeywordGenerator {
    pub fn new(config: KeywordConfig) -> Result<Self> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeouts as u64))
            .build()?;

        Ok(Self {
            config,
            client,
            keywords: Arc::new(Mutex::new(HashSet::new())),
        })
    }

    pub async fn run(&self, seed_keyword: &str) -> Result<()> {
        println!("{}", "Starting Keyword Generator...".green().bold());
        println!("Seed Keyword: {}", seed_keyword.cyan());
        println!("Threads: {}", self.config.threads);

        let mut tasks = vec![];

        if self.config.google {
            let seed = seed_keyword.to_string();
            let client = self.client.clone();
            let keywords = self.keywords.clone();
            let task = tokio::spawn(async move {
                Self::fetch_keywords_google_static(seed, client, keywords).await
            });
            tasks.push(task);
        }

        if self.config.bing {
            let seed = seed_keyword.to_string();
            let client = self.client.clone();
            let keywords = self.keywords.clone();
            let task = tokio::spawn(async move {
                Self::fetch_keywords_bing_static(seed, client, keywords).await
            });
            tasks.push(task);
        }

        if self.config.yahoo {
            let seed = seed_keyword.to_string();
            let client = self.client.clone();
            let keywords = self.keywords.clone();
            let task = tokio::spawn(async move {
                Self::fetch_keywords_yahoo_static(seed, client, keywords).await
            });
            tasks.push(task);
        }

        if self.config.duckduckgo {
            let seed = seed_keyword.to_string();
            let client = self.client.clone();
            let keywords = self.keywords.clone();
            let task = tokio::spawn(async move {
                Self::fetch_keywords_duckduckgo_static(seed, client, keywords).await
            });
            tasks.push(task);
        }

        if self.config.brave {
            let seed = seed_keyword.to_string();
            let client = self.client.clone();
            let keywords = self.keywords.clone();
            let task = tokio::spawn(async move {
                Self::fetch_keywords_brave_static(seed, client, keywords).await
            });
            tasks.push(task);
        }

        for task in tasks {
            let _ = task.await;
        }

        let keywords = self.keywords.lock().await;
        println!("\n{}", format!("Found {} keywords", keywords.len()).green().bold());
        
        for keyword in keywords.iter() {
            println!("{}", keyword.cyan());
        }

        Ok(())
    }

    async fn fetch_keywords_google_static(
        seed: String,
        client: Client,
        keywords: Arc<Mutex<HashSet<String>>>,
    ) -> Result<()> {
        println!("{}", "Fetching keywords from Google...".yellow());
        
        let search_url = format!(
            "https://www.google.com/complete/search?q={}&client=firefox",
            urlencoding::encode(&seed)
        );

        match client.get(&search_url).send().await {
            Ok(response) => {
                if let Ok(json) = response.json::<serde_json::Value>().await {
                    if let Some(suggestions) = json.get(1).and_then(|v| v.as_array()) {
                        let mut kws = keywords.lock().await;
                        for suggestion in suggestions {
                            if let Some(keyword) = suggestion.as_str() {
                                kws.insert(keyword.to_string());
                            }
                        }
                    }
                }
            }
            Err(e) => eprintln!("Google keyword fetch error: {}", e),
        }

        Ok(())
    }

    async fn fetch_keywords_bing_static(
        seed: String,
        client: Client,
        keywords: Arc<Mutex<HashSet<String>>>,
    ) -> Result<()> {
        println!("{}", "Fetching keywords from Bing...".yellow());
        
        let search_url = format!(
            "https://api.bing.com/osjson.aspx?query={}",
            urlencoding::encode(&seed)
        );

        match client.get(&search_url).send().await {
            Ok(response) => {
                if let Ok(json) = response.json::<serde_json::Value>().await {
                    if let Some(suggestions) = json.get(1).and_then(|v| v.as_array()) {
                        let mut kws = keywords.lock().await;
                        for suggestion in suggestions {
                            if let Some(keyword) = suggestion.as_str() {
                                kws.insert(keyword.to_string());
                            }
                        }
                    }
                }
            }
            Err(e) => eprintln!("Bing keyword fetch error: {}", e),
        }

        Ok(())
    }

    async fn fetch_keywords_yahoo_static(
        seed: String,
        client: Client,
        keywords: Arc<Mutex<HashSet<String>>>,
    ) -> Result<()> {
        println!("{}", "Fetching keywords from Yahoo...".yellow());
        
        let search_url = format!(
            "https://search.yahoo.com/sugg/gossip/gossip-us-ura/?output=sd1&command={}",
            urlencoding::encode(&seed)
        );

        match client.get(&search_url).send().await {
            Ok(response) => {
                if let Ok(json) = response.json::<serde_json::Value>().await {
                    if let Some(results) = json.get("r").and_then(|v| v.as_array()) {
                        let mut kws = keywords.lock().await;
                        for result in results {
                            if let Some(keyword) = result.get("k").and_then(|v| v.as_str()) {
                                kws.insert(keyword.to_string());
                            }
                        }
                    }
                }
            }
            Err(e) => eprintln!("Yahoo keyword fetch error: {}", e),
        }

        Ok(())
    }

    async fn fetch_keywords_duckduckgo_static(
        seed: String,
        client: Client,
        keywords: Arc<Mutex<HashSet<String>>>,
    ) -> Result<()> {
        println!("{}", "Fetching keywords from DuckDuckGo...".yellow());
        
        let search_url = format!(
            "https://ac.duckduckgo.com/ac/?q={}&type=list",
            urlencoding::encode(&seed)
        );

        match client.get(&search_url).send().await {
            Ok(response) => {
                if let Ok(json) = response.json::<serde_json::Value>().await {
                    if let Some(suggestions) = json.get(1).and_then(|v| v.as_array()) {
                        let mut kws = keywords.lock().await;
                        for suggestion in suggestions {
                            if let Some(keyword) = suggestion.as_str() {
                                kws.insert(keyword.to_string());
                            }
                        }
                    }
                }
            }
            Err(e) => eprintln!("DuckDuckGo keyword fetch error: {}", e),
        }

        Ok(())
    }

    async fn fetch_keywords_brave_static(
        seed: String,
        client: Client,
        keywords: Arc<Mutex<HashSet<String>>>,
    ) -> Result<()> {
        println!("{}", "Fetching keywords from Brave...".yellow());
        
        // Brave uses DuckDuckGo's API
        let search_url = format!(
            "https://ac.duckduckgo.com/ac/?q={}&type=list",
            urlencoding::encode(&seed)
        );

        match client.get(&search_url).send().await {
            Ok(response) => {
                if let Ok(json) = response.json::<serde_json::Value>().await {
                    if let Some(suggestions) = json.get(1).and_then(|v| v.as_array()) {
                        let mut kws = keywords.lock().await;
                        for suggestion in suggestions {
                            if let Some(keyword) = suggestion.as_str() {
                                kws.insert(keyword.to_string());
                            }
                        }
                    }
                }
            }
            Err(e) => eprintln!("Brave keyword fetch error: {}", e),
        }

        Ok(())
    }
}
