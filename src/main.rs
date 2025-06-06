use clap::Parser;
use regex::Regex;
use std::fs;
use std::time::SystemTime;
use walkdir::WalkDir;
use chrono::{DateTime, Utc};

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
    
    /// Sort files chronologically by modification date
    #[arg(long, default_value = "true")]
    chronological: bool,

    /// Remove emojis to save tokens  
    #[arg(long, default_value = "true")]  
    no_emojis: bool,

    /// Output format (md, vrd, json, yaml)
    #[arg(long, default_value = "md")]
    format: String,
}

struct CompressionStats {
    original_size: usize,
    compressed_size: usize,
    original_lines: usize,
    compressed_lines: usize,
    chunks_created: usize,
}

struct VrdFile {
    name: String,
    modified: DateTime<Utc>,
    size: usize,
    lines: usize,
    tags: Vec<String>,
    headers: Vec<String>,
    content: String,
    code_blocks: Vec<String>,
}

struct VrdMetadata {
    files_count: usize,
    estimated_tokens: usize,
    compression_ratio: f64,
    generated: DateTime<Utc>,
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
    
    // Read all files with optional chronological sorting
    read_all_files_with_sorting(&md_files, &mut all_files_content, &mut stats, args.stats, args.chronological);
    
    // Remove duplicates if needed
    if args.level != "low" {
        println!("\n🔄 Removing duplicate content across files...");
        all_files_content = remove_duplicate_content(all_files_content, args.stats);
    }
    
