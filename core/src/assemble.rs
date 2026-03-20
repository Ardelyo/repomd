use crate::discover::ScoredFile;
use crate::compress::compress_content;
use crate::{Config, FileDetail};
use tiktoken_rs::cl100k_base;

/// Original compose_markdown (backward compat for API)
pub fn compose_markdown(entries: Vec<ScoredFile>, config: &Config) -> anyhow::Result<String> {
    let (md, _, _) = compose_markdown_with_details(entries, config)?;
    Ok(md)
}

/// Enhanced version that returns per-file details and total tokens
pub fn compose_markdown_with_details(
    entries: Vec<ScoredFile>,
    config: &Config,
) -> anyhow::Result<(String, Vec<FileDetail>, usize)> {
    let bpe = cl100k_base()?;
    let mut output = String::new();
    
    output.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    output.push_str("<context>\n");
    output.push_str("  <repository_details>\n");
    output.push_str("    <description>The following is an optimized codebase extraction. Files are compressed according to context priority.</description>\n");
    output.push_str(&format!("    <total_files_scanned>{}</total_files_scanned>\n", entries.len()));
    output.push_str("  </repository_details>\n\n");

    output.push_str("  <repository_structure>\n");
    for entry in &entries {
        output.push_str(&format!("    - {}\n", entry.path.display()));
    }
    output.push_str("  </repository_structure>\n\n");

    output.push_str("  <code_base>\n");

    let mut total_tokens = 0;
    let limit = config.target_tokens.unwrap_or(usize::MAX);
    let preset_level = config.preset.unwrap_or(1);

    let mut file_details: Vec<FileDetail> = Vec::new();

    for mut entry in entries {
        if entry.compression_level == 0 {
            entry.compression_level = preset_level;
        }

        let extension = entry.path.extension().and_then(|s| s.to_str());
        let compressed = compress_content(&entry.content, &entry.role, entry.compression_level, extension);

        let file_content = format!(
            "    <file path=\"{}\" cps=\"{:.2}\" level=\"{}\">\n```{}\n{}\n```\n    </file>\n\n",
            entry.path.display(),
            entry.cps,
            entry.compression_level,
            extension.unwrap_or("text"),
            compressed
        );
        let tokens = bpe.encode_with_special_tokens(&file_content).len();

        if total_tokens + tokens > limit {
            // Record remaining files as omitted
            file_details.push(FileDetail {
                path: entry.path.display().to_string(),
                role: format!("{:?}", entry.role),
                cps: entry.cps,
                compression_level: entry.compression_level,
                tokens,
                included: false,
            });
            output.push_str(&format!(
                "    <!-- Limit Reached: {} tokens. Remaining files omitted. -->\n",
                limit
            ));
            // Don't break yet — mark remaining as omitted for stats
            continue;
        }

        output.push_str(&file_content);
        total_tokens += tokens;

        file_details.push(FileDetail {
            path: entry.path.display().to_string(),
            role: format!("{:?}", entry.role),
            cps: entry.cps,
            compression_level: entry.compression_level,
            tokens,
            included: true,
        });
    }

    output.push_str("  </code_base>\n");
    output.push_str("</context>\n");
    
    // Add token hint at the end outside the XML
    output.push_str(&format!(
        "\n<!-- Total Tokens Evaluated: {} -->\n",
        total_tokens
    ));

    Ok((output, file_details, total_tokens))
}
