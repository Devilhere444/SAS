# Build Instructions for Swiss Army Suite (Rust Implementation)

## Quick Start

### Prerequisites

1. **Install Rust** (if not already installed)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   ```

2. **Verify Installation**
   ```bash
   rustc --version
   cargo --version
   ```

### Build the Project

1. **Clone the repository** (if needed)
   ```bash
   git clone https://github.com/Devilhere444/SAS.git
   cd SAS
   ```

2. **Build in Release Mode** (recommended)
   ```bash
   cargo build --release
   ```

3. **Run the application**
   ```bash
   ./target/release/swiss-army-suite
   ```

### Development Build

For faster compilation during development:
```bash
cargo build
./target/debug/swiss-army-suite
```

## Platform-Specific Instructions

### Linux

```bash
# Ubuntu/Debian - Install dependencies
sudo apt-get update
sudo apt-get install build-essential pkg-config libssl-dev

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build
cargo build --release

# Run
./target/release/swiss-army-suite
```

### macOS

```bash
# Install Xcode Command Line Tools
xcode-select --install

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build
cargo build --release

# Run
./target/release/swiss-army-suite
```

### Windows

```powershell
# Install Rust from https://rustup.rs
# Download and run rustup-init.exe

# Install Visual Studio Build Tools (required)
# Download from: https://visualstudio.microsoft.com/downloads/

# Build
cargo build --release

# Run
.\target\release\swiss-army-suite.exe
```

## Build Options

### Optimization Levels

```bash
# Debug build (fast compilation, no optimizations)
cargo build

# Release build (optimized)
cargo build --release

# Maximum optimization
RUSTFLAGS="-C target-cpu=native" cargo build --release
```

### Custom Features

The project uses these optimization settings in `Cargo.toml`:
```toml
[profile.release]
opt-level = 3        # Maximum optimization
lto = true           # Link-time optimization
codegen-units = 1    # Single codegen unit for better optimization
```

## Troubleshooting

### OpenSSL Errors (Linux)

If you get OpenSSL-related errors:
```bash
# Ubuntu/Debian
sudo apt-get install libssl-dev pkg-config

# Fedora/RHEL
sudo dnf install openssl-devel

# Arch Linux
sudo pacman -S openssl pkg-config
```

### Build Errors

```bash
# Clean the build cache
cargo clean

# Update dependencies
cargo update

# Rebuild
cargo build --release
```

### Rust Version Issues

Ensure you have Rust 1.70 or later:
```bash
rustup update
rustc --version
```

## Build Output

After a successful build:

- **Debug Build**: `target/debug/swiss-army-suite`
  - Size: ~15 MB
  - No optimizations
  - Fast compilation

- **Release Build**: `target/release/swiss-army-suite`
  - Size: ~3.9 MB
  - Full optimizations
  - Slower compilation, faster execution

## Configuration

Before running, ensure `config.json` is present in the same directory:

```json
{
  "parser": {
    "threads": 50,
    "pages": 10,
    "timeouts": 60,
    ...
  },
  "vulnscanner": {
    "threads": 100,
    "timeouts": 30
  },
  ...
}
```

The default `config.json` is already included in the repository.

## Running Tests

```bash
# Run all tests (when added)
cargo test

# Run with verbose output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

## Code Quality Tools

### Format Code
```bash
cargo fmt
```

### Lint Code
```bash
cargo clippy
```

### Check Without Building
```bash
cargo check
```

## Performance Profiling

### Build with Debug Symbols
```bash
cargo build --release --profile=release-with-debug-info
```

### Profile with Valgrind (Linux)
```bash
valgrind --tool=callgrind ./target/release/swiss-army-suite
```

## Cross-Compilation

### Build for Different Targets

```bash
# List available targets
rustup target list

# Add a target
rustup target add x86_64-unknown-linux-musl

# Build for specific target
cargo build --release --target x86_64-unknown-linux-musl
```

### Common Targets

- **Linux**: `x86_64-unknown-linux-gnu` (default)
- **Windows**: `x86_64-pc-windows-msvc`
- **macOS**: `x86_64-apple-darwin`
- **Static Linux**: `x86_64-unknown-linux-musl`

## Docker Build

Create a `Dockerfile`:

```dockerfile
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/swiss-army-suite /usr/local/bin/
COPY config.json /app/
WORKDIR /app
CMD ["swiss-army-suite"]
```

Build and run:
```bash
docker build -t swiss-army-suite .
docker run -it swiss-army-suite
```

## Deployment

### Standalone Executable

The release build is statically linked (mostly) and can be distributed:

```bash
# Copy executable and config
cp target/release/swiss-army-suite /destination/
cp config.json /destination/

# Run from destination
cd /destination
./swiss-army-suite
```

### Installation

```bash
# Install to system (Linux/macOS)
sudo cp target/release/swiss-army-suite /usr/local/bin/
sudo cp config.json /etc/swiss-army-suite/

# Or install using cargo
cargo install --path .
```

## Build Time

Approximate build times:

- **Debug Build**: 30-60 seconds (first time), 5-10 seconds (incremental)
- **Release Build**: 60-120 seconds (first time), 10-30 seconds (incremental)

## Size Comparison

| Build Type | Size | Notes |
|------------|------|-------|
| Debug | ~15 MB | Includes debug symbols |
| Release | ~3.9 MB | Optimized |
| Release (stripped) | ~2.5 MB | Debug symbols removed |

To strip debug symbols:
```bash
strip target/release/swiss-army-suite
```

## Dependencies

The project will automatically download ~250 crates from crates.io during the first build. This is normal and managed by Cargo.

Total download size: ~50 MB  
Total disk usage: ~200 MB (including build cache)

## Continuous Integration

### GitHub Actions Example

```yaml
name: Build

on: [push, pull_request]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: cargo build --release
      - run: cargo test
```

## Help

For issues or questions:
1. Check this document
2. Review [RUST_IMPLEMENTATION.md](RUST_IMPLEMENTATION.md)
3. Check Rust documentation: https://doc.rust-lang.org/
4. Cargo documentation: https://doc.rust-lang.org/cargo/

## Summary

**Minimum Requirements**: Rust 1.70+, ~200 MB disk space  
**Build Time**: 1-2 minutes (first time)  
**Output**: 3.9 MB executable  
**Platform**: Linux, macOS, Windows  

---

*Last Updated: October 24, 2025*
