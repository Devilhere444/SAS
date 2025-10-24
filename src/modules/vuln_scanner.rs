use anyhow::Result;
use colored::Colorize;
use reqwest::Client;
use std::sync::Arc;
use tokio::sync::Semaphore;

use crate::config::VulnScanner as VulnScannerConfig;

pub struct VulnScanner {
    config: VulnScannerConfig,
    client: Client,
}

impl VulnScanner {
    pub fn new(config: VulnScannerConfig) -> Result<Self> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeouts as u64))
            .danger_accept_invalid_certs(true)
            .build()?;

        Ok(Self { config, client })
    }

    pub async fn run(&self, targets: Vec<String>) -> Result<()> {
        println!("{}", "Starting Vulnerability Scanner...".green().bold());
        println!("Threads: {}", self.config.threads);
        println!("Targets: {}", targets.len());

        let semaphore = Arc::new(Semaphore::new(self.config.threads as usize));
        let mut tasks = vec![];

        for target in targets {
            let sem = semaphore.clone();
            let client = self.client.clone();
            let config = self.config.clone();

            let task = tokio::spawn(async move {
                let _permit = sem.acquire().await.unwrap();
                Self::scan_target(&client, &target, &config).await
            });

            tasks.push(task);
        }

        for task in tasks {
            let _ = task.await;
        }

        println!("{}", "Vulnerability scan complete!".green().bold());
        Ok(())
    }

    async fn scan_target(client: &Client, target: &str, _config: &VulnScannerConfig) -> Result<()> {
        println!("Scanning: {}", target.cyan());

        // Check for common vulnerabilities
        let vuln_paths = vec![
            "/admin",
            "/phpmyadmin",
            "/wp-admin",
            "/.git",
            "/.env",
            "/config.php",
            "/backup",
            "/sql",
            "/database",
        ];

        for path in vuln_paths {
            let url = format!("{}{}", target, path);
            
            match client.get(&url).send().await {
                Ok(response) => {
                    let status = response.status();
                    if status.is_success() {
                        println!(
                            "{} {} - {}",
                            "FOUND:".red().bold(),
                            url.yellow(),
                            status
                        );
                    }
                }
                Err(_) => {}
            }

            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }

        Ok(())
    }
}
