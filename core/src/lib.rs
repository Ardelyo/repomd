pub mod ingest;
pub mod assemble;
pub mod discover;
pub mod compress;

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub source: Option<String>,
    pub target_tokens: Option<usize>,
    pub preset: Option<u8>, // 1: light, 2: medium, 3: aggressive, 4: ultra
    pub output_path: Option<PathBuf>,
    pub include_tests: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            source: Some(".".to_string()),
            target_tokens: Some(50_000),
            preset: Some(1),
            output_path: Some(PathBuf::from("repo.md")),
            include_tests: false,
        }
    }
}

/// Rich result returned by generate_with_stats
#[derive(Debug, Clone)]
pub struct GenerateResult {
    pub markdown: String,
    pub stats: GenerationStats,
}

#[derive(Debug, Clone)]
pub struct GenerationStats {
    pub files_scanned: usize,
    pub files_included: usize,
    pub files_omitted: usize,
    pub total_tokens: usize,
    pub token_budget: usize,
    pub preset_name: String,
    pub preset_level: u8,
    pub processing_time_ms: u128,
    pub output_bytes: usize,
    pub absolute_output_path: Option<String>,
    /// Per-file info for verbose/inspect mode
    pub file_details: Vec<FileDetail>,
}

#[derive(Debug, Clone)]
pub struct FileDetail {
    pub path: String,
    pub role: String,
    pub cps: f32,
    pub compression_level: u8,
    pub tokens: usize,
    pub included: bool,
}

pub struct SourceContext {
    pub search_path: String,
    pub _temp_dir: Option<tempfile::TempDir>,
}

pub fn resolve_source(source: &str) -> anyhow::Result<SourceContext> {
    let is_url = source.starts_with("http://") || source.starts_with("https://");

    let temp_dir: Option<tempfile::TempDir> = if is_url {
        let dir = tempfile::tempdir()?;
        let status = std::process::Command::new("git")
            .arg("clone")
            .arg(source)
            .arg(dir.path())
            .arg("--depth")
            .arg("1")
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()?;
            
        if !status.success() {
            anyhow::bail!("Failed to clone repository: {}", source);
        }
        Some(dir)
    } else {
        None
    };

    let search_path = if let Some(dir) = &temp_dir {
        dir.path().to_string_lossy().to_string()
    } else {
        source.to_string()
    };
    
    Ok(SourceContext { search_path, _temp_dir: temp_dir })
}

pub fn preset_name(level: u8) -> &'static str {
    match level {
        1 => "light",
        2 => "medium",
        3 => "aggressive",
        4 => "ultra",
        _ => "custom",
    }
}

/// Original simple generate (kept for API backward compat)
pub fn generate(config: Config) -> anyhow::Result<String> {
    let result = generate_with_stats(config)?;
    Ok(result.markdown)
}

/// Enhanced generate that returns rich metadata
pub fn generate_with_stats(config: Config) -> anyhow::Result<GenerateResult> {
    let start = std::time::Instant::now();

    let source = config.source.clone().unwrap_or_else(|| ".".to_string());
    let source_ctx = resolve_source(&source)?;

    let files = ingest::walk_directory(&source_ctx.search_path)?;
    let files_scanned = files.len();
    let preset_level = config.preset.unwrap_or(1);
    let token_budget = config.target_tokens.unwrap_or(50_000);

    let (markdown, file_details, total_tokens) = assemble::compose_markdown_with_details(files, &config)?;

    let files_included = file_details.iter().filter(|f| f.included).count();
    let files_omitted = file_details.iter().filter(|f| !f.included).count();
    let elapsed = start.elapsed().as_millis();

    let markdown_len = markdown.len();

    let absolute_output_path = config.output_path.as_ref().map(|p| {
        std::env::current_dir()
            .ok()
            .map(|cwd| cwd.join(p))
            .unwrap_or_else(|| p.clone())
            .to_string_lossy()
            .to_string()
    });

    Ok(GenerateResult {
        markdown,
        stats: GenerationStats {
            files_scanned,
            files_included,
            files_omitted,
            total_tokens,
            token_budget,
            preset_name: preset_name(preset_level).to_string(),
            preset_level,
            processing_time_ms: elapsed,
            output_bytes: markdown_len,
            absolute_output_path,
            file_details,
        },
    })
}
