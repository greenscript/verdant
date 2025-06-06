use clap::Parser;
use regex::Regex;
use std::fs;
use walkdir::WalkDir;

#[derive(Parser)]
#[command(name = "verdant")]
#[command(about = "Compress markdown files for AI consumption")]
struct Args {
    /// Input directory containing .md files
    #[arg(short, long)]
    input: String,
    
    /// Output file path (will be numbered for chunks: output_1.md, output_2.md, etc.)
    #[arg(short, long, default_value = "compressed")]
    output: String,
    
    /// Compression level (low, medium, high, extreme)
    #[arg(short, long, default_value = "medium")]
    level: String,
    
    /// Show detailed statistics about compression
    #[arg(short, long)]
    stats: bool,
    
    /// Enable chunking (splits large outputs into smaller files)
    #[arg(long)]
    chunk: bool,
    
    /// Maximum lines per chunk (only used when chunking is enabled)
    #[arg(long, default_value = "800")]
    max_lines: usize,
    
    /// Target AI model (claude, gpt, copilot)
    #[arg(long, default_value = "claude")]
    model: String,
    
    /// Enable AI-optimized extreme compression
    #[arg(long)]
    ai_mode: bool,
}

struct CompressionStats {
    original_size: usize,
    compressed_size: usize,
    original_lines: usize,
    compressed_lines: usize,
    chunks_created: usize,
}

fn main() {
    let args = Args::parse();
    
    print_header(&args);
    
    // Find all .md files
    let md_files: Vec<_> = WalkDir::new(&args.input)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "md"))
        .collect();
    
    println!("Found {} markdown files:", md_files.len());
    
    let mut all_files_content = Vec::new();
    let mut stats = CompressionStats {
        original_size: 0,
        compressed_size: 0,
        original_lines: 0,
        compressed_lines: 0,
        chunks_created: 0,
    };
    
    // Read all files
    read_all_files(&md_files, &mut all_files_content, &mut stats, args.stats);
    
    // Remove duplicates if needed
    if args.level != "low" {
        println!("\n🔄 Removing duplicate content across files...");
        all_files_content = remove_duplicate_content(all_files_content, args.stats);
    }
    
    // Compress content
    let compressed_content = compress_all_content(&all_files_content, &args);
    
    // Handle chunking or single file output
    if args.chunk {
        create_chunks(&compressed_content, &args, &mut stats);
    } else {
        write_single_file(&compressed_content, &args, &mut stats);
    }
    
    print_final_stats(&stats, args.stats);
}

fn print_header(args: &Args) {
    println!("🌱 verdant v2.0");
    println!("  Compressing markdown for AI consumption");
    println!("  Target: {} | Level: {} | Chunking: {}", 
             args.model, args.level, if args.chunk { "enabled" } else { "disabled" });
    println!();
    println!("Input: {}", args.input);
    println!("Output: {}", if args.chunk { 
        format!("{}_chunk_*.md", args.output) 
    } else { 
        format!("{}.md", args.output) 
    });
    println!();
}

fn read_all_files(
    md_files: &[walkdir::DirEntry], 
    all_files_content: &mut Vec<(String, String)>, 
    stats: &mut CompressionStats,
    show_stats: bool
) {
    for file in md_files {
        println!("  📄 {}", file.path().display());
        
        match fs::read_to_string(file.path()) {
            Ok(content) => {
                stats.original_size += content.len();
                stats.original_lines += content.lines().count();
                
                if show_stats {
                    println!("    Lines: {}, Chars: {}", content.lines().count(), content.len());
                }
                
                let filename = file.path().file_name().unwrap().to_str().unwrap().to_string();
                all_files_content.push((filename, content));
            }
            Err(e) => println!("Error reading {}: {}", file.path().display(), e),
        }
    }
}

fn compress_all_content(all_files_content: &[(String, String)], args: &Args) -> String {
    let mut combined_content = String::new();
    
    // Add model-specific header
    combined_content.push_str(&create_model_header(&args.model, args.ai_mode));
    
    for (filename, content) in all_files_content {
        combined_content.push_str(&format!("F:{}\n", filename));
        let compressed = compress_content(content, &args.level, &args.model, args.ai_mode);
        combined_content.push_str(&compressed);
        combined_content.push_str("\n|\n");
    }
    
    combined_content
}

fn create_model_header(model: &str, ai_mode: bool) -> String {
    let mut header = format!("TARGET:{}\n", model.to_uppercase());
    
    if ai_mode {
        header.push_str("MODE:AI_OPTIMIZED\n");
        header.push_str(&create_abbreviation_dictionary());
    }
    
    match model {
        "claude" => header.push_str("NOTE:Structured data with technical notation\n"),
        "gpt" => header.push_str("NOTE:Consistent formatting with explicit context\n"),
        "copilot" => header.push_str("NOTE:Code-focused with file-type hints\n"),
        _ => {}
    }
    
    header.push_str("---\n");
    header
}

