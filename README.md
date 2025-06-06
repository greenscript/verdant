# 🌱 Verdant v2.1

**Advanced Markdown Compression for AI Consumption**

Verdant is a specialized tool that compresses markdown files into dense, AI-readable format while preserving all important content. Perfect for fitting large documentation sets into AI context windows with intelligent chunking and model-specific optimization.

## ✨ Features

- **AI-Optimized Compression**: Reduces token usage while maintaining readability
- **Intelligent Chunking**: Split large docs into digestible AI-friendly chunks with navigation
- **Model-Specific Optimization**: Tailored compression for Claude, GPT, and GitHub Copilot
- **Chronological Organization**: Files automatically sorted by modification date for logical context flow
- **Token Optimization**: Emoji removal and content streamlining for maximum efficiency
- **Markdown-Aware**: Understands document structure and formatting
- **Duplicate Detection**: Removes redundant content across multiple files
- **Progressive Compression**: Four levels (low, medium, high, extreme) for different needs
- **Detailed Statistics**: Track compression ratios and token savings
- **Batch Processing**: Combine multiple .md files with intelligent organization

## 🚀 New in v2.1

- **📅 Smart File Ordering**: Chronological sorting by default creates logical progression (oldest → newest)
- **🚫 Token Efficiency**: Automatic emoji removal saves tokens for actual content
- **🧠 Better AI Context**: Files ordered by modification time help AI understand project evolution
- **⚡ Optimized Defaults**: Best practices enabled by default, advanced users can override

## 🆕 Previous Updates (v2.0)

- **🔗 Smart Chunking**: Automatically splits large outputs with navigation links
- **🤖 Model Targeting**: Optimized compression for specific AI models
- **⚡ Extreme Mode**: Ultra-aggressive compression with AI-optimized abbreviations
- **📊 Enhanced Analytics**: Detailed statistics and token estimation
- **🧠 Context Awareness**: Maintains document relationships across chunks

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
# Compress all .md files in a directory (optimized defaults)
verdant --input ./docs --output compressed

# Enable chunking for large documentation sets
verdant --input ./docs --output docs --chunk --max-lines 800

# Optimize for specific AI models
verdant --input ./docs --output claude_docs --model claude --chunk

# Maximum compression with AI optimizations
verdant --input ./docs --output compressed --level extreme --ai-mode
```

### Advanced Examples

```bash
# GitHub Copilot optimization with aggressive chunking
verdant -i ./api-docs -o copilot --model copilot --chunk --max-lines 600 --level high

# Claude optimization with detailed stats and chronological ordering
verdant -i ./guides -o claude_guide --model claude --chunk --ai-mode --stats

# Custom ordering and emoji preservation (override defaults)
verdant -i ./wiki -o wiki --no-chronological --emojis --level medium
```

### Options

#### Core Options
- `--input, -i`: Input directory containing .md files (required)
- `--output, -o`: Output file path/prefix (default: `compressed`)
- `--level, -l`: Compression level - `low`, `medium`, `high`, `extreme` (default: `medium`)
- `--stats, -s`: Show detailed compression statistics

#### Chunking Options
- `--chunk`: Enable chunking (splits large outputs into smaller files)
- `--max-lines`: Maximum lines per chunk when chunking enabled (default: `800`)

#### AI Optimization
- `--model`: Target AI model - `claude`, `gpt`, `copilot` (default: `claude`)
- `--ai-mode`: Enable AI-optimized extreme compression
- `--chronological`: Sort files by modification date (default: enabled)
- `--no-emojis`: Remove emojis to save tokens (default: enabled)

#### Override Defaults (Advanced)
- `--no-chronological`: Disable chronological sorting
- `--emojis`: Keep emojis in output

### Compression Levels

- **Low**: Basic whitespace removal and header compression
- **Medium**: + Code block compression, list optimization, fluff word removal, duplicate detection
- **High**: + Aggressive sentence compression and redundant phrase removal
- **Extreme**: + Article removal, abbreviations, mathematical notation

### Model-Specific Optimizations

- **Claude**: Structured data with technical notation, complex nested information
- **GPT**: Consistent formatting with explicit context markers, discrete chunks
- **Copilot**: Code-focused compression, file-type hints, smaller context windows

## Example Output

```
🌱 verdant v2.1
  Compressing markdown for AI consumption
  Target: claude | Level: medium | Chunking: enabled | Chronological: enabled | Emoji removal: enabled

Input: ./docs
Output: compressed_chunk_*.md

Found 20 markdown files:
📅 Files sorted chronologically (oldest → newest)
  📄 ./docs/ARCHITECTURE.md
  📄 ./docs/API_DESIGN.md  
  📄 ./docs/RECENT_FEATURES.md

