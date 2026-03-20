use std::path::{Path, PathBuf};
use std::fs;
use std::time::SystemTime;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileRole {
    CoreLogic,
    EntryPoint,
    Interface,
    Schema,
    Config,
    Test,
    Documentation,
    DataFile,
    StyleSheet,
    Generated,
}

impl FileRole {
    pub fn rd_weight(&self) -> f32 {
        match self {
            FileRole::CoreLogic => 1.00,
            FileRole::EntryPoint => 0.90,
            FileRole::Interface => 0.85,
            FileRole::Schema => 0.80,
            FileRole::Config => 0.65,
            FileRole::Test => 0.50,
            FileRole::Documentation => 0.75,
            FileRole::DataFile => 0.30,
            FileRole::StyleSheet => 0.10,
            FileRole::Generated => 0.00,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ScoredFile {
    pub path: PathBuf,
    pub content: String,
    pub role: FileRole,
    pub cps: f32,
    pub compression_level: u8,
}

pub fn classify_role(path: &std::path::Path, content: &str) -> FileRole {
    let path_str = path.to_string_lossy().to_lowercase();
    let name = path.file_name().unwrap_or_default().to_string_lossy();
    let name_lower = name.to_lowercase();
    
    // 1. Path pattern
    if path_str.contains("/dist/") || path_str.contains("/node_modules/") 
        || path_str.contains("/.next/") || path_str.contains("/target/") 
        || path_str.contains("/build/") || path_str.contains("/.git/") 
        || path_str.contains("/__pycache__/") {
        return FileRole::Generated;
    }

    // 2. Filename exact match & Content analysis for Build Artifacts
    if name_lower == "package-lock.json" || name_lower == "cargo.lock" 
        || name_lower == "yarn.lock" || name_lower.ends_with(".lock") 
        || name_lower.ends_with(".resolved") || name_lower == "check.json"
        || (name_lower.starts_with("cargo-") && name_lower.ends_with(".json")) {
        return FileRole::Generated;
    }
    
    if name_lower.ends_with(".json") && (content.contains(r#""reason":"compiler-artifact""#) || content.contains(r#""reason": "compiler-artifact""#)) {
        return FileRole::Generated;
    }

    if name_lower.contains("test") || name_lower.contains("spec") || path_str.contains("/tests/") || path_str.contains("/spec/") {
        return FileRole::Test;
    }

    if name_lower == "main.rs" || name_lower == "lib.rs" || name_lower == "mod.rs" 
        || name_lower == "index.ts" || name_lower == "index.js" || name_lower == "app.py" 
        || name_lower == "app.tsx" || name_lower == "page.tsx" || name_lower == "layout.tsx" {
        return FileRole::EntryPoint;
    }
    
    match path.extension().and_then(|s| s.to_str()) {
        Some(ext) => match ext.to_lowercase().as_str() {
            "css" | "scss" | "less" | "styl" => FileRole::StyleSheet,
            "json" | "csv" | "sql" => FileRole::DataFile,
            "md" | "mdx" | "rst" | "txt" | "html" | "htm" => FileRole::Documentation,
            "toml" | "yaml" | "yml" | "env" | "conf" | "properties" | "ini" => FileRole::Config,
            "d.ts" | "graphql" | "proto" | "prisma" => FileRole::Schema,
            "rs" | "ts" | "js" | "py" | "go" | "java" | "c" | "cpp" | "h" | "tsx" | "jsx" => {
                if name_lower.contains("types") || name_lower.contains("interface") {
                    FileRole::Interface
                } else {
                    FileRole::CoreLogic
                }
            },
            "png" | "jpg" | "jpeg" | "gif" | "mp4" | "exe" | "dll" | "so" | "woff" | "woff2" | "pdf" => FileRole::Generated, // Dropped
            "pb.go" | "min.js" => FileRole::Generated,
            _ => FileRole::CoreLogic,
        },
        None => {
            if name_lower == "dockerfile" || name_lower == "makefile" {
                FileRole::Config
            } else {
                FileRole::CoreLogic
            }
        }
    }
}

pub fn calculate_cps(role: &FileRole, path: &std::path::Path, content: &str) -> f32 {
    let rd_weight = role.rd_weight();
    
    // Centrality heuristic for Phase 1
    // Entry points score 0.9. Others scale by inverse depth.
    let components = path.components().count();
    let centrality = if *role == FileRole::EntryPoint {
        0.9
    } else {
        match components {
            1..=2 => 0.6,
            3..=4 => 0.4,
            _ => 0.2, // Leaf files or deep files
        }
    };
    
    // LogicDensity heuristic for Phase 1
    // decision-making tokens: if, match, switch, ?, function, fn, class, inline intent comments
    let mut decision_markers = 0;
    let lines: Vec<&str> = content.lines().collect();
    let total_lines = lines.len();
    
    if total_lines > 0 {
        for line in &lines {
            let l = line.trim();
            if l.starts_with("if ") || l.starts_with("match ") || l.starts_with("switch ")
                || l.starts_with("fn ") || l.starts_with("function ") || l.starts_with("class ")
                || l.starts_with("pub fn ") || l.starts_with("struct ") || l.starts_with("impl ")
                || l.contains("?") || l.contains("// why") || l.contains("// note") 
                || l.contains("// FIXME") || l.contains("// TODO") {
                decision_markers += 1;
            }
        }
    }
    
    let raw_density = if total_lines > 0 {
        (decision_markers as f32) / (total_lines as f32)
    } else {
        0.0
    };
    // Normalize density to 0.0-1.0 roughly. Assume 20% markers is 1.0
    let logic_density = (raw_density * 5.0).min(1.0);
    
    // Freshness heuristic for Phase 1
    // Modified within 14 days = 1.0, 90 days = 0.7, otherwise 0.4
    let mut freshness = 0.7; // default
    if let Ok(metadata) = fs::metadata(path) {
        if let Ok(modified) = metadata.modified() {
            if let Ok(duration) = SystemTime::now().duration_since(modified) {
                let days = duration.as_secs() / (60 * 60 * 24);
                if days <= 14 {
                    freshness = 1.0;
                } else if days <= 90 {
                    freshness = 0.7;
                } else {
                    freshness = 0.4;
                }
            }
        }
    }
    
    // CPS_v2 Formula
    (rd_weight * 0.40) + (centrality * 0.30) + (logic_density * 0.20) + (freshness * 0.10)
}