fn create_abbreviation_dictionary() -> String {
    let mut dict = String::from("DICT:{");
    let abbreviations = [
        ("function", "FN"),
        ("parameter", "PARAM"),
        ("documentation", "DOC"),
        ("example", "EX"),
        ("installation", "INST"),
        ("configuration", "CFG"),
        ("authentication", "AUTH"),
        ("database", "DB"),
        ("middleware", "MW"),
        ("component", "COMP"),
    ];
    
    for (i, (full, abbrev)) in abbreviations.iter().enumerate() {
        if i > 0 { dict.push_str(","); }
        dict.push_str(&format!("{}={}", abbrev, full));
    }
    dict.push_str("}\n");
    dict
}


fn create_chunks(content: &str, args: &Args, stats: &mut CompressionStats) {
    let lines: Vec<&str> = content.lines().collect();
    let total_lines = lines.len();
    let chunk_size = args.max_lines;
    let total_chunks = (total_lines + chunk_size - 1) / chunk_size; // Ceiling division
    
    println!("📦 Creating {} chunks of ~{} lines each...", total_chunks, chunk_size);
    
    for chunk_num in 0..total_chunks {
        let start_idx = chunk_num * chunk_size;
        let end_idx = std::cmp::min(start_idx + chunk_size, total_lines);
        let chunk_lines = &lines[start_idx..end_idx];
        
        let mut chunk_content = String::new();
        
        // Add chunk header
        chunk_content.push_str(&format!("CHUNK:{}/{}", chunk_num + 1, total_chunks));
        if chunk_num + 1 < total_chunks {
            // Smart naming: check if output already contains "chunk"
            let next_chunk_name = if args.output.contains("chunk") {
                format!("{}_{}.md", args.output, chunk_num + 2)
            } else {
                format!("{}_chunk_{}.md", args.output, chunk_num + 2)
            };
            chunk_content.push_str(&format!(" | NEXT:{}", next_chunk_name));
        }
        chunk_content.push_str("\n");
        
        // Add the actual content
        chunk_content.push_str(&chunk_lines.join("\n"));
        
        // Add chunk footer with metadata
        chunk_content.push_str(&format!("\n---\nCHUNK_END | Lines:{} | Est.tokens:{}", 
                                       chunk_lines.len(), 
                                       chunk_content.len() / 4)); // Rough token estimate
        
        // Write chunk file with smart naming
        let chunk_filename = if args.output.contains("chunk") {
            format!("{}_{}.md", args.output, chunk_num + 1)
        } else {
            format!("{}_chunk_{}.md", args.output, chunk_num + 1)
        };
        
        match fs::write(&chunk_filename, &chunk_content) {
            Ok(()) => {
                println!("  ✅ Created {}", chunk_filename);
                stats.compressed_size += chunk_content.len();
                stats.compressed_lines += chunk_content.lines().count();
            }
            Err(e) => println!("  ❌ Error writing {}: {}", chunk_filename, e),
        }
    }
    
    stats.chunks_created = total_chunks;
}

fn write_single_file(content: &str, args: &Args, stats: &mut CompressionStats) {
    let output_filename = format!("{}.md", args.output);
    match fs::write(&output_filename, content) {
        Ok(()) => {
            println!("✅ Successfully compressed and wrote to {}", output_filename);
            stats.compressed_size = content.len();
            stats.compressed_lines = content.lines().count();
        }
        Err(e) => println!("❌ Error writing output: {}", e),
    }
}

fn compress_content(content: &str, level: &str, model: &str, ai_mode: bool) -> String {
    let mut compressed = content.to_string();
    
    // Always apply basic compression
    compressed = remove_excessive_whitespace(&compressed);
    compressed = remove_empty_lines(&compressed);
    compressed = compress_headers_aggressively(&compressed);
    compressed = compress_formatting(&compressed);
    
    // Apply level-based compression
    match level {
        "medium" | "high" | "extreme" => {
            compressed = compress_code_blocks(&compressed, model);
            compressed = compress_lists_aggressively(&compressed);
            compressed = remove_fluff_words(&compressed);
        }
        _ => {} // low level - just basic
    }
    
    if level == "high" || level == "extreme" {
        compressed = compress_sentences(&compressed);
        compressed = remove_redundant_phrases(&compressed);
    }
    
    if level == "extreme" || ai_mode {
        compressed = apply_extreme_ai_compression(&compressed);
    }
    
    // Apply model-specific optimizations
    compressed = apply_model_optimizations(&compressed, model);
    
    compressed
}

