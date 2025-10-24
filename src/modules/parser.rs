use anyhow::Result;
use colored::Colorize;
use reqwest::Client;
use scraper::{Html, Selector};
use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::Mutex;
use url::Url;

use crate::config::Parser as ParserConfig;

pub struct WebParser {
    config: ParserConfig,
    client: Client,
    found_urls: Arc<Mutex<HashSet<String>>>,
}

impl WebParser {
    pub fn new(config: ParserConfig) -> Result<Self> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeouts as u64))
            .build()?;

        Ok(Self {
            config,
            client,
            found_urls: Arc::new(Mutex::new(HashSet::new())),
        })
    }

    pub async fn run(&self, query: &str) -> Result<()> {
        println!("{}", "Starting Web Parser...".green().bold());
        println!("Query: {}", query.cyan());
        println!("Threads: {}", self.config.threads);
        println!("Pages: {}", self.config.pages);

        let mut tasks = vec![];

        if self.config.google {
            let query = query.to_string();
            let config = self.config.clone();
            let client = self.client.clone();
            let found_urls = self.found_urls.clone();
            let task = tokio::spawn(async move {
                Self::search_google_static(query, config, client, found_urls).await
            });
            tasks.push(task);
        }

        if self.config.yahoo {
            let query = query.to_string();
            let config = self.config.clone();
            let client = self.client.clone();
            let found_urls = self.found_urls.clone();
            let task = tokio::spawn(async move {
                Self::search_yahoo_static(query, config, client, found_urls).await
            });
            tasks.push(task);
        }

        if self.config.bing {
            let query = query.to_string();
            let config = self.config.clone();
            let client = self.client.clone();
            let found_urls = self.found_urls.clone();
            let task = tokio::spawn(async move {
                Self::search_bing_static(query, config, client, found_urls).await
            });
            tasks.push(task);
        }

        if self.config.ask {
            let query = query.to_string();
            let config = self.config.clone();
            let client = self.client.clone();
            let found_urls = self.found_urls.clone();
            let task = tokio::spawn(async move {
                Self::search_ask_static(query, config, client, found_urls).await
            });
            tasks.push(task);
        }

        if self.config.yandex {
            let query = query.to_string();
            let config = self.config.clone();
            let client = self.client.clone();
            let found_urls = self.found_urls.clone();
            let task = tokio::spawn(async move {
                Self::search_yandex_static(query, config, client, found_urls).await
            });
            tasks.push(task);
        }

        for task in tasks {
            let _ = task.await;
        }

        let urls = self.found_urls.lock().await;
        println!("\n{}", format!("Found {} unique URLs", urls.len()).green().bold());
        
        for url in urls.iter() {
            println!("{}", url.cyan());
        }

        Ok(())
    }

    async fn search_google_static(
        query: String,
        config: ParserConfig,
        client: Client,
        found_urls: Arc<Mutex<HashSet<String>>>,
    ) -> Result<()> {
        println!("{}", "Searching Google...".yellow());
        
        for page in 0..config.pages {
            let start = page * 10;
            let search_url = format!(
                "https://www.google.com/search?q={}&start={}",
                urlencoding::encode(&query),
                start
            );

            match Self::fetch_and_parse_static(&client, &search_url).await {
                Ok(urls) => {
                    let mut found = found_urls.lock().await;
                    for url in urls {
                        if !config.domain_dedupe || !found.contains(&url) {
                            found.insert(url);
                        }
                    }
                }
                Err(e) => eprintln!("Google search error: {}", e),
            }

            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }

        Ok(())
    }

    async fn search_yahoo_static(
        query: String,
        config: ParserConfig,
        client: Client,
        found_urls: Arc<Mutex<HashSet<String>>>,
    ) -> Result<()> {
        println!("{}", "Searching Yahoo...".yellow());
        
        for page in 0..config.pages {
            let start = page * 10 + 1;
            let search_url = format!(
                "https://search.yahoo.com/search?p={}&b={}",
                urlencoding::encode(&query),
                start
            );

            match Self::fetch_and_parse_static(&client, &search_url).await {
                Ok(urls) => {
                    let mut found = found_urls.lock().await;
                    for url in urls {
                        if !config.domain_dedupe || !found.contains(&url) {
                            found.insert(url);
                        }
                    }
                }
                Err(e) => eprintln!("Yahoo search error: {}", e),
            }

            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }

        Ok(())
    }

    async fn search_bing_static(
        query: String,
        config: ParserConfig,
        client: Client,
        found_urls: Arc<Mutex<HashSet<String>>>,
    ) -> Result<()> {
        println!("{}", "Searching Bing...".yellow());
        
        for page in 0..config.pages {
            let first = page * 10 + 1;
            let search_url = format!(
                "https://www.bing.com/search?q={}&first={}",
                urlencoding::encode(&query),
                first
            );

            match Self::fetch_and_parse_static(&client, &search_url).await {
                Ok(urls) => {
                    let mut found = found_urls.lock().await;
                    for url in urls {
                        if !config.domain_dedupe || !found.contains(&url) {
                            found.insert(url);
                        }
                    }
                }
                Err(e) => eprintln!("Bing search error: {}", e),
            }

            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }

        Ok(())
    }

    async fn search_ask_static(
        query: String,
        config: ParserConfig,
        client: Client,
        found_urls: Arc<Mutex<HashSet<String>>>,
    ) -> Result<()> {
        println!("{}", "Searching Ask...".yellow());
        
        for page in 1..=config.pages {
            let search_url = format!(
                "https://www.ask.com/web?q={}&page={}",
                urlencoding::encode(&query),
                page
            );

            match Self::fetch_and_parse_static(&client, &search_url).await {
                Ok(urls) => {
                    let mut found = found_urls.lock().await;
                    for url in urls {
                        if !config.domain_dedupe || !found.contains(&url) {
                            found.insert(url);
                        }
                    }
                }
                Err(e) => eprintln!("Ask search error: {}", e),
            }

            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }

        Ok(())
    }

    async fn search_yandex_static(
        query: String,
        config: ParserConfig,
        client: Client,
        found_urls: Arc<Mutex<HashSet<String>>>,
    ) -> Result<()> {
        println!("{}", "Searching Yandex...".yellow());
        
        for page in 0..config.pages {
            let search_url = format!(
                "https://yandex.com/search/?text={}&p={}",
                urlencoding::encode(&query),
                page
            );

            match Self::fetch_and_parse_static(&client, &search_url).await {
                Ok(urls) => {
                    let mut found = found_urls.lock().await;
                    for url in urls {
                        if !config.domain_dedupe || !found.contains(&url) {
                            found.insert(url);
                        }
                    }
                }
                Err(e) => eprintln!("Yandex search error: {}", e),
            }

            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }

        Ok(())
    }

    async fn fetch_and_parse_static(client: &Client, url: &str) -> Result<Vec<String>> {
        let response = client.get(url).send().await?;
        let html = response.text().await?;
        let document = Html::parse_document(&html);

        let mut urls = Vec::new();
        
        // Parse links from the HTML
        let link_selector = Selector::parse("a").unwrap();
        for element in document.select(&link_selector) {
            if let Some(href) = element.value().attr("href") {
                if let Ok(parsed_url) = Url::parse(href) {
                    if parsed_url.scheme() == "http" || parsed_url.scheme() == "https" {
                        urls.push(parsed_url.to_string());
                    }
                }
            }
        }

        Ok(urls)
    }
}