🔄 Removing duplicate content across files...
   ✂️  Removed 25 duplicate paragraphs

🚫 Removed 127 emojis (~254 tokens saved)

📦 Creating 8 chunks of ~800 lines each...
  ✅ Created compressed_chunk_1.md
  ✅ Created compressed_chunk_2.md
  ✅ Created compressed_chunk_3.md

📊 COMPRESSION RESULTS:
   Created 8 chunks
   6477 lines → 3039 lines (53.1% reduction)
   217852 chars → 208637 chars (4.2% reduction)
   Est. tokens: 54463 → 52159 (saved ~2304)
```

## How It Works

Verdant applies multiple compression strategies in this order:

### Smart Organization (New in v2.1)
1. **Chronological Sorting**: Orders files by modification date for logical progression
2. **Token Optimization**: Removes emojis and unnecessary visual elements

### Core Compression
3. **Whitespace Optimization**: Removes excessive spacing and empty lines
4. **Header Compression**: `# Title` → `H1:Title`
5. **Code Block Compression**: Condenses code while preserving functionality
6. **List Optimization**: Streamlines bullet points and numbered lists
7. **Duplicate Detection**: Removes identical paragraphs across files

### Advanced Compression
8. **Fluff Removal**: Eliminates verbose phrases and connectors
9. **Sentence Compression**: Removes redundant words and phrases
10. **AI Abbreviations**: Common terms → concise notation (function → FN)
11. **Mathematical Notation**: "returns" → "→", "therefore" → "∴"

### Intelligent Chunking
12. **Smart Splitting**: Breaks documents at logical boundaries
13. **Navigation Links**: Each chunk links to the next for continuity
14. **Context Preservation**: Maintains document relationships
15. **Metadata Tracking**: Lines, tokens, and content estimates per chunk

## Use Cases

- **AI Context Optimization**: Fit more documentation into ChatGPT/Claude prompts
- **Daily Development**: Quick context preparation for AI pair programming
- **Large Codebase Analysis**: Compress extensive documentation for AI code reviews
- **Knowledge Base Compression**: Condense large wikis for AI consumption
- **Documentation Analysis**: Prepare docs for AI-powered analysis
- **Context Window Management**: Maximize information density for LLMs
- **Model-Specific Preparation**: Optimize content for different AI models

## Example Compression Results

Typical results on large documentation sets:
- **Characters**: 4-15% reduction
- **Lines**: 50-70% reduction  
- **Tokens**: 1,500-3,000 saved per 100KB of docs
- **Emojis**: 100-500 tokens saved on typical documentation
- **Chunks**: 3,000+ line files → 8 manageable chunks
- **Readability**: Fully preserved for AI consumption

## Perfect For

- **Daily Development**: Quick context preparation for AI pair programming
- **Documentation Teams**: Preparing large docs for AI analysis with logical flow
- **Technical Writing**: Optimizing content for AI-powered tools
- **Code Reviews**: Condensing project documentation for AI assistance
- **Knowledge Management**: Making large wikis AI-accessible
- **Project Handoffs**: Creating chronologically organized context for new team members

## Why Verdant?

**The Problem**: Modern documentation sets are too large for AI context windows, causing:
- Truncated responses
- Lost context
- AI models getting overwhelmed
- Slow processing times
- Poor understanding of project evolution

**The Solution**: Verdant intelligently compresses and organizes documentation while:
- Preserving all important information
- Maintaining logical chronological structure
- Adding navigation between chunks
- Optimizing for specific AI models
- Removing token waste (emojis, redundancy)
- Providing detailed analytics

## Smart Defaults Philosophy

Verdant v2.1 embraces "smart by default" design:
- **Chronological ordering** helps AI understand project progression
- **Emoji removal** maximizes content density
- **Medium compression** balances efficiency with safety
- **Claude targeting** works well for most AI models
- **Advanced users** can override any default with flags

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [Rust](https://www.rust-lang.org/) for maximum performance
- CLI powered by [clap](https://github.com/clap-rs/clap) for excellent UX
- Regex processing via [regex](https://github.com/rust-lang/regex) for reliable text processing
- File traversal with [walkdir](https://github.com/BurntSushi/walkdir) for efficient directory handling

---

**Made with 🌱 for the AI-powered developer community**

*"Because your documentation shouldn't fight your AI assistant"*

### Version History
- **v2.1**: Smart defaults (chronological ordering, emoji removal), enhanced UX
- **v2.0**: Chunking, model-specific optimization, extreme compression modes
- **v1.0**: Core compression engine, duplicate detection, basic statistics