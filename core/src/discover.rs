use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq)]
pub enum FileRole {
    Source,
    Config,
    Test,
    Doc,
    Generated,
    Asset,
}

#[derive(Debug, Clone)]
pub struct ScoredFile {
    pub path: PathBuf,
    pub content: String,
    pub role: FileRole,
    pub cps: f32,
    pub compression_level: u8,
}

pub fn classify_role(path: &std::path::Path) -> FileRole {
    let name = path.file_name().unwrap_or_default().to_string_lossy();
    let name_lower = name.to_lowercase();
    
    if name_lower.contains("test") || name_lower.contains("spec") {
        return FileRole::Test;
    }
    
    match path.extension().and_then(|s| s.to_str()) {
        Some(ext) => match ext.to_lowercase().as_str() {
            "rs" | "ts" | "js" | "py" | "go" | "java" | "c" | "cpp" | "h" | "txt" => FileRole::Source,
            "toml" | "json" | "yaml" | "yml" | "env" | "conf" => FileRole::Config,
            "md" | "mdx" | "rst" => FileRole::Doc,
            "png" | "jpg" | "jpeg" | "gif" | "mp4" | "exe" | "dll" | "so" => FileRole::Asset,
            "lock" | "pb.go" | "min.js" => FileRole::Generated,
            _ => FileRole::Source,
        },
        None => {
            if name_lower == "dockerfile" || name_lower == "makefile" {
                FileRole::Config
            } else {
                FileRole::Source
            }
        }
    }
}

pub fn calculate_cps(role: &FileRole, path: &std::path::Path, content: &str) -> f32 {
    let mut score = 1.0;
    
    match role {
        FileRole::Source => score += 0.5,
        FileRole::Config => score += 0.2,
        FileRole::Doc => score += 0.1,
        _ => score = 0.0,
    }
    
    // Centrality heuristic: paths closer to root are often more central
    let components = path.components().count();
    if components < 3 { score += 0.3; }
    
    // Complexity heuristic: length of file content
    if content.len() > 1000 { score += 0.2; }
    if content.len() > 10000 { score += 0.4; }
    
    score
}
