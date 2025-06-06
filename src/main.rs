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
    
    /// Output file path
    #[arg(short, long, default_value = "compressed.md")]
    output: String,
    
    /// Compression level (low, medium, high)
    #[arg(short, long, default_value = "medium")]
    level: String,
    
    /// Show detailed statistics about compression
    #[arg(short, long)]
    stats: bool,
}

fn compress_content(content: &str, level: &str) -> String {
    let mut compressed = content.to_string();
    
    // Always apply aggressive whitespace removal
    compressed = remove_excessive_whitespace(&compressed);
    compressed = remove_empty_lines(&compressed);
    
    // Basic compression
    compressed = compress_headers_aggressively(&compressed);
    compressed = compress_formatting(&compressed);
    
    // Medium and high
    if level == "medium" || level == "high" {
        compressed = compress_code_blocks(&compressed);
        compressed = compress_lists_aggressively(&compressed);
        compressed = remove_fluff_words(&compressed);
    }
    
    // High only - very aggressive
    if level == "high" {
        compressed = compress_sentences(&compressed);
        compressed = remove_redundant_phrases(&compressed);
    }
    
    compressed
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
            // Only check substantial paragraphs (more than 30 chars)
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
    let re_multiple_newlines = Regex::new(r"\n{2,}").unwrap(); // 2+ newlines -> 1
    let re_multiple_spaces = Regex::new(r" {2,}").unwrap();    // 2+ spaces -> 1
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
    result = re_h1.replace_all(&result, "H1:$1").to_string(); // No space after colon
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

fn compress_code_blocks(content: &str) -> String {
    let re_code_block = Regex::new(r"```(\w+)?\n([\s\S]*?)```").unwrap();
    
    re_code_block.replace_all(content, |caps: &regex::Captures| {
        let lang = caps.get(1).map_or("", |m| m.as_str());
        let code = caps.get(2).map_or("", |m| m.as_str());
        
        // Keep code but compress whitespace
        let compressed_code = code.lines()
            .filter(|line| !line.trim().is_empty())
            .collect::<Vec<_>>()
            .join("\n");
            
        if lang.is_empty() {
            format!("CODE:{}", compressed_code.replace('\n', "|"))
        } else {
            format!("CODE({}):{}", lang, compressed_code.replace('\n', "|"))
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

fn main() {
    let args = Args::parse();
    
    println!("🌱 verdant");
    println!("  Compressing markdown for AI consumption");
    println!("  by greenscript");
    println!("  Version 1.0");
    println!();
    println!("Input: {}", args.input);
    println!("Output: {}", args.output);
    println!("Compression: {}", args.level);
    println!();
    
    // Find all .md files
    let md_files: Vec<_> = WalkDir::new(&args.input)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "md"))
        .collect();
    
    println!("Found {} markdown files:", md_files.len());
    
    let mut all_files_content = Vec::new();
    let mut total_original_size = 0;
    let mut total_original_lines = 0;
    
    // First pass: read all files
    for file in md_files {
        println!("  📄 {}", file.path().display());
        
        match fs::read_to_string(file.path()) {
            Ok(content) => {
                total_original_size += content.len();
                total_original_lines += content.lines().count();
                
                if args.stats {
                    println!("    Lines: {}, Chars: {}", content.lines().count(), content.len());
                }
                
                let filename = file.path().file_name().unwrap().to_str().unwrap().to_string();
                all_files_content.push((filename, content));
            }
            Err(e) => println!("Error reading {}: {}", file.path().display(), e),
        }
    }
    
    // Remove duplicate content across files
    if args.level == "medium" || args.level == "high" {
        println!("\n🔄 Removing duplicate content across files...");
        all_files_content = remove_duplicate_content(all_files_content, args.stats);
    }
    
    // Second pass: compress and combine
    let mut combined_content = String::new();
    for (filename, content) in all_files_content {
        combined_content.push_str(&format!("F:{}\n", filename));
        let compressed = compress_content(&content, &args.level);
        combined_content.push_str(&compressed);
        combined_content.push_str("\n|\n");
    }
    
    // Write compressed output
    match fs::write(&args.output, &combined_content) {
        Ok(()) => {
            let compressed_size = combined_content.len();
            let compressed_lines = combined_content.lines().count();
            let compression_ratio = (1.0 - (compressed_size as f64 / total_original_size as f64)) * 100.0;
            let line_compression_ratio = (1.0 - (compressed_lines as f64 / total_original_lines as f64)) * 100.0;
            
            println!("\n✅ Successfully compressed and wrote to {}", args.output);
            
            if args.stats {
                println!("📊 DETAILED STATISTICS:");
                println!("   Original:   {} lines, {} chars", total_original_lines, total_original_size);
                println!("   Compressed: {} lines, {} chars", compressed_lines, compressed_size);
                println!("   Line compression: {:.1}%", line_compression_ratio);
                println!("   Char compression: {:.1}%", compression_ratio);
                
                // Token estimation (rough: ~4 chars per token)
                let original_tokens = total_original_size / 4;
                let compressed_tokens = compressed_size / 4;
                println!("   Est. tokens: {} → {} (saved ~{})", original_tokens, compressed_tokens, original_tokens - compressed_tokens);
            } else {
                println!("📊 Original: {} chars, Compressed: {} chars", total_original_size, compressed_size);
                println!("📈 Compression ratio: {:.1}%", compression_ratio);
                println!("📄 Lines: {} → {} ({:.1}% reduction)", total_original_lines, compressed_lines, line_compression_ratio);
            }
        }
        Err(e) => println!("❌ Error writing output: {}", e),
    }
}