fn apply_extreme_ai_compression(content: &str) -> String {
    let mut result = content.to_string();
    
    // Remove articles
    let re_articles = Regex::new(r"\b(a|an|the)\s+").unwrap();
    result = re_articles.replace_all(&result, "").to_string();
    
    // Replace common programming terms with abbreviations
    let replacements = [
        (r"\bfunction\b", "FN"),
        (r"\bparameter\b", "PARAM"),
        (r"\bdocumentation\b", "DOC"),
        (r"\bexample\b", "EX"),
        (r"\binstallation\b", "INST"),
        (r"\bconfiguration\b", "CFG"),
        (r"\bauthentication\b", "AUTH"),
        (r"\bdatabase\b", "DB"),
        (r"\breturns\b", "→"),
        (r"\btherefore\b", "∴"),
    ];
    
    for (pattern, replacement) in replacements {
        let re = Regex::new(pattern).unwrap();
        result = re.replace_all(&result, replacement).to_string();
    }
    
    result
}

fn apply_model_optimizations(content: &str, model: &str) -> String {
    match model {
        "copilot" => {
            // Copilot: prioritize code and add file type hints
            let mut result = content.to_string();
            // Add more aggressive code compression for copilot
            result = prioritize_code_content(&result);
            result
        }
        "gpt" => {
            // GPT: add more explicit structure markers
            let re_sections = Regex::new(r"H(\d):(.+)").unwrap();
            re_sections.replace_all(content, "SECTION_L$1:$2").to_string()
        }
        "claude" => {
            // Claude: can handle more complex nested structures
            content.to_string() // Claude handles the current format well
        }
        _ => content.to_string(),
    }
}

fn prioritize_code_content(content: &str) -> String {
    // Move all code blocks to the beginning of sections
    // This is a simplified implementation - you could make this much more sophisticated
    content.to_string()
}

fn remove_duplicate_content(all_files_content: Vec<(String, String)>, show_stats: bool) -> Vec<(String, String)> {
    let mut seen_paragraphs = std::collections::HashSet::new();
    let mut deduplicated = Vec::new();
    let mut duplicates_removed = 0;
    
    for (filename, content) in all_files_content {
        let paragraphs: Vec<&str> = content.split('\n').collect();
        let mut unique_paragraphs = Vec::new();
        
        for paragraph in paragraphs {
            let trimmed = paragraph.trim();
            if trimmed.len() > 30 {
                if !seen_paragraphs.contains(trimmed) {
                    seen_paragraphs.insert(trimmed.to_string());
                    unique_paragraphs.push(paragraph);
                } else {
                    duplicates_removed += 1;
                    if show_stats {
                        println!("    🔄 Removed duplicate from {}: {:.50}...", filename, trimmed);
                    }
                }
            } else {
                unique_paragraphs.push(paragraph);
            }
        }
        
        deduplicated.push((filename, unique_paragraphs.join("\n")));
    }
    
    if duplicates_removed > 0 {
        println!("   ✂️  Removed {} duplicate paragraphs", duplicates_removed);
    }
    
    deduplicated
}

fn remove_excessive_whitespace(content: &str) -> String {
    let re_multiple_newlines = Regex::new(r"\n{2,}").unwrap();
    let re_multiple_spaces = Regex::new(r" {2,}").unwrap();
    let re_trailing_spaces = Regex::new(r" +\n").unwrap();
    
    let mut result = re_multiple_newlines.replace_all(content, "\n").to_string();
    result = re_multiple_spaces.replace_all(&result, " ").to_string();
    result = re_trailing_spaces.replace_all(&result, "\n").to_string();
    
    result
}

