mod config;
mod modules;

use anyhow::Result;
use colored::Colorize;
use std::io::{self, Write};

use config::Configuration;
use modules::{DorkChecker, DorkGenerator, KeywordGenerator, VulnScanner, WebParser};

#[tokio::main]
async fn main() -> Result<()> {
    print_banner();

    let config = match Configuration::load() {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("{}", format!("Error loading config.json: {}", e).red());
            return Ok(());
        }
    };

    loop {
        print_menu();

        let choice = read_input("Enter your choice: ")?;

        match choice.trim() {
            "1" => run_parser(&config).await?,
            "2" => run_vuln_scanner(&config).await?,
            "3" => run_keyword_generator(&config).await?,
            "4" => run_dork_generator(&config).await?,
            "5" => run_dork_checker(&config).await?,
            "6" => {
                println!("{}", "Goodbye!".green().bold());
                break;
            }
            _ => println!("{}", "Invalid choice. Please try again.".red()),
        }

        println!();
    }

    Ok(())
}

fn print_banner() {
    let banner = r#"
╔═══════════════════════════════════════════════════════════╗
║                                                           ║
║              Swiss Army Suite (SAS)                       ║
║        Security and Web Research Toolkit                  ║
║                                                           ║
║                    Version 1.0.0                          ║
║                                                           ║
╚═══════════════════════════════════════════════════════════╝
"#;
    
    println!("{}", banner.cyan().bold());
}

fn print_menu() {
    println!("\n{}", "═══════════ MAIN MENU ═══════════".yellow().bold());
    println!("{}", "1. Web Parser/Crawler".cyan());
    println!("{}", "2. Vulnerability Scanner".cyan());
    println!("{}", "3. Keyword Generator".cyan());
    println!("{}", "4. Dork Generator".cyan());
    println!("{}", "5. Dork Checker/Validator".cyan());
    println!("{}", "6. Exit".red());
    println!("{}", "═════════════════════════════════".yellow().bold());
}

fn read_input(prompt: &str) -> Result<String> {
    print!("{}", prompt.green());
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    Ok(input)
}

async fn run_parser(config: &Configuration) -> Result<()> {
    println!("\n{}", "Web Parser/Crawler".yellow().bold());
    
    let query = read_input("Enter search query: ")?;
    let query = query.trim();

    if query.is_empty() {
        println!("{}", "Query cannot be empty.".red());
        return Ok(());
    }

    let parser = WebParser::new(config.parser.clone())?;
    parser.run(query).await?;

    Ok(())
}

async fn run_vuln_scanner(config: &Configuration) -> Result<()> {
    println!("\n{}", "Vulnerability Scanner".yellow().bold());
    
    let targets_input = read_input("Enter target URLs (comma-separated): ")?;
    let targets: Vec<String> = targets_input
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    if targets.is_empty() {
        println!("{}", "No targets provided.".red());
        return Ok(());
    }

    let scanner = VulnScanner::new(config.vulnscanner.clone())?;
    scanner.run(targets).await?;

    Ok(())
}

async fn run_keyword_generator(config: &Configuration) -> Result<()> {
    println!("\n{}", "Keyword Generator".yellow().bold());
    
    let seed = read_input("Enter seed keyword: ")?;
    let seed = seed.trim();

    if seed.is_empty() {
        println!("{}", "Seed keyword cannot be empty.".red());
        return Ok(());
    }

    let generator = KeywordGenerator::new(config.keywords.clone())?;
    generator.run(seed).await?;

    Ok(())
}

async fn run_dork_generator(config: &Configuration) -> Result<()> {
    println!("\n{}", "Dork Generator".yellow().bold());
    
    let keywords_input = read_input("Enter keywords (comma-separated): ")?;
    let keywords: Vec<String> = keywords_input
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    if keywords.is_empty() {
        println!("{}", "No keywords provided.".red());
        return Ok(());
    }

    let file_types_input = read_input("Enter file types (comma-separated, e.g., pdf,doc,xls): ")?;
    let file_types: Vec<String> = file_types_input
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    let domains_input = read_input("Enter domains (comma-separated, e.g., .com,.org): ")?;
    let domains: Vec<String> = domains_input
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    let generator = DorkGenerator::new(config.dorkgen.clone());
    generator.run(keywords, file_types, domains)?;

    Ok(())
}

async fn run_dork_checker(config: &Configuration) -> Result<()> {
    println!("\n{}", "Dork Checker/Validator".yellow().bold());
    
    let dorks_input = read_input("Enter dorks to check (comma-separated): ")?;
    let dorks: Vec<String> = dorks_input
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    if dorks.is_empty() {
        println!("{}", "No dorks provided.".red());
        return Ok(());
    }

    let checker = DorkChecker::new(config.dorkchecker.clone())?;
    checker.run(dorks).await?;

    Ok(())
}
