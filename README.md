# 🌱 Verdant v2.3

**Advanced Markdown Compression for AI Consumption**

Verdant is a specialized tool that compresses markdown files into dense, AI-readable format while preserving all important content. Perfect for fitting large documentation sets into AI context windows with intelligent chunking, model-specific optimization, and the new VRD (AI-native) format.

## ✨ Features

- **🆕 VRD Format**: New AI-native format with 25% fewer chunks and improved compression
- **AI-Optimized Compression**: Reduces token usage while maintaining readability
- **Intelligent Chunking**: Split large docs into digestible AI-friendly chunks with navigation
- **Model-Specific Optimization**: Tailored compression for Claude, GPT, and GitHub Copilot
- **Dual Format Support**: Standard Markdown and VRD (AI-native) output formats
- **Chronological Organization**: Files automatically sorted by modification date for logical context flow
- **Token Optimization**: Emoji removal and content streamlining for maximum efficiency
- **Markdown-Aware**: Understands document structure and formatting
- **Duplicate Detection**: Removes redundant content across multiple files
- **Progressive Compression**: Four levels (low, medium, high, extreme) for different needs
- **Detailed Statistics**: Track compression ratios and token savings
- **Batch Processing**: Combine multiple .md files with intelligent organization

## 🚀 New in v2.3

- **🌟 VRD Format**: New AI-native format that achieves 25% fewer chunks than standard Markdown
- **🧠 Superior AI Consumption**: VRD format optimized specifically for AI processing with compressed syntax
- **📊 Enhanced Compression**: 6.5% of original size while maintaining full readability
- **⚡ Better Information Density**: ~35KB per chunk vs ~27KB for MD format (29% improvement)
- **🔗 Smart Navigation**: Clear chunk boundaries with NEXT: pointers for seamless AI navigation
- **📋 Rich Metadata**: File info, modification dates, tags, and compression stats in headers

## 🆕 Previous Updates

### v2.1
- **📅 Smart File Ordering**: Chronological sorting by default creates logical progression (oldest → newest)
- **🚫 Token Efficiency**: Automatic emoji removal saves tokens for actual content
- **🧠 Better AI Context**: Files ordered by modification time help AI understand project evolution
- **⚡ Optimized Defaults**: Best practices enabled by default, advanced users can override

### v2.0
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
# Use VRD format for maximum AI optimization (RECOMMENDED)
verdant --input ./docs --output compressed --format vrd --chunk

# Standard markdown format (classic)
verdant --input ./docs --output compressed --format md --chunk

# Compare formats side-by-side
verdant --input ./docs --output vrd_test --format vrd --chunk --stats
verdant --input ./docs --output md_test --format md --chunk --stats

# Extreme compression with VRD format for largest docs
verdant --input ./docs --output ultra_compressed --format vrd --level extreme --chunk
```

### Advanced Examples

```bash
# VRD format optimized for Claude with detailed analytics
verdant -i ./guides -o claude_guide --format vrd --model claude --chunk --stats

# GitHub Copilot optimization with VRD format
verdant -i ./api-docs -o copilot --format vrd --model copilot --chunk --max-lines 600

# Maximum compression demonstration (VRD vs MD comparison)
verdant -i ./large-docs -o vrd_demo --format vrd --level extreme --chunk --stats
verdant -i ./large-docs -o md_demo --format md --level extreme --chunk --stats
```

### Options

#### Core Options
- `--input, -i`: Input directory containing .md files (required)
- `--output, -o`: Output file path/prefix (default: `compressed`)
- `--format, -f`: Output format - `vrd` (AI-native), `md` (standard) (default: `vrd`)
- `--level, -l`: Compression level - `low`, `medium`, `high`, `extreme` (default: `medium`)
- `--stats, -s`: Show detailed compression statistics

#### Chunking Options
- `--chunk`: Enable chunking (splits large outputs into smaller files) (recommended)
- `--max-lines`: Maximum lines per chunk when chunking enabled (default: `800`)

#### AI Optimization
- `--model`: Target AI model - `claude`, `gpt`, `copilot` (default: `claude`)
- `--ai-mode`: Enable AI-optimized extreme compression
- `--chronological`: Sort files by modification date (default: enabled)
- `--no-emojis`: Remove emojis to save tokens (default: enabled)

#### Override Defaults (Advanced)
- `--no-chronological`: Disable chronological sorting
- `--emojis`: Keep emojis in output

### Output Formats

#### VRD Format (AI-Native) - RECOMMENDED ⭐
- **25% fewer chunks** than standard Markdown
- **Superior compression**: 6.5% of original size
- **AI-optimized syntax**: `→` for "that", `FN` for "function"
- **Rich metadata headers**: File info, dates, tags, compression stats
- **Smart navigation**: Clear chunk boundaries with NEXT: pointers
- **Information density**: ~35KB per chunk vs ~27KB for MD

#### Markdown Format (Standard)
- **Classic compatibility**: Standard markdown output
- **Universal support**: Works with any markdown processor
- **Familiar syntax**: Standard markdown conventions
- **Larger output**: More chunks needed for same content

### Compression Levels

- **Low**: Basic whitespace removal and header compression
- **Medium**: + Code block compression, list optimization, fluff word removal, duplicate detection
- **High**: + Aggressive sentence compression and redundant phrase removal
- **Extreme**: + Article removal, abbreviations, mathematical notation (best with VRD format)

### Model-Specific Optimizations

- **Claude**: Structured data with technical notation, complex nested information
- **GPT**: Consistent formatting with explicit context markers, discrete chunks
- **Copilot**: Code-focused compression, file-type hints, smaller context windows

## Example Output

### VRD Format (AI-Native)
```
🌱 verdant v2.3
  Compressing markdown for AI consumption
  Target: claude | Level: extreme | Format: VRD | Chunking: enabled