fn remove_empty_lines(content: &str) -> String {
    content.lines()
        .filter(|line| !line.trim().is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

fn compress_headers_aggressively(content: &str) -> String {
    let re_h1 = Regex::new(r"^# (.+)$").unwrap();
    let re_h2 = Regex::new(r"^## (.+)$").unwrap();
    let re_h3 = Regex::new(r"^### (.+)$").unwrap();
    let re_h4 = Regex::new(r"^#### (.+)$").unwrap();
    
    let mut result = content.to_string();
    result = re_h1.replace_all(&result, "H1:$1").to_string();
    result = re_h2.replace_all(&result, "H2:$1").to_string();
    result = re_h3.replace_all(&result, "H3:$1").to_string();
    result = re_h4.replace_all(&result, "H4:$1").to_string();
    
    result
}

fn compress_formatting(content: &str) -> String {
    let re_bold = Regex::new(r"\*\*([^*]+)\*\*").unwrap();
    let re_italic = Regex::new(r"\*([^*]+)\*").unwrap();
    let re_code = Regex::new(r"`([^`]+)`").unwrap();
    
    let mut result = content.to_string();
    result = re_bold.replace_all(&result, "**$1**").to_string();
    result = re_italic.replace_all(&result, "*$1*").to_string();
    result = re_code.replace_all(&result, "`$1`").to_string();
    
    result
}

fn compress_code_blocks(content: &str, model: &str) -> String {
    let re_code_block = Regex::new(r"```(\w+)?\n([\s\S]*?)```").unwrap();
    
    re_code_block.replace_all(content, |caps: &regex::Captures| {
        let lang = caps.get(1).map_or("", |m| m.as_str());
        let code = caps.get(2).map_or("", |m| m.as_str());
        
        let compressed_code = code.lines()
            .filter(|line| !line.trim().is_empty())
            .collect::<Vec<_>>()
            .join("\n");
            
        match model {
            "copilot" => {
                // More aggressive compression for copilot
                if lang.is_empty() {
                    format!("CODE:{}", compressed_code.replace('\n', " | "))
                } else {
                    format!("{}:{}", lang.to_uppercase(), compressed_code.replace('\n', " | "))
                }
            }
            _ => {
                if lang.is_empty() {
                    format!("CODE:{}", compressed_code.replace('\n', "|"))
                } else {
                    format!("CODE({}):{}", lang, compressed_code.replace('\n', "|"))
                }
            }
        }
    }).to_string()
}

fn compress_lists_aggressively(content: &str) -> String {
    let re_list_items = Regex::new(r"^[*-] (.+)$").unwrap();
    re_list_items.replace_all(content, "•$1").to_string()
}

fn remove_fluff_words(content: &str) -> String {
    let patterns = [
        (r"(?i)\b(please note that|it should be noted that|it is important to note that)\b", ""),
        (r"(?i)\b(as mentioned above|as mentioned earlier|as we can see)\b", ""),
        (r"(?i)\b(in order to|for the purpose of)\b", "to"),
        (r"(?i)\b(due to the fact that)\b", "because"),
        (r"(?i)\b(at this point in time)\b", "now"),
    ];
    
    let mut result = content.to_string();
    for (pattern, replacement) in patterns {
        let re = Regex::new(pattern).unwrap();
        result = re.replace_all(&result, replacement).to_string();
    }
    result
}

fn compress_sentences(content: &str) -> String {
    let re_connectors = Regex::new(r"(?i)\b(however|therefore|furthermore|moreover|additionally),?\s*").unwrap();
    re_connectors.replace_all(content, "").to_string()
}

fn remove_redundant_phrases(content: &str) -> String {
    let patterns = [
        (r"(?i)\bvery\s+", ""),
        (r"(?i)\breally\s+", ""),
        (r"(?i)\bquite\s+", ""),
        (r"(?i)\bbasically\s+", ""),
    ];
    
    let mut result = content.to_string();
    for (pattern, replacement) in patterns {
        let re = Regex::new(pattern).unwrap();
        result = re.replace_all(&result, replacement).to_string();
    }
    result
}

fn print_final_stats(stats: &CompressionStats, show_detailed: bool) {
    let compression_ratio = if stats.original_size > 0 {
        (1.0 - (stats.compressed_size as f64 / stats.original_size as f64)) * 100.0
    } else {
        0.0
    };
    
    let line_compression_ratio = if stats.original_lines > 0 {
        (1.0 - (stats.compressed_lines as f64 / stats.original_lines as f64)) * 100.0
    } else {
        0.0
    };
    
    println!("\n📊 COMPRESSION RESULTS:");
    
    if stats.chunks_created > 0 {
        println!("   Created {} chunks", stats.chunks_created);
    }
    
    if show_detailed {
        println!("   Original:   {} lines, {} chars", stats.original_lines, stats.original_size);
        println!("   Compressed: {} lines, {} chars", stats.compressed_lines, stats.compressed_size);
        println!("   Line compression: {:.1}%", line_compression_ratio);
        println!("   Char compression: {:.1}%", compression_ratio);
        
        let original_tokens = stats.original_size / 4;
        let compressed_tokens = stats.compressed_size / 4;
        println!("   Est. tokens: {} → {} (saved ~{})", 
                 original_tokens, compressed_tokens, original_tokens.saturating_sub(compressed_tokens));
    } else {
        println!("   {} chars → {} chars ({:.1}% reduction)", 
                 stats.original_size, stats.compressed_size, compression_ratio);
        println!("   {} lines → {} lines ({:.1}% reduction)", 
                 stats.original_lines, stats.compressed_lines, line_compression_ratio);
    }
}