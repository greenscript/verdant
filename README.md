# 🌱 Verdant

**Markdown Compression for AI Consumption**

Verdant is a specialized tool that compresses markdown files into dense, AI-readable format while preserving all important content. Perfect for fitting large documentation sets into AI context windows.

## Features

- **AI-Optimized Compression**: Reduces token usage while maintaining readability
- **Markdown-Aware**: Understands document structure and formatting
- **Duplicate Detection**: Removes redundant content across multiple files
- **Progressive Compression**: Three levels (low, medium, high) for different needs
- **Detailed Statistics**: Track compression ratios and token savings
- **Batch Processing**: Combine multiple .md files into one compressed output

## Installation

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable version)

### Quick Install

1. **Clone the repository:**
   ```bash
   git clone https://github.com/yourusername/verdant.git
   cd verdant
   ```

2. **Run the install script:**
   ```bash
   chmod +x install.sh
   ./install.sh
   ```

3. **Add to PATH** (if not already done):
   ```bash
   echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc
   source ~/.zshrc
   ```

### Manual Installation

```bash
# Build the optimized binary
cargo build --release

# Copy to a directory in your PATH
cp target/release/verdant ~/.local/bin/

# Or install system-wide (requires sudo)
sudo cp target/release/verdant /usr/local/bin/
```

## Usage

### Basic Usage

```bash
# Compress all .md files in a directory
verdant --input ./docs --output compressed.md

# Specify compression level
verdant --input ./docs --output compressed.md --level high

# Show detailed statistics
verdant --input ./docs --output compressed.md --level high --stats
```

### Options

- `--input, -i`: Input directory containing .md files (required)
- `--output, -o`: Output file path (default: `compressed.md`)
- `--level, -l`: Compression level - `low`, `medium`, `high` (default: `medium`)
- `--stats, -s`: Show detailed compression statistics

### Compression Levels

- **Low**: Basic whitespace removal and header compression
- **Medium**: + Code block compression, list optimization, fluff word removal, duplicate detection
- **High**: + Aggressive sentence compression and redundant phrase removal

## Example Output

```
🌱 verdant
  Compressing markdown for AI consumption
  by greenscript
  Version 1.0

Input: ./docs
Output: compressed.md
Compression: high

Found 15 markdown files:
  📄 ./docs/README.md
  📄 ./docs/API.md
  📄 ./docs/GUIDE.md

🔄 Removing duplicate content across files...
   ✂️  Removed 23 duplicate paragraphs

✅ Successfully compressed and wrote to compressed.md
📊 Original: 134080 chars, Compressed: 127922 chars
📈 Compression ratio: 4.6%
📄 Lines: 4400 → 1767 (59.8% reduction)
```

## How It Works

Verdant applies multiple compression strategies:

1. **Whitespace Optimization**: Removes excessive spacing and empty lines
2. **Header Compression**: `# Title` → `H1:Title`
3. **Code Block Compression**: Condenses code while preserving functionality
4. **List Optimization**: Streamlines bullet points and numbered lists
5. **Duplicate Detection**: Removes identical paragraphs across files
6. **Fluff Removal**: Eliminates verbose phrases and connectors
7. **Sentence Compression**: Removes redundant words and phrases

## Use Cases

- **AI Context Optimization**: Fit more documentation into ChatGPT/Claude prompts
- **Knowledge Base Compression**: Condense large wikis for AI consumption
- **Documentation Analysis**: Prepare docs for AI-powered analysis
- **Context Window Management**: Maximize information density for LLMs

## Example Compression Results

Typical results on documentation sets:
- **Characters**: 5-15% reduction
- **Lines**: 50-70% reduction  
- **Tokens**: ~1,500-3,000 saved per 100KB of docs
- **Readability**: Fully preserved for AI

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- CLI powered by [clap](https://github.com/clap-rs/clap)
- Regex processing via [regex](https://github.com/rust-lang/regex)
- File traversal with [walkdir](https://github.com/BurntSushi/walkdir)

---

**Made with 🌱 for the AI-powered developer community**