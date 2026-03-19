use crate::discover::FileRole;
use regex::Regex;

pub enum CompressionLevel {
    Verbatim = 0,
    Clean = 1,
    Structural = 2,
    Semantic = 3,
    Ultra = 4,
}

pub fn compress_content(
    content: &str,
    role: &FileRole,
    level: u8,
    extension: Option<&str>,
) -> String {
    if level == 0 || *role == FileRole::Config {
        return content.to_string();
    }

    match level {
        1 => level_1_clean(content),
        2 => level_2_structural(content, extension),
        3 | 4 => level_3_semantic(content, role),
        _ => content.to_string(),
    }
}

fn level_1_clean(content: &str) -> String {
    // Basic formatting cleanup: collapse multiple blank lines into one
    let re_blank_lines = Regex::new(r"(?m)^\s*\n").unwrap();
    let cleaned = re_blank_lines.replace_all(content, "\n");
    // Strip simple single-line comments for common languages (rudimentary implementation)
    let re_comments = Regex::new(r"(?m)^\s*(//|#).*$").unwrap();
    re_comments.replace_all(&cleaned, "").to_string()
}

fn level_2_structural(content: &str, extension: Option<&str>) -> String {
    // In a full implementation, this uses Tree-sitter AST to extract signatures.
    // E.g., for Rust: query `(function_item name: (identifier) parameters: (parameters)) @func`
    // Since tree-sitter rust compilation requires C++ tools, we implement a robust 
    // regex-based fallback heuristic for Level 2 signature extraction.
    
    let mut signatures = Vec::new();
    let ext = extension.unwrap_or("");
    
    match ext {
        "rs" => {
            let re = Regex::new(r"^(pub\s+)?(fn|struct|enum|trait)\s+[^{;]+").unwrap();
            for line in content.lines() {
                if let Some(m) = re.find(line) {
                    signatures.push(format!("{} ...", m.as_str()));
                }
            }
        }
        "ts" | "js" | "jsx" | "tsx" => {
            let re = Regex::new(r"^(export\s+)?(function|class|interface|type|const\s+\w+\s*=\s*(?:async\s*)?\([^)]*\)\s*=>)\s*").unwrap();
            for line in content.lines() {
                if let Some(m) = re.find(line) {
                    signatures.push(format!("{} ...", m.as_str()));
                }
            }
        }
        "py" => {
            let re = Regex::new(r"^(async\s+)?(def|class)\s+[^(:]+").unwrap();
            for line in content.lines() {
                if let Some(m) = re.find(line) {
                    signatures.push(format!("{} ...", m.as_str()));
                }
            }
        }
        _ => {
            // Fallback for unsupported languages in Level 2: just do Level 1
            return level_1_clean(content);
        }
    }
    
    if signatures.is_empty() {
        return level_1_clean(content);
    }
    
    signatures.join("\n")
}

fn level_3_semantic(content: &str, role: &FileRole) -> String {
    format!("[Summarized File ({:?})] {} bytes removed to save tokens.", role, content.len())
}
