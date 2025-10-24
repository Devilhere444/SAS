use anyhow::Result;
use colored::Colorize;
use reqwest::Client;
use scraper::{Html, Selector};
use std::sync::Arc;
use tokio::sync::Semaphore;

use crate::config::DorkChecker as DorkCheckerConfig;

pub struct DorkChecker {
    config: DorkCheckerConfig,
    client: Client,
}

impl DorkChecker {
    pub fn new(config: DorkCheckerConfig) -> Result<Self> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeouts as u64))
            .build()?;

        Ok(Self { config, client })
    }

    pub async fn run(&self, dorks: Vec<String>) -> Result<Vec<(String, u32)>> {
        println!("{}", "Starting Dork Checker...".green().bold());
        println!("Threads: {}", self.config.threads);
        println!("Dorks to check: {}", dorks.len());
        println!("Result threshold: {}", self.config.threshold);

        let semaphore = Arc::new(Semaphore::new(self.config.threads as usize));
        let mut tasks = vec![];

        for dork in dorks {
            let sem = semaphore.clone();
            let client = self.client.clone();
            let config = self.config.clone();

            let task = tokio::spawn(async move {
                let _permit = sem.acquire().await.unwrap();
                Self::check_dork(&client, &dork, &config).await
            });

            tasks.push(task);
        }

        let mut valid_dorks = Vec::new();

        for task in tasks {
            if let Ok(Ok(Some(result))) = task.await {
                valid_dorks.push(result);
            }
        }

        valid_dorks.sort_by(|a, b| b.1.cmp(&a.1));

        println!("\n{}", format!("Found {} valid dorks", valid_dorks.len()).green().bold());
        
        for (dork, count) in &valid_dorks {
            println!("{}: {} results", dork.cyan(), count.to_string().yellow());
        }

        Ok(valid_dorks)
    }

    async fn check_dork(
        client: &Client,
        dork: &str,
        config: &DorkCheckerConfig,
    ) -> Result<Option<(String, u32)>> {
        let search_url = format!(
            "https://www.google.com/search?q={}",
            urlencoding::encode(dork)
        );

        match client.get(&search_url).send().await {
            Ok(response) => {
                let html = response.text().await?;
                let result_count = Self::extract_result_count(&html);

                if result_count >= config.threshold {
                    println!(
                        "{} {} - {} results",
                        "VALID:".green().bold(),
                        dork.cyan(),
                        result_count.to_string().yellow()
                    );
                    return Ok(Some((dork.to_string(), result_count)));
                } else {
                    println!(
                        "{} {} - {} results (below threshold)",
                        "SKIP:".red(),
                        dork,
                        result_count
                    );
                }
            }
            Err(e) => {
                eprintln!("Error checking dork '{}': {}", dork, e);
            }
        }

        Ok(None)
    }

    fn extract_result_count(html: &str) -> u32 {
        let document = Html::parse_document(html);
        
        // Try to find result count from Google's result stats
        let result_stats_selector = Selector::parse("#result-stats").unwrap();
        
        if let Some(element) = document.select(&result_stats_selector).next() {
            let text = element.text().collect::<String>();
            
            // Extract number from text like "About 1,234 results"
            let numbers: String = text
                .chars()
                .filter(|c| c.is_numeric())
                .collect();
            
            if let Ok(count) = numbers.parse::<u32>() {
                return count;
            }
        }

        // Fallback: count search result divs
        let result_selector = Selector::parse(".g").unwrap();
        document.select(&result_selector).count() as u32
    }
}
