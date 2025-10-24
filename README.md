# SwissArmySuite (SAS) - Rust Source Code

A comprehensive security and web research toolkit built in Rust.

## 📁 Repository Structure

```
SAS/
├── src/                     # Rust source code
│   ├── main.rs             # Main entry point with CLI menu
│   ├── config.rs           # Configuration structures
│   └── modules/            # Feature modules
│       ├── parser.rs       # Web parser/crawler
│       ├── vuln_scanner.rs # Vulnerability scanner
│       ├── keyword_generator.rs # Keyword research
│       ├── dork_generator.rs    # Dork generation
│       └── dork_checker.rs      # Dork validation
├── Cargo.toml              # Rust project configuration
├── config.json             # Application configuration
└── README.md               # This file
```

## 🎯 Quick Start

### Prerequisites
- Rust 1.70+ (2021 edition)
- Cargo (included with Rust)

### Build
```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release
```

### Run
```bash
# Run with cargo
cargo run --release

# Or run the compiled executable
./target/release/swiss-army-suite
```


## 🔍 Features

SwissArmySuite is a security research and web scraping toolkit that includes:

1. **Web Parser/Crawler** - Multi-threaded web scraping with search engine support (Google, Yahoo, Bing, Ask, Yandex, DuckDuckGo, Brave)
2. **Vulnerability Scanner** - Network security scanning capabilities
3. **Keyword Research Tool** - Multi-engine keyword extraction
4. **Google Dork Generator** - Automated dork pattern generation
5. **Dork Validator** - Automated dork checking and validation

## ⚙️ Configuration

Edit `config.json` to customize:

```json
{
  "parser": {
    "threads": 50,
    "pages": 10,
    "timeouts": 60,
    "domain_dedupe": true,
    "google": true,
    "yahoo": true,
    ...
  },
  "vulnscanner": {
    "threads": 100,
    "timeouts": 30
  },
  ...
}
```

## 💻 Usage

The application provides an interactive menu:

```
╔═══════════════════════════════════════════════════════════╗
║                                                           ║
║              Swiss Army Suite (SAS)                       ║
║        Security and Web Research Toolkit                  ║
║                                                           ║
║                    Version 1.0.0                          ║
║                                                           ║
╚═══════════════════════════════════════════════════════════╝

═══════════ MAIN MENU ═══════════
1. Web Parser/Crawler
2. Vulnerability Scanner
3. Keyword Generator
4. Dork Generator
5. Dork Checker/Validator
6. Exit
═════════════════════════════════

Enter your choice:
```

### Example Usage

**Web Parser:**
```bash
Enter your choice: 1
Enter search query: site:example.com
# Searches across multiple search engines
```

**Vulnerability Scanner:**
```bash
Enter your choice: 2
Enter target URLs: https://example.com,https://test.com
# Scans for common vulnerabilities
```

**Keyword Generator:**
```bash
Enter your choice: 3
Enter seed keyword: security
# Generates related keywords from multiple search engines
```

**Dork Generator:**
```bash
Enter your choice: 4
Enter keywords: password,login
Enter file types: pdf,doc
Enter domains: .com,.org
# Generates Google dorks based on patterns
```

**Dork Checker:**
```bash
Enter your choice: 5
Enter dorks to check: inurl:admin,site:edu login
# Validates dorks by checking result counts
```

## 🛠️ Development

### Dependencies

The project uses the following Rust crates:
- **tokio** - Async runtime
- **reqwest** - HTTP client
- **serde/serde_json** - JSON handling
- **clap** - CLI argument parsing
- **colored** - Terminal colors
- **scraper** - HTML parsing
- **regex** - Pattern matching
- **anyhow** - Error handling

### Testing

```bash
# Run tests
cargo test

# Check code
cargo check

# Format code
cargo fmt

# Run linter
cargo clippy
```

## 📝 License

This is a security research and web scraping toolkit.

---

**Status**: ✅ Clean Rust Implementation
**Platform**: Cross-platform (Linux, macOS, Windows)

*Last Updated: October 24, 2025*

