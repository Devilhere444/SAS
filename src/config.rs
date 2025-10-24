use serde::{Deserialize, Serialize};
use std::fs;
use anyhow::Result;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Configuration {
    pub parser: Parser,
    pub vulnscanner: VulnScanner,
    pub keywords: KeywordGenerator,
    pub dorkgen: DorkGenerator,
    pub dorkchecker: DorkChecker,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Parser {
    pub threads: u32,
    pub pages: u32,
    pub timeouts: u32,
    pub domain_dedupe: bool,
    pub google: bool,
    pub googleproto: bool,
    pub googleknowldge: bool,
    pub googleapi: bool,
    pub googlemobile: bool,
    pub ask: bool,
    pub bing: bool,
    pub yahoo: bool,
    pub startpage: bool,
    pub myway: bool,
    pub tonline: bool,
    pub yandex: bool,
    pub use_antipub: bool,
    pub auto_antipub: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VulnScanner {
    pub threads: u32,
    pub timeouts: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct KeywordGenerator {
    pub threads: u32,
    pub timeouts: u32,
    pub google: bool,
    pub bing: bool,
    pub ask: bool,
    pub duckduckgo: bool,
    pub yahoo: bool,
    pub brave: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DorkGenerator {
    pub limit: u32,
    pub patterns: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DorkChecker {
    pub threads: u32,
    pub timeouts: u32,
    pub threshold: u32,
}

impl Configuration {
    pub fn load() -> Result<Self> {
        let content = fs::read_to_string("config.json")?;
        let config: Configuration = serde_json::from_str(&content)?;
        Ok(config)
    }
}
