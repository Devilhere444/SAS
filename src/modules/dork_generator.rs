use anyhow::Result;
use colored::Colorize;
use std::collections::HashSet;

use crate::config::DorkGenerator as DorkGenConfig;

pub struct DorkGenerator {
    config: DorkGenConfig,
}

impl DorkGenerator {
    pub fn new(config: DorkGenConfig) -> Self {
        Self { config }
    }

    pub fn run(&self, keywords: Vec<String>, file_types: Vec<String>, domains: Vec<String>) -> Result<Vec<String>> {
        println!("{}", "Starting Dork Generator...".green().bold());
        println!("Keywords: {}", keywords.len());
        println!("File Types: {}", file_types.len());
        println!("Domains: {}", domains.len());
        println!("Limit: {}", self.config.limit);

        let mut dorks = HashSet::new();

        for pattern in &self.config.patterns {
            let generated = self.generate_from_pattern(
                pattern,
                &keywords,
                &file_types,
                &domains,
            );
            
            for dork in generated {
                if dorks.len() >= self.config.limit as usize {
                    break;
                }
                dorks.insert(dork);
            }

            if dorks.len() >= self.config.limit as usize {
                break;
            }
        }

        // Generate common dork patterns
        dorks.extend(self.generate_inurl_dorks(&keywords, &file_types, &domains));
        dorks.extend(self.generate_intext_dorks(&keywords, &file_types, &domains));
        dorks.extend(self.generate_filetype_dorks(&keywords, &file_types, &domains));
        dorks.extend(self.generate_site_dorks(&keywords, &file_types, &domains));

        let dorks: Vec<String> = dorks.into_iter().take(self.config.limit as usize).collect();

        println!("\n{}", format!("Generated {} dorks", dorks.len()).green().bold());
        
        for dork in &dorks {
            println!("{}", dork.cyan());
        }

        Ok(dorks)
    }

    fn generate_from_pattern(
        &self,
        pattern: &str,
        keywords: &[String],
        file_types: &[String],
        domains: &[String],
    ) -> Vec<String> {
        let mut dorks = Vec::new();

        for kw in keywords {
            for ft in file_types {
                for domain in domains {
                    let dork = pattern
                        .replace("(kw)", kw)
                        .replace("(pt)", ft)
                        .replace("(de)", domain)
                        .replace("(sf)", "site:");
                    
                    dorks.push(dork);

                    if dorks.len() >= self.config.limit as usize {
                        return dorks;
                    }
                }
            }
        }

        dorks
    }

    fn generate_inurl_dorks(
        &self,
        keywords: &[String],
        file_types: &[String],
        domains: &[String],
    ) -> Vec<String> {
        let mut dorks = Vec::new();

        for kw in keywords.iter().take(10) {
            for ft in file_types.iter().take(5) {
                let dork = format!("inurl:{} filetype:{}", kw, ft);
                dorks.push(dork);
            }

            for domain in domains.iter().take(5) {
                let dork = format!("inurl:{} site:{}", kw, domain);
                dorks.push(dork);
            }
        }

        dorks
    }

    fn generate_intext_dorks(
        &self,
        keywords: &[String],
        file_types: &[String],
        _domains: &[String],
    ) -> Vec<String> {
        let mut dorks = Vec::new();

        for kw in keywords.iter().take(10) {
            for ft in file_types.iter().take(5) {
                let dork = format!("intext:{} filetype:{}", kw, ft);
                dorks.push(dork);
            }
        }

        dorks
    }

    fn generate_filetype_dorks(
        &self,
        keywords: &[String],
        file_types: &[String],
        domains: &[String],
    ) -> Vec<String> {
        let mut dorks = Vec::new();

        for kw in keywords.iter().take(10) {
            for ft in file_types.iter().take(5) {
                for domain in domains.iter().take(3) {
                    let dork = format!("{} filetype:{} site:{}", kw, ft, domain);
                    dorks.push(dork);
                }
            }
        }

        dorks
    }

    fn generate_site_dorks(
        &self,
        keywords: &[String],
        _file_types: &[String],
        domains: &[String],
    ) -> Vec<String> {
        let mut dorks = Vec::new();

        for kw in keywords.iter().take(10) {
            for domain in domains.iter().take(10) {
                let dork = format!("site:{} \"{}\"", domain, kw);
                dorks.push(dork);
            }
        }

        dorks
    }
}