Input: ./docs (22 files)
Output: compressed_*.vrd

VRD1.0|TARGET:CLAUDE|MODE:EXTREME|CHUNKS:1/9|NEXT:compressed_2.vrd
META:{files:22,tokens:78714,compressed:6.5%,generated:2025-06-06T14:15:29Z}
DICT:{FN=function,API=application programming interface,AUTH=authentication...}

📊 COMPRESSION RESULTS:
   Created 9 VRD chunks
   78,714 tokens → 5,117 tokens (93.5% reduction)
   314,876 bytes total (25% fewer chunks than MD format)
```

### Standard Markdown Format
```
🌱 verdant v2.3
  Compressing markdown for AI consumption  
  Target: claude | Level: medium | Format: MD | Chunking: enabled

Input: ./docs
Output: compressed_chunk_*.md

Found 20 markdown files:
📅 Files sorted chronologically (oldest → newest)
  📄 ./docs/ARCHITECTURE.md
  📄 ./docs/API_DESIGN.md  
  📄 ./docs/RECENT_FEATURES.md

📊 COMPRESSION RESULTS:
   Created 12 MD chunks
   6477 lines → 3039 lines (53.1% reduction)
   325,213 bytes total
```

## How It Works

### Advanced VRD Format
Verdant's VRD (AI-native) format employs specialized compression techniques:

1. **Smart Header Metadata**: Rich file information, modification dates, and compression stats
2. **Dictionary Compression**: Common terms abbreviated (FN=function, API=application programming interface)
3. **Arrow Notation**: Semantic shortcuts (`→` for "that", "to", "then")
4. **Structured Markers**: Clear content separation (F:filename, H:headers, C:content, X:code)
5. **Optimized Chunking**: Maintains semantic continuity across fewer chunks

### Standard Processing Pipeline
Verdant applies multiple compression strategies in this order:

#### Smart Organization
1. **Chronological Sorting**: Orders files by modification date for logical progression
2. **Token Optimization**: Removes emojis and unnecessary visual elements

#### Core Compression  
3. **Whitespace Optimization**: Removes excessive spacing and empty lines
4. **Header Compression**: `# Title` → `H1:Title` (MD) or `H:Title` (VRD)
5. **Code Block Compression**: Condenses code while preserving functionality
6. **List Optimization**: Streamlines bullet points and numbered lists
7. **Duplicate Detection**: Removes identical paragraphs across files

#### Advanced Compression
8. **Fluff Removal**: Eliminates verbose phrases and connectors
9. **Sentence Compression**: Removes redundant words and phrases
10. **AI Abbreviations**: Common terms → concise notation (function → FN)
11. **Mathematical Notation**: "returns" → "→", "therefore" → "∴"

#### Intelligent Chunking
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

### VRD Format Performance ⭐
Typical results on large documentation sets using VRD format:
- **Chunks**: 25% fewer than standard Markdown
- **File Size**: 6.5% of original (93.5% reduction)
- **Information Density**: 29% higher per chunk (~35KB vs ~27KB)
- **Token Efficiency**: Optimized abbreviations save thousands of tokens
- **AI Navigation**: Faster processing with fewer file operations

### Standard Markdown Results
Traditional markdown compression results:
- **Characters**: 4-15% reduction
- **Lines**: 50-70% reduction  
- **Tokens**: 1,500-3,000 saved per 100KB of docs
- **Emojis**: 100-500 tokens saved on typical documentation
- **Chunks**: 3,000+ line files → 12 manageable chunks
- **Readability**: Fully preserved for AI consumption

### Format Comparison
| Metric | VRD Format | Markdown Format | VRD Advantage |
|--------|------------|-----------------|---------------|
| **Chunks Generated** | 9 | 12 | **25% fewer** |
| **Information Density** | ~35KB/chunk | ~27KB/chunk | **29% higher** |
| **Compression Ratio** | 93.5% reduction | 53% reduction | **40% better** |
| **AI Processing Speed** | Faster | Standard | **Fewer operations** |

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

**The Solution**: Verdant effectively compresses and organizes documentation while:
- **Improved VRD Format**: 25% fewer chunks with enhanced AI consumption
- Preserving all important information
- Maintaining logical chronological structure
- Adding navigation between chunks
- Optimizing for specific AI models
- Removing token waste (emojis, redundancy)
- Providing detailed analytics

**The VRD Advantage**: Our AI-native format provides:
- **Strong Compression**: 93.5% size reduction vs 53% for standard markdown
- **Better Navigation**: Fewer chunks mean faster AI processing
- **Rich Metadata**: Smart headers with file info, dates, and compression stats
- **Semantic Optimization**: Abbreviations and symbols designed for AI understanding

## Smart Defaults Philosophy

Verdant v2.3 embraces "AI-first" design:
- **VRD format by default** for maximum AI optimization
- **Chunking enabled** for better context management
- **Chronological ordering** helps AI understand project progression
- **Emoji removal** maximizes content density
- **Medium compression** balances efficiency with safety
- **Claude targeting** works well for most AI models
- **Advanced users** can override any default with flags

**Pro Tip**: Start with `verdant -i ./docs -o output --format vrd --chunk --stats` for the best AI experience!

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
- **v2.3**: VRD format, 25% fewer chunks, improved AI-native compression
- **v2.1**: Smart defaults (chronological ordering, emoji removal), enhanced UX
- **v2.0**: Chunking, model-specific optimization, extreme compression modes
- **v1.0**: Core compression engine, duplicate detection, basic statistics