    // Show emoji removal stats if enabled
    if args.no_emojis {
        let emoji_count: usize = all_files_content.iter()
            .map(|(_, content, _)| count_emojis(content))
            .sum();
        if emoji_count > 0 {
            println!("🚫 Removed {} emojis (~{} tokens saved)", emoji_count, emoji_count * 2);
        }
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

fn count_emojis(content: &str) -> usize {
    // Quick emoji count for stats
    let emoji_regex = regex::Regex::new(r"[\u{1F600}-\u{1F64F}]|[\u{1F300}-\u{1F5FF}]|[\u{1F680}-\u{1F6FF}]|[\u{1F1E0}-\u{1F1FF}]|[\u{2600}-\u{26FF}]|[\u{2700}-\u{27BF}]").unwrap();
    emoji_regex.find_iter(content).count()
}


fn print_header(args: &Args) {
    println!("🌱 verdant v2.3");
    println!("  Compressing markdown for AI consumption");
    
    let mut features = vec![
        format!("Target: {}", args.model),
        format!("Level: {}", args.level),
        format!("Format: {}", args.format.to_uppercase()),
        format!("Chunking: {}", if args.chunk { "enabled" } else { "disabled" }),
    ];
    
    if args.chronological {
        features.push("Chronological: enabled".to_string());
    }
    
    if args.no_emojis {
        features.push("Emoji removal: enabled".to_string());
    }
    
    if args.ai_mode {
        features.push("AI mode: enabled".to_string());
    }
    
    println!("  {}", features.join(" | "));
    println!();
    println!("Input: {}", args.input);
    
    let extension = if args.format == "vrd" { "vrd" } else { "md" };
    println!("Output: {}", if args.chunk { 
        format!("{}_*.{}", args.output, extension) 
    } else { 
        format!("{}.{}", args.output, extension) 
    });
    println!();
}

fn read_all_files_with_sorting(
    md_files: &[walkdir::DirEntry], 
    all_files_content: &mut Vec<(String, String, std::path::PathBuf)>, // Add PathBuf
    stats: &mut CompressionStats,
    show_stats: bool,
    chronological: bool
) {
    let mut files_with_time: Vec<(walkdir::DirEntry, SystemTime)> = Vec::new();
    
    for file in md_files {
        if let Ok(metadata) = file.metadata() {
            let modified_time = metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH);
            files_with_time.push((file.clone(), modified_time));
        }
    }
    
    if chronological {
        files_with_time.sort_by(|a, b| a.1.cmp(&b.1));
        println!("📅 Files sorted chronologically (oldest → newest)");
    }
    
    for (file, _) in files_with_time {
        println!("  📄 {}", file.path().display());
        
        match fs::read_to_string(file.path()) {
            Ok(content) => {
                stats.original_size += content.len();
                stats.original_lines += content.lines().count();
                
                if show_stats {
                    println!("    Lines: {}, Chars: {}", content.lines().count(), content.len());
                }
                
                let filename = file.path().file_name().unwrap().to_str().unwrap().to_string();
                all_files_content.push((filename, content, file.path().to_path_buf()));
            }
            Err(e) => println!("Error reading {}: {}", file.path().display(), e),
        }
    }
}

fn remove_emojis(content: &str) -> String {
    use regex::Regex;
    
    let emoji_patterns = [
        r"[\u{1F600}-\u{1F64F}]", // Emoticons
        r"[\u{1F300}-\u{1F5FF}]", // Misc Symbols and Pictographs
        r"[\u{1F680}-\u{1F6FF}]", // Transport and Map
        r"[\u{1F1E0}-\u{1F1FF}]", // Regional indicators (flags)
        r"[\u{2600}-\u{26FF}]",   // Misc symbols
        r"[\u{2700}-\u{27BF}]",   // Dingbats
        r"[\u{1F900}-\u{1F9FF}]", // Supplemental Symbols and Pictographs
        r"[\u{1FA70}-\u{1FAFF}]", // Symbols and Pictographs Extended-A
    ];
    
    let mut result = content.to_string();
    for pattern in emoji_patterns {
        let re = Regex::new(pattern).unwrap();
        result = re.replace_all(&result, "").to_string();
    }
    
    result
}

fn compress_all_content(all_files_content: &[(String, String, std::path::PathBuf)], args: &Args) -> String {
    match args.format.as_str() {
        "vrd" => {
            // Warn if using VRD format with single file (inefficient due to overhead)
            if all_files_content.len() == 1 {
                println!("⚠️  WARNING: VRD format with single file may be less efficient due to format overhead.");
                println!("   Consider using regular markdown compression (remove --format vrd) for single files.");
                println!("   VRD format is optimized for multi-file documentation sets.\n");
            }
            
            // Calculate original stats for VRD
            let original_stats = CompressionStats {
                original_size: all_files_content.iter().map(|(_, c, _)| c.len()).sum(),
                compressed_size: 0,
                original_lines: all_files_content.iter().map(|(_, c, _)| c.lines().count()).sum(),
                compressed_lines: 0,
                chunks_created: 0,
            };
            generate_vrd_content(all_files_content, args, &original_stats)
        }
        "md" => {
            // Existing markdown generation...
            let mut combined_content = String::new();
            combined_content.push_str(&create_model_header(&args.model, args.ai_mode));
            
            for (filename, content, _) in all_files_content {
                combined_content.push_str(&format!("F:{}\n", filename));
                let compressed = compress_content(content, &args.level, &args.model, args.ai_mode, args.no_emojis);
                combined_content.push_str(&compressed);
                combined_content.push_str("\n|\n");
            }
            
            combined_content
        }
        _ => {
            println!("❌ Unsupported format: {}", args.format);
            std::process::exit(1);
        }
    }
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
    let total_chunks = (total_lines + chunk_size - 1) / chunk_size;
    
    println!("📦 Creating {} chunks of ~{} lines each...", total_chunks, chunk_size);
    
    for chunk_num in 0..total_chunks {
        let start_idx = chunk_num * chunk_size;
        let end_idx = std::cmp::min(start_idx + chunk_size, total_lines);
        let chunk_lines = &lines[start_idx..end_idx];
        
        let mut chunk_content = String::new();
        
        // For VRD format, don't add markdown-style chunk headers
        if args.format == "vrd" {
            // For VRD, update the header to reflect the chunk number
            let vrd_content = update_vrd_chunk_header(chunk_lines.join("\n"), chunk_num + 1, total_chunks, args);
            chunk_content = vrd_content;
        } else {
            // Original markdown chunking logic
            chunk_content.push_str(&format!("CHUNK:{}/{}", chunk_num + 1, total_chunks));
            if chunk_num + 1 < total_chunks {
                let next_chunk_name = if args.output.contains("chunk") {
                    format!("{}_{}.{}", args.output, chunk_num + 2, if args.format == "vrd" { "vrd" } else { "md" })
                } else {
                    format!("{}_chunk_{}.{}", args.output, chunk_num + 2, if args.format == "vrd" { "vrd" } else { "md" })
                };
                chunk_content.push_str(&format!(" | NEXT:{}", next_chunk_name));
            }
            chunk_content.push_str("\n");
            chunk_content.push_str(&chunk_lines.join("\n"));
            chunk_content.push_str(&format!("\n---\nCHUNK_END | Lines:{} | Est.tokens:{}", 
                                           chunk_lines.len(), 
                                           chunk_content.len() / 4));
        }
        
        // Write chunk file with correct extension
        let chunk_filename = if args.output.contains("chunk") {
            format!("{}_{}.{}", args.output, chunk_num + 1, if args.format == "vrd" { "vrd" } else { "md" })
        } else {
            format!("{}_chunk_{}.{}", args.output, chunk_num + 1, if args.format == "vrd" { "vrd" } else { "md" })
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
    let output_filename = if args.format == "vrd" {
        format!("{}.vrd", args.output)
    } else {
        format!("{}.md", args.output)
    };
    
    match fs::write(&output_filename, content) {
        Ok(()) => {
            println!("✅ Successfully compressed and wrote to {}", output_filename);
            stats.compressed_size = content.len();
            stats.compressed_lines = content.lines().count();
        }
        Err(e) => println!("❌ Error writing output: {}", e),
    }
}

fn compress_content(content: &str, level: &str, model: &str, ai_mode: bool, no_emojis: bool) -> String {
    let mut compressed = content.to_string();
    
    // Remove emojis if requested (do this early to save processing)
    if no_emojis {
        compressed = remove_emojis(&compressed);
    }
    
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

fn remove_duplicate_content(all_files_content: Vec<(String, String, std::path::PathBuf)>, show_stats: bool) -> Vec<(String, String, std::path::PathBuf)> {
    let mut seen_paragraphs = std::collections::HashSet::new();
    let mut deduplicated = Vec::new();
    let mut duplicates_removed = 0;
    
    for (filename, content, path) in all_files_content {
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
        
        deduplicated.push((filename, unique_paragraphs.join("\n"), path));
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


fn generate_vrd_content(all_files_content: &[(String, String, std::path::PathBuf)], args: &Args, original_stats: &CompressionStats) -> String {
    let mut vrd_files = Vec::new();
    
    // Process each file into VRD format
    for (filename, content, path) in all_files_content {
        let vrd_file = process_file_for_vrd(filename, content, args, path);
        vrd_files.push(vrd_file);
    }
    
    // Build VRD content first to calculate accurate size
    let vrd_content = build_vrd_output(&vrd_files, args);
    
    // Calculate actual compression stats
    let compressed_size = vrd_content.len();
    let _compressed_lines = vrd_content.lines().count();
    
    // Generate metadata with accurate compression stats
    let metadata = VrdMetadata {
        files_count: all_files_content.len(),
        estimated_tokens: compressed_size / 4,
        compression_ratio: if original_stats.original_size > 0 {
            // Positive compression ratio (should be positive when we save space)
            ((original_stats.original_size as f64 - compressed_size as f64) / original_stats.original_size as f64) * 100.0
        } else {
            0.0
        },
        generated: Utc::now(),
    };
    
    // Update the content with correct metadata
    update_vrd_metadata(&vrd_content, &metadata)
}

fn update_vrd_metadata(content: &str, metadata: &VrdMetadata) -> String {
    content.replace(
        "META:{files:0,tokens:0,compressed:0.0%,generated:2025-01-01T00:00:00Z}",
        &format!(
            "META:{{files:{},tokens:{},compressed:{:.1}%,generated:{}}}",
            metadata.files_count,
            metadata.estimated_tokens,
            metadata.compression_ratio,
            metadata.generated.format("%Y-%m-%dT%H:%M:%SZ")
        )
    )
}

fn process_file_for_vrd(filename: &str, content: &str, args: &Args, file_path: &std::path::Path) -> VrdFile {
    // Get real file modification time
    let modified_time = if let Ok(metadata) = file_path.metadata() {
        if let Ok(modified) = metadata.modified() {
            DateTime::<Utc>::from(modified)
        } else {
            Utc::now()
        }
    } else {
        Utc::now()
    };

    let mut vrd_file = VrdFile {
        name: filename.to_string(),
        modified: modified_time, // Use actual file time
        size: content.len(),
        lines: content.lines().count(),
        tags: extract_enhanced_tags_from_content(content),
        headers: extract_headers_for_vrd(content, args.no_emojis),
        content: String::new(),
        code_blocks: Vec::new(),
    };
    
    // Process content through compression pipeline
    let mut processed_content = content.to_string();
    
    if args.no_emojis {
        processed_content = remove_emojis(&processed_content);
    }
    
    vrd_file.code_blocks = extract_and_compress_code_blocks(&processed_content);
    processed_content = apply_vrd_compression(&processed_content, &args.level);
    vrd_file.content = processed_content;
    vrd_file
}

fn apply_vrd_compression(content: &str, level: &str) -> String {
    let mut result = content.to_string();
    
    // Remove code blocks (they're handled separately)
    let re_code_block = regex::Regex::new(r"```[\s\S]*?```").unwrap();
    result = re_code_block.replace_all(&result, "").to_string();
    
    // Remove headers (they're in the H: field) - apply line by line
    result = result
        .lines()
        .filter(|line| !line.trim_start().starts_with('#'))
        .collect::<Vec<_>>()
        .join("\n");
    
    // Apply standard compression
    result = remove_excessive_whitespace(&result);
    result = remove_empty_lines(&result);
    
    // VRD-specific optimizations
    result = apply_arrow_notation(&result);
    result = apply_vrd_abbreviations(&result);
    result = compress_vrd_lists(&result);
    result = compress_vrd_sentences(&result);
    
    match level {
        "high" | "extreme" => {
            result = apply_extreme_vrd_compression(&result);
            result = apply_mathematical_notation(&result);
        }
        _ => {}
    }
    
    result
}

fn compress_vrd_lists(content: &str) -> String {
    let mut result = content.to_string();
    
    // Convert bullet points to more compact notation
    let re_bullets = regex::Regex::new(r"^[*-]\s+(.+)$").unwrap();
    result = re_bullets.replace_all(&result, "•$1").to_string();
    
    // Convert numbered lists to compact notation
    let re_numbered = regex::Regex::new(r"^\d+\.\s+(.+)$").unwrap();
    result = re_numbered.replace_all(&result, "№$1").to_string();
    
    result
}

fn compress_vrd_sentences(content: &str) -> String {
    let mut result = content.to_string();
    
    // Replace common verbose phrases with concise equivalents
    let replacements = [
        (r"in order to", "to"),
        (r"due to the fact that", "because"),
        (r"it is important to note that", "NOTE:"),
        (r"please note that", "NOTE:"),
        (r"as mentioned above", "↑"),
        (r"as shown below", "↓"),
        (r"for example", "EX:"),
        (r"such as", "e.g."),
        (r"and so on", "etc"),
        (r"at this point in time", "now"),
        (r"in the event that", "if"),
        (r"on the other hand", "vs"),
    ];
    
    for (pattern, replacement) in replacements {
        let re = regex::Regex::new(&format!(r"(?i){}", pattern)).unwrap();
        result = re.replace_all(&result, replacement).to_string();
    }
    
    result
}

fn extract_headers_for_vrd(content: &str, no_emojis: bool) -> Vec<String> {
    let mut headers = Vec::new();
    
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('#') {
            // Extract header text, removing markdown symbols
            let mut header_text = trimmed
                .trim_start_matches('#')
                .trim()
                .to_string();
            
            // Apply emoji removal if enabled
            if no_emojis {
                header_text = remove_emojis(&header_text);
            }
            
            headers.push(header_text);
        }
    }
    
    headers
}

fn extract_and_compress_code_blocks(content: &str) -> Vec<String> {
    let re_code_block = regex::Regex::new(r"```(\w+)?\n([\s\S]*?)```").unwrap();
    let mut code_blocks = Vec::new();
    
    for cap in re_code_block.captures_iter(content) {
        let lang = cap.get(1).map_or("", |m| m.as_str());
        let code = cap.get(2).map_or("", |m| m.as_str());
        
        // Compress code using arrow notation
        let compressed_code = compress_code_for_vrd(code, lang);
        code_blocks.push(compressed_code);
    }
    
    code_blocks
}

fn compress_code_for_vrd(code: &str, _lang: &str) -> String {
    // Ultra-aggressive code compression for VRD
    let compressed = code
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            line.trim()
                // Replace common patterns with arrows
                .replace(" => ", "→")
                .replace(" -> ", "→")
                .replace("return ", "→")
                .replace("async function ", "async FN ")
                .replace("function ", "FN ")
                .replace("const ", "")
                .replace("let ", "")
                .replace("var ", "")
                // Remove unnecessary spaces
                .replace("( ", "(")
                .replace(" )", ")")
                .replace("{ ", "{")
                .replace(" }", "}")
        })
        .collect::<Vec<_>>()
        .join("→");
    
    compressed
}

fn apply_arrow_notation(content: &str) -> String {
    let mut result = apply_enhanced_arrow_notation(content);
    
    let basic_patterns = [
        (r" then ", "→"),
        (r" and then ", "→"),
        (r" which ", "→"),
        (r" that ", "→"),
        (r" leads to ", "→"),
        (r" results in ", "→"),
        (r" causes ", "→"),
        (r" triggers ", "→"),
        (r" followed by ", "→"),
    ];
    
    for (pattern, replacement) in basic_patterns {
        let re = regex::Regex::new(pattern).unwrap();
        result = re.replace_all(&result, replacement).to_string();
    }
    
    result
}

fn apply_vrd_abbreviations(content: &str) -> String {
    let abbreviations = [
        ("application", "app"),
        ("configuration", "CFG"),
        ("authentication", "AUTH"),
        ("authorization", "AUTHZ"),
        ("database", "DB"),
        ("function", "FN"),
        ("parameter", "PARAM"),
        ("variable", "var"),
        ("interface", "interface"),
        ("implementation", "IMPL"),
        ("documentation", "DOC"),
        ("example", "EX"),
        ("installation", "INST"),
        ("development", "dev"),
        ("production", "prod"),
        ("environment", "env"),
        ("repository", "repo"),
    ];
    
    let mut result = content.to_string();
    for (full, abbrev) in abbreviations {
        let re = regex::Regex::new(&format!(r"\b{}\b", regex::escape(full))).unwrap();
        result = re.replace_all(&result, abbrev).to_string();
    }
    
    result
}

fn apply_extreme_vrd_compression(content: &str) -> String {
    let mut result = content.to_string();
    
    // Remove articles
    let re_articles = regex::Regex::new(r"\b(a|an|the)\s+").unwrap();
    result = re_articles.replace_all(&result, "").to_string();
    
    // Remove filler words
    let fillers = ["really", "very", "quite", "just", "simply", "basically", "essentially", "actually", "literally"];
    for filler in fillers {
        let re = regex::Regex::new(&format!(r"\b{}\s+", filler)).unwrap();
        result = re.replace_all(&result, "").to_string();
    }
    
    // Remove redundant markdown formatting since it's already structured
    result = result.replace("**", "");
    result = result.replace("*", "");
    
    // Compress common phrases aggressively
    let aggressive_replacements = [
        ("in order to", "to"),
        ("due to the fact that", "because"),
        ("it is important to note that", "NOTE:"),
        ("please note that", "NOTE:"),
        ("as mentioned above", "↑"),
        ("as shown below", "↓"),
        ("for example", "EX:"),
        ("such as", "e.g."),
        ("and so on", "etc"),
        ("at this point in time", "now"),
        ("in the event that", "if"),
        ("on the other hand", "vs"),
        ("Generated:", "Gen:"),
        ("Created:", "Made:"),
        ("Implemented:", "Built:"),
        ("Achievement:", "Win:"),
        ("Accomplished:", "Done:"),
        ("Features", "F:"),
        ("Priority", "P"),
        ("Current", "Now"),
        ("Strategic", "Strategy"),
        ("Technical", "Tech"),
        ("Development", "Dev"),
        ("Implementation", "Impl"),
        ("Optimization", "Opt"),
        ("Specification", "Spec"),
        ("Documentation", "Doc"),
        ("Repository", "Repo"),
        ("Application", "App"),
        ("Configuration", "Config"),
        ("Environment", "Env"),
        ("Performance", "Perf"),
        ("Quality Assurance", "QA"),
        ("User Experience", "UX"),
        ("Breakthrough", "Win"),
        ("represents", "="),
        ("demonstrates", "shows"),
        ("successfully", "✓"),
        ("efficiently", "fast"),
        ("comprehensive", "full"),
        ("innovative", "new"),
        ("revolutionary", "new"),
        ("significant", "big"),
        ("important", "key"),
        ("potential", "could"),
        ("capability", "can"),
        ("functionality", "work"),
        ("opportunity", "chance"),
        ("improvement", "fix"),
        ("enhancement", "boost"),
    ];
    
    for (full, short) in aggressive_replacements {
        let re = regex::Regex::new(&format!(r"(?i)\b{}\b", regex::escape(full))).unwrap();
        result = re.replace_all(&result, short).to_string();
    }
    
    result
}

fn apply_mathematical_notation(content: &str) -> String {
    let mut result = content.to_string();
    
    let math_replacements = [
        (r"\breturn\b", "→"),
        (r"\byield\b", "⟶"),
        (r"\btherefore\b", "∴"),
        (r"\bbecause\b", "∵"),
        (r"\bequals?\b", "="),
        (r"\bnot equal", "≠"),
        (r"\bgreater than or equal", "≥"),
        (r"\bless than or equal", "≤"),
        (r"\bapproximately", "≈"),
        (r"\binfinity", "∞"),
        (r"\bsum of", "Σ"),
        (r"\bfor all", "∀"),
        (r"\bthere exists", "∃"),
        (r"\bmapping to", "↦"),
        (r"\bimplies", "⟹"),
        (r"\bif and only if", "⟺"),
    ];
    
    for (pattern, replacement) in math_replacements {
        let re = regex::Regex::new(&format!(r"(?i){}", pattern)).unwrap();
        result = re.replace_all(&result, replacement).to_string();
    }
    
    result
}

fn apply_enhanced_arrow_notation(content: &str) -> String {
    let patterns = [
        // Process flows
        (r"user submits form", "user→form"),
        (r"server validates data", "server→validate"),
        (r"database stores result", "DB→store"),
        (r"system sends response", "system→response"),
        
        // Causal relationships
        (r"(\w+)\s+triggers\s+(\w+)", "$1→$2"),
        (r"(\w+)\s+causes\s+(\w+)", "$1→$2"),
        (r"(\w+)\s+leads to\s+(\w+)", "$1→$2"),
        (r"(\w+)\s+results in\s+(\w+)", "$1→$2"),
        
        // Temporal sequences
        (r"after\s+(\w+),?\s+(\w+)", "$1→$2"),
        (r"once\s+(\w+),?\s+(\w+)", "$1→$2"),
        (r"when\s+(\w+),?\s+(\w+)", "$1→$2"),
        (r"then\s+(\w+)", "→$1"),
        
        // Data flows
        (r"(\w+)\s+passes\s+(\w+)\s+to\s+(\w+)", "$1→$2→$3"),
        (r"(\w+)\s+sends\s+(\w+)", "$1→$2"),
        (r"(\w+)\s+receives\s+(\w+)", "$2→$1"),
    ];
    
    let mut result = content.to_string();
    for (pattern, replacement) in patterns {
        let re = regex::Regex::new(&format!(r"(?i){}", pattern)).unwrap();
        result = re.replace_all(&result, replacement).to_string();
    }
    
    result
}

fn update_vrd_chunk_header(content: String, chunk_num: usize, total_chunks: usize, args: &Args) -> String {
    // Replace the CHUNKS field in the VRD header
    let updated = content.replace(
        "CHUNKS:1/1",
        &format!("CHUNKS:{}/{}", chunk_num, total_chunks)
    );
    
    // Add NEXT reference if not the last chunk
    if chunk_num < total_chunks {
        let next_filename = if args.output.contains("chunk") {
            format!("{}_{}.vrd", args.output, chunk_num + 1)
        } else {
            format!("{}_chunk_{}.vrd", args.output, chunk_num + 1)
        };
        
        // Insert NEXT after CHUNKS
        updated.replace(
            &format!("CHUNKS:{}/{}", chunk_num, total_chunks),
            &format!("CHUNKS:{}/{}|NEXT:{}", chunk_num, total_chunks, next_filename)
        )
    } else {
        updated
    }
}

fn build_vrd_output(vrd_files: &[VrdFile], args: &Args) -> String {
    let mut output = String::new();
    
    // Header (metadata will be updated later)
    output.push_str(&format!(
        "VRD1.0|TARGET:{}|MODE:{}|CHUNKS:1/1\n",
        args.model.to_uppercase(),
        args.level.to_uppercase()
    ));
    
    // Placeholder metadata (will be updated)
    output.push_str("META:{files:0,tokens:0,compressed:0.0%,generated:2025-01-01T00:00:00Z}\n");
    
    // Dictionary
    output.push_str("DICT:{");
    let dict_entries = [
        ("FN", "function"),
        ("PARAM", "parameter"),
        ("AUTH", "authentication"),
        ("DB", "database"),
        ("API", "application programming interface"),
        ("CFG", "configuration"),
        ("DOC", "documentation"),
        ("IMPL", "implementation"),
        ("ENV", "environment"),
        ("REPO", "repository"),
    ];
    
    for (i, (abbrev, full)) in dict_entries.iter().enumerate() {
        if i > 0 { output.push(','); }
        output.push_str(&format!("{}={}", abbrev, full));
    }
    output.push_str("}\n---\n");
    
    // File contents
    for (i, file) in vrd_files.iter().enumerate() {
        if i > 0 { output.push('\n'); }
        
        // File header
        output.push_str(&format!(
            "F:{}|D:{}|S:{}|L:{}|T:{}\n",
            file.name,
            file.modified.format("%Y-%m-%dT%H:%M:%SZ"),
            file.size,
            file.lines,
            file.tags.join(",")
        ));
        
        // Headers
        if !file.headers.is_empty() {
            output.push_str(&format!("H:{}\n", file.headers.join(",")));
        }
        
        // Content
        if !file.content.trim().is_empty() {
            output.push_str(&format!("C:{}\n", file.content.trim()));
        }
        
        // Code blocks
        for code_block in &file.code_blocks {
            output.push_str(&format!("X:{}\n", code_block));
        }
        
        output.push_str("|\n");
    }
    
    output
}

fn extract_enhanced_tags_from_content(content: &str) -> Vec<String> {
    let mut tags = std::collections::HashSet::new();
    let content_lower = content.to_lowercase();
    
    // Technical framework tags
    let frameworks = [
        ("react", "react"), ("vue", "vue"), ("angular", "angular"),
        ("express", "express"), ("fastapi", "fastapi"), ("django", "django"),
        ("flask", "flask"), ("spring", "spring"), ("rails", "rails"),
        ("nextjs", "nextjs"), // Added to make 10 elements
    ];
    
    // Language tags
    let languages = [
        ("javascript", "js"), ("typescript", "ts"), ("python", "python"),
        ("rust", "rust"), ("go", "go"), ("java", "java"), ("c++", "cpp"),
        ("c#", "csharp"), ("php", "php"), ("ruby", "ruby"),
    ];
    
    // Technology tags
    let technologies = [
        ("docker", "docker"), ("kubernetes", "k8s"), ("aws", "aws"),
        ("azure", "azure"), ("gcp", "gcp"), ("redis", "redis"),
        ("postgresql", "postgres"), ("mysql", "mysql"), ("mongodb", "mongo"),
        ("elasticsearch", "elastic"),
    ];
    
    // Concept tags
    let concepts = [
        ("authentication", "auth"), ("authorization", "authz"),
        ("security", "security"), ("testing", "testing"), ("deployment", "deploy"),
        ("monitoring", "monitoring"), ("logging", "logging"), ("caching", "cache"),
        ("scaling", "scale"), ("performance", "perf"),
    ];
    
    let all_patterns = [frameworks, languages, technologies, concepts].concat();
    
    for (pattern, tag) in all_patterns {
        if content_lower.contains(pattern) {
            tags.insert(tag.to_string());
        }
    }
    
    // Convert to sorted vector, limit to 5 most relevant
    let mut tag_vec: Vec<String> = tags.into_iter().collect();
    tag_vec.sort();
    tag_vec.truncate(5);
    tag_vec
}