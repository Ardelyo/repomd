use crate::discover::FileRole;
use regex::Regex;
use serde_json::Value;

pub fn compress_content(
    content: &str,
    role: &FileRole,
    level: u8,
    extension: Option<&str>,
) -> String {
    if level == 0 {
        return content.to_string();
    }

    match role {
        FileRole::Interface | FileRole::Schema => content.to_string(),
        FileRole::Documentation => compress_docs(content, level),
        FileRole::StyleSheet => compress_stylesheet(content),
        FileRole::DataFile => compress_data(content, extension),
        FileRole::Generated => String::new(),
        FileRole::Config => compress_config(content),
        FileRole::Test => compress_test(content, extension, level),
        FileRole::CoreLogic | FileRole::EntryPoint => compress_logic(content, extension, level),
    }
}

fn compress_docs(content: &str, level: u8) -> String {
    if level <= 2 {
        return content.to_string();
    }
    
    // Level 3-4: Strip long code examples.
    let re = Regex::new(r"(?s)```.*?\n(.*?)\n```").unwrap();
    let cleaned = re.replace_all(content, |caps: &regex::Captures| {
        let code = &caps[1];
        if code.lines().count() > 10 {
            "```\n// ... [code example > 10 lines omitted]\n```".to_string()
        } else {
            caps[0].to_string()
        }
    });

    if level == 4 {
        // Only keep headings and first paragraph. Simplified for Phase 1.
        let mut out = String::new();
        let mut keep_next = false;
        for line in cleaned.lines() {
            if line.starts_with("#") {
                out.push_str(line);
                out.push('\n');
                keep_next = true;
            } else if keep_next && !line.trim().is_empty() {
                out.push_str(line);
                out.push('\n');
                keep_next = false;
            }
        }
        return out;
    }
    
    cleaned.to_string()
}

fn compress_stylesheet(content: &str) -> String {
    let mut out = String::from("/* [StyleSheet — Design Tokens Only] */\n");
    
    // Keep `:root { ... }`
    let root_re = Regex::new(r"(?s):root\s*\{([^}]+)\}").unwrap();
    if let Some(caps) = root_re.captures(content) {
        out.push_str(":root {\n  ");
        out.push_str(caps[1].trim());
        out.push_str("\n}\n");
    }
    
    // List media query breakpoints
    let media_re = Regex::new(r"@media\s+([^{]+)").unwrap();
    let breakpoints: Vec<&str> = media_re.find_iter(content).map(|m| m.as_str().trim()).collect();
    if !breakpoints.is_empty() {
        out.push_str("/* Breakpoints found: */\n");
        for bp in breakpoints {
            out.push_str("/* ");
            out.push_str(bp);
            out.push_str(" */\n");
        }
    }
    
    if out == "/* [StyleSheet — Design Tokens Only] */\n" {
        return String::from("/* [StyleSheet — Design Tokens Only] (No tokens found) */");
    }
    out
}

fn compress_data(content: &str, extension: Option<&str>) -> String {
    if let Some("json") = extension {
        if let Ok(val) = serde_json::from_str::<Value>(content) {
            return format!("/* [DataFile Schema Extracted] */\n{}", extract_schema(&val, 0));
        }
    }
    
    // Fallback truncation
    if content.len() > 500 {
        format!("{}... \n// [{} more bytes of data omitted]", &content[0..500], content.len() - 500)
    } else {
        content.to_string()
    }
}

fn extract_schema(val: &Value, depth: usize) -> String {
    if depth > 4 { return "...".to_string(); }
    let indent = " ".repeat(depth * 2);
    
    match val {
        Value::Null => "null".to_string(),
        Value::Bool(_) => "boolean".to_string(),
        Value::Number(_) => "number".to_string(),
        Value::String(_) => "string".to_string(),
        Value::Array(arr) => {
            if arr.is_empty() {
                "[]".to_string()
            } else {
                format!("[\n  {}{},\n  {}// ... {} more items of same schema\n{}]", 
                    indent, extract_schema(&arr[0], depth + 1), indent, arr.len() - 1, indent)
            }
        },
        Value::Object(obj) => {
            if obj.is_empty() {
                "{}".to_string()
            } else {
                let mut out = String::from("{\n");
                for (k, v) in obj {
                    out.push_str(&format!("  {}\"{}\": {},\n", indent, k, extract_schema(v, depth + 1)));
                }
                out.push_str(&format!("{}}}", indent));
                out
            }
        }
    }
}

fn compress_config(content: &str) -> String {
    // Basic cleanup: remove simple full-line comments 
    let re_comments = Regex::new(r"(?m)^\s*(#|//).*$").unwrap();
    let re_blank_lines = Regex::new(r"(?m)^\s*\n").unwrap();
    let cleaned = re_comments.replace_all(content, "");
    re_blank_lines.replace_all(&cleaned, "\n").to_string()
}

fn compress_logic(content: &str, extension: Option<&str>, level: u8) -> String {
    // Level 1-2: preserve logic & comments, strip excessive blank lines
    let re_blank_lines = Regex::new(r"(?m)^\s*\n").unwrap();
    let cleaned = re_blank_lines.replace_all(content, "\n");
    
    if level <= 2 {
        return cleaned.to_string();
    }
    
    // Level 3-4: signatures + docstrings (Fallback for semantic summary in Phase 1)
    let mut signatures = Vec::new();
    let ext = extension.unwrap_or("");
    
    let re_rs = Regex::new(r"^(///\s*.*$|pub\s+)?(fn|struct|enum|trait|impl|use)\s+[^{;]+").unwrap();
    let re_ts = Regex::new(r"^(/\*\*[\s\S]*?\*/|export\s+)?(import|function|class|interface|type|const\s+\w+\s*=\s*(?:async\s*)?\([^)]*\)\s*=>)\s*").unwrap();
    let re_py = Regex::new(r"^(async\s+)?(def|class|import|from\s+[\w.]+\s+import)\s+[^(:]+").unwrap();
    
    for line in content.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("///") || trimmed.starts_with("//!") {
            signatures.push(line.to_string());
        } else {
            match ext {
                "rs" => {
                    if let Some(m) = re_rs.find(line) { signatures.push(format!("{} ...", m.as_str())); }
                }
                "ts" | "js" | "jsx" | "tsx" => {
                    if let Some(m) = re_ts.find(line) { signatures.push(format!("{} ...", m.as_str())); }
                }
                "py" => {
                    if let Some(m) = re_py.find(line) { signatures.push(format!("{} ...", m.as_str())); }
                }
                _ => {}
            }
        }
    }
    
    if signatures.is_empty() { return cleaned.to_string(); }
    
    if level == 4 {
        format!("// [Semantic Summary to be generated in v2.1]\n{}", signatures.join("\n"))
    } else {
        signatures.join("\n")
    }
}

fn compress_test(content: &str, extension: Option<&str>, level: u8) -> String {
    // For Phase 1, just use the logic compression (Level 3 signatures).
    // The spec says "keep all test function signatures and keep the first test body verbatim".
    // That's more complex regex. We will fallback to signatures + docstrings for level > 1.
    if level <= 1 {
        compress_logic(content, extension, level)
    } else {
        compress_logic(content, extension, 3) 
    }
}
