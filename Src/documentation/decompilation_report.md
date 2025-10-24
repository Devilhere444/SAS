# SwissArmySuite.exe Decompilation Report

## Executive Summary
This document provides a comprehensive decompilation and reverse engineering analysis of `SwissArmySuite.exe`, a Windows PE32+ executable compiled from Rust source code.

## Binary Information
- **File**: SwissArmySuite.exe
- **Type**: PE32+ executable (console) x86-64, for MS Windows
- **Size**: 8.1 MB (8,531,968 bytes)
- **Architecture**: x86-64 (AMD64)
- **Platform**: Windows CUI (Console User Interface)
- **Compiler**: Rust (rustc version: 98aa3624be70462d6a25ed5544333e3df62f4c66)
- **Compile Date**: Tue Mar 19 18:07:45 2024
- **Security Features**:
  - NX (No-Execute) enabled
  - PIE (Position Independent Executable) enabled
  - No canary protection
  - Not stripped (debug symbols present)

## Compiler Details
The executable was compiled using Rust compiler with the following build hash:
```
rustc/98aa3624be70462d6a25ed5544333e3df62f4c66
```

Key Rust dependencies found:
- rustc-demangle-0.1.23
- Various Windows API bindings

## Structure Analysis

### Sections
The binary contains 7 sections typical of Windows PE executables:
1. `.text` - Executable code
2. `.rdata` - Read-only data
3. `.data` - Initialized data
4. `.pdata` - Exception handling data
5. `_RDATA` - Additional read-only data
6. `.rsrc` - Resources
7. `.reloc` - Relocation data

### Entry Points
- Entry Point: `0x1405850bc` (398 bytes)
- Main Function: `0x140126620` (422 bytes)

## Decompiled Components

### Available Artifacts
1. **disassembly.asm** - Complete disassembly (1,555,083 lines)
   - Full x86-64 assembly code for all functions
   - Includes instruction addresses and opcodes
   
2. **strings.txt** - Extracted strings (102,113 lines)
   - All text strings found in the binary
   - URLs, error messages, configuration data
   
3. **headers_and_sections.txt** - Binary structure (257,191 lines)
   - PE header information
   - Section details
   - Symbol tables and relocations
   
4. **functions_list.txt** - Function definitions
   - Entry points and main function addresses
   
5. **imports.txt** - Imported functions (228 entries)
   - Windows API functions used by the program
   
6. **exports.txt** - Exported functions (4 entries)
   - Functions exposed by the executable
   
7. **binary_info.txt** - Complete binary metadata
   - Compiler information
   - Security features
   - Architecture details

## Functionality Analysis

### Core Features
Based on the configuration file (`config.json`) and string analysis, this appears to be a Swiss Army Suite tool with the following capabilities:

#### 1. Web Parser/Crawler
- Multi-threaded parsing (50 threads)
- Support for multiple search engines:
  - Google (multiple variants)
  - Yahoo
  - Bing
  - Ask
  - Yandex
  - Startpage
  - MyWay
  - T-Online
- Domain deduplication
- Anti-captcha support (antipub integration)

#### 2. Vulnerability Scanner
- 100 concurrent threads
- 30-second timeouts
- Automated vulnerability detection

#### 3. Keyword Research
- Multi-engine keyword search
- Support for:
  - Google
  - Bing
  - Ask
  - DuckDuckGo
  - Yahoo
  - Brave
- 100 threads with 15-second timeouts

#### 4. Dork Generator
- Pattern-based dork generation
- Configurable limits (50,000)
- Custom pattern support

#### 5. Dork Checker
- Validates generated dorks
- 100 threads
- Threshold-based filtering (minimum 5 results)

### Network Capabilities
The binary includes extensive networking code with references to:
- HTTP/HTTPS protocols
- TLS/SSL support
- WebSocket connections
- Multiple user agents
- Cookie handling
- Proxy support

### Windows API Usage
Key Windows APIs imported:
- `kernel32.dll` - Core Windows functionality
- `ntdll.dll` - NT kernel interface
- `api-ms-win-core-synch-l1-2-0.dll` - Synchronization primitives

### Cryptographic Features
Includes cryptographic random number generation:
- RtlGenRandom (Windows)
- RDRAND instruction support
- SecRandomCopyBytes (iOS/macOS compatibility layer)
- Web Crypto API fallback
- Node.js crypto module support

## Rust-Specific Elements

### Standard Library Components
The binary includes standard Rust library components:
- `std::sync::once` - Thread synchronization
- `alloc::collections::vec_deque` - Double-ended queue
- `core::slice::index` - Slice indexing
- `std::sys::pal::windows::io` - Windows I/O
- `core::io::borrowed_buf` - Buffered I/O

### Memory Safety
As a Rust program, the binary benefits from:
- Compile-time memory safety guarantees
- Bounds checking (visible in panic strings)
- Safe concurrency patterns

## Security Considerations

### Identified Security Features
- Modern Windows security features enabled (NX, PIE)
- HTTPS/TLS support for secure communications
- Cryptographically secure random number generation
- Input validation (visible in panic messages)

### Potential Security Concerns
- No stack canary protection
- Network communication capabilities
- Web scraping/crawling functionality
- Vulnerability scanning features

## Usage Patterns

Based on string analysis, the tool appears to:
1. Load configuration from `config.json`
2. Initialize multiple worker threads
3. Connect to various web services and search engines
4. Parse and analyze web content
5. Store results locally
6. Support command-line operation (Console UI)

## Reconstruction Notes

### Limitations of Decompilation
As a compiled Rust binary:
- Original variable names are not preserved
- High-level abstractions are compiled away
- Generic code is monomorphized
- Some optimizations obscure original logic

### Available Information
- Function boundaries are identifiable
- String literals are preserved
- API calls are visible
- Control flow can be reconstructed
- Data structures can be inferred

## Conclusion

This is a comprehensive tool suite for web reconnaissance and security testing, written in Rust for performance and safety. The complete disassembly and extracted artifacts provide maximum insight into the binary's functionality without access to the original source code.

All decompilation artifacts are provided in machine-readable formats suitable for further analysis:
- Assembly code for low-level analysis
- Strings for understanding text/messages
- Import/Export tables for API usage
- Binary metadata for platform information

This represents the maximum achievable decompilation using industry-standard reverse engineering tools (radare2, objdump) on a native Rust Windows executable.
