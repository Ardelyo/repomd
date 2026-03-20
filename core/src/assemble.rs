use crate::discover::{ScoredFile, FileRole};
use crate::compress::compress_content;
use crate::{Config, FileDetail};
use tiktoken_rs::cl100k_base;

pub fn compose_markdown(entries: Vec<ScoredFile>, config: &Config) -> anyhow::Result<String> {
    let (md, _, _) = compose_markdown_with_details(entries, config)?;
    Ok(md)
}

pub fn compose_markdown_with_details(
    mut entries: Vec<ScoredFile>,
    config: &Config,
) -> anyhow::Result<(String, Vec<FileDetail>, usize)> {
    let bpe = cl100k_base()?;
    
    // Sort descending by CPS initially
    entries.sort_by(|a, b| b.cps.partial_cmp(&a.cps).unwrap_or(std::cmp::Ordering::Equal));
    
    let budget = config.target_tokens.unwrap_or(50_000);
    
    let mut included_files = Vec::new();
    let mut dropped_files = Vec::new();
    let mut raw_tokens = 0;
    
    // Calculate raw tokens and floor budgets
    for entry in entries {
        if entry.role == FileRole::Generated {
            dropped_files.push((entry, "Generated file".to_string()));
            continue;
        }
        
        // Exclude self-referential
        let content_start = entry.content.chars().take(200).collect::<String>();
        if content_start.contains("<manifest>") && content_start.contains("<budget_summary>") {
            dropped_files.push((entry, "Self-referential output (v2)".to_string()));
            continue;
        }
        if entry.content.contains("[Summarized File") && entry.path.to_string_lossy().ends_with(".md") {
            dropped_files.push((entry, "Self-referential output (v1)".to_string()));
            continue;
        }
        
        let path_str = entry.path.to_string_lossy();
        // Just in case, exclude output path
        if let Some(ref out) = config.output_path {
            if path_str.ends_with(&out.to_string_lossy().to_string()) {
                dropped_files.push((entry, "Self-referential output".to_string()));
                continue;
            }
        }
        
        let tok = bpe.encode_with_special_tokens(&entry.content).len();
        raw_tokens += tok;
        included_files.push((entry, tok, 0)); // (entry, tokens, level)
    }
    
    let tokens_allowed = budget;
    
    // Assign floors
    let get_floor = |role: &FileRole, raw: usize| -> usize {
        match role {
            FileRole::CoreLogic | FileRole::EntryPoint => 200.max((raw as f32 * 0.4) as usize),
            FileRole::Interface | FileRole::Schema => raw, // verbatim
            FileRole::Documentation => if raw < 500 { raw } else { 300.max((raw as f32 * 0.2) as usize) },
            FileRole::Config => 50,
            FileRole::Test => 80,
            FileRole::DataFile => 60,
            FileRole::StyleSheet => 15,
            FileRole::Generated => 0,
        }
    };
    
    if raw_tokens > tokens_allowed {
        // Drop loop: if sum(floors) > budget, drop lowest CPS
        loop {
            let sum_floors: usize = included_files.iter().map(|(e, t, _)| get_floor(&e.role, *t)).sum();
            if sum_floors <= tokens_allowed || included_files.is_empty() {
                break;
            }
            if let Some(dropped) = included_files.pop() {
                let reason = format!("Budget exhausted, CPS {:.2}", dropped.0.cps);
                dropped_files.push((dropped.0, reason));
            }
        }
        
        let mut current_total: usize = included_files.iter().map(|(_, t, _)| *t).sum();
        
        // 1. Enforce max caps (15%) if total_used is approaching budget 
        let cap = (tokens_allowed as f32 * 0.15) as usize;
        if current_total > (tokens_allowed as f32 * 0.85) as usize {
            for (entry, tokens, lvl) in included_files.iter_mut() {
                let is_readme = entry.path.file_name().unwrap_or_default().to_string_lossy().to_lowercase() == "readme.md";
                if !is_readme && *tokens > cap {
                    let ext = entry.path.extension().and_then(|e| e.to_str());
                    for l in 1..=4 {
                        let compressed = compress_content(&entry.content, &entry.role, l, ext);
                        let tok = bpe.encode_with_special_tokens(&compressed).len();
                        *lvl = l;
                        entry.content = compressed;
                        *tokens = tok;
                        if *tokens <= cap {
                            break;
                        }
                    }
                }
            }
        }

        current_total = included_files.iter().map(|(_, t, _)| *t).sum();

        // 2. Progressively compress lowest CPS files first to meet budget
        let max_level_for_role = |role: &FileRole| -> u8 {
            if *role == FileRole::Documentation { 3 } else { 4 }
        };

        while current_total > tokens_allowed {
            let mut compressed_any = false;
            
            for i in (0..included_files.len()).rev() { // Start from lowest CPS
                let (entry, tokens, lvl) = &mut included_files[i];
                let is_readme = entry.path.file_name().unwrap_or_default().to_string_lossy().to_lowercase() == "readme.md";
                let actual_max_l = if is_readme { 2 } else { max_level_for_role(&entry.role) };

                if *lvl < actual_max_l {
                    let next_lvl = *lvl + 1;
                    let ext = entry.path.extension().and_then(|e| e.to_str());
                    let compressed = compress_content(&entry.content, &entry.role, next_lvl, ext);
                    let new_tok = bpe.encode_with_special_tokens(&compressed).len();
                    
                    if new_tok < *tokens {
                        current_total -= *tokens - new_tok;
                        *tokens = new_tok;
                        entry.content = compressed;
                        *lvl = next_lvl;
                        compressed_any = true;
                        
                        if current_total <= tokens_allowed { break; }
                    } else {
                        *lvl = next_lvl; 
                    }
                }
            }
            if !compressed_any { break; }
        }
    }
    
    // Output Formatting
    let mut out = String::new();
    
    let total_scanned = included_files.len() + dropped_files.len();
    let total_used: usize = included_files.iter().map(|(_, t, _)| *t).sum();
    let efficiency = if budget > 0 { (total_used as f64 / budget as f64) * 100.0 } else { 100.0 };
    
    let mut entry_points = Vec::new();
    for (f, _, _) in &included_files {
        if f.role == FileRole::EntryPoint { entry_points.push(f.path.to_string_lossy().to_string()); }
    }
    let mut build_errors_present = false;
    for (f, _, _) in &included_files {
        let name = f.path.file_name().unwrap_or_default().to_string_lossy().to_lowercase();
        if name == "error.txt" || name == "errors.txt" || name == "build_errors.txt" { build_errors_present = true; break; }
    }
    for (f, _) in &dropped_files {
        let name = f.path.file_name().unwrap_or_default().to_string_lossy().to_lowercase();
        if name == "error.txt" || name == "errors.txt" || name == "build_errors.txt" { build_errors_present = true; break; }
    }
    let repo_name = std::env::current_dir().unwrap_or_default().file_name().unwrap_or_default().to_string_lossy().to_string();

    out.push_str("<manifest>\n");
    out.push_str("  <repository_details>\n");
    out.push_str(&format!("    Project: {}\n", repo_name));
    if build_errors_present {
        out.push_str("    Build State: Errors present — codebase does not currently compile.\n");
    }
    out.push_str("    Entry Points:\n");
    for ep in entry_points.iter().take(8) { out.push_str(&format!("      - {}\n", ep)); }
    if entry_points.len() > 8 { out.push_str(&format!("      - ...and {} more\n", entry_points.len() - 8)); }
    out.push_str("  </repository_details>\n");
    out.push_str("  <budget_summary>\n");
    out.push_str(&format!("    Total files scanned: {} · Included: {} · Dropped: {}\n", 
        total_scanned, included_files.len(), dropped_files.len()));
    out.push_str(&format!("    Token budget: {} · Tokens used: {} · Efficiency: {:.1}%\n", 
        budget, total_used, efficiency));
    out.push_str("  </budget_summary>\n");
    out.push_str("</manifest>\n\n");
    
    // Section B - Drop Log
    out.push_str("[DROP LOG]\n");
    if dropped_files.is_empty() {
        out.push_str("  None.\n");
    } else {
        for (f, reason) in &dropped_files {
            out.push_str(&format!("  Excluded ({:?}): {} → {}\n", f.role, f.path.display(), reason));
        }
        out.push_str(&format!("  Note: {} files were dropped.\n", dropped_files.len()));
    }
    out.push_str("\n");
    
    // Section C - Included Files
    let mut file_details = Vec::new();
    for (entry, t, lvl) in &included_files {
        out.push_str(&format!("[{} · {:?} · CPS {:.2} · Level {} · {} tokens]\n", 
            entry.path.display(), entry.role, entry.cps, lvl, t));
        out.push_str("```\n");
        out.push_str(&entry.content);
        out.push_str("\n```\n\n");
        
        file_details.push(FileDetail {
            path: entry.path.to_string_lossy().to_string(),
            role: format!("{:?}", entry.role),
            cps: entry.cps,
            compression_level: *lvl,
            tokens: *t,
            included: true,
        });
    }
    
    for (f, _) in &dropped_files {
        file_details.push(FileDetail {
            path: f.path.to_string_lossy().to_string(),
            role: format!("{:?}", f.role),
            cps: f.cps,
            compression_level: 0, // It drops, we don't care exactly
            tokens: 0,
            included: false,
        });
    }
    
    // Section D - Compression Inventory
    out.push_str("[Compression Inventory]\n");
    out.push_str(format!("{:<40} {:<15} {:<6} {:<5} {:<8}\n", "FILE", "CLASS", "CPS", "LEVEL", "TOKENS").as_str());
    for (entry, t, lvl) in &included_files {
        let p = entry.path.to_string_lossy();
        let short_p = if p.len() > 38 { format!("{}..{}", &p[0..15], &p[p.len()-21..]) } else { p.to_string() };
        out.push_str(&format!("{:<40} {:<15} {:.2}   {:<5} {:<8}\n", short_p, format!("{:?}", entry.role), entry.cps, lvl, t));
    }
    for (entry, _) in &dropped_files {
        let p = entry.path.to_string_lossy();
        let short_p = if p.len() > 38 { format!("{}..{}", &p[0..15], &p[p.len()-21..]) } else { p.to_string() };
        out.push_str(&format!("{:<40} {:<15} {:.2}   {:<5} {:<8}\n", short_p, format!("{:?}", entry.role), entry.cps, "DROP", "0"));
    }
    
    Ok((out, file_details, total_used))
}
