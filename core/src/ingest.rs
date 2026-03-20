use crate::discover::{classify_role, calculate_cps, ScoredFile};
use ignore::WalkBuilder;
use mime_guess::from_path;

pub fn walk_directory(path: &str) -> anyhow::Result<Vec<ScoredFile>> {
    let mut entries = Vec::new();
    let walker = WalkBuilder::new(path)
        .hidden(false)
        .git_ignore(true)
        .build();

    for result in walker {
        let entry = result?;
        let path = entry.path();
        
        if path.is_file() {
            let mime = from_path(path).first_or_octet_stream();
            if mime.type_() == "text" || mime.subtype() == "xml" || mime.subtype() == "json" {
                if let Ok(content) = std::fs::read_to_string(path) {
                    let role = classify_role(path, &content);
                    let cps = calculate_cps(&role, path, &content);
                    
                    entries.push(ScoredFile {
                        path: path.to_path_buf(),
                        content,
                        role,
                        cps,
                        compression_level: 0,
                    });
                }
            }
        }
    }
    
    // Sort by CPS descending so most important files are prioritized
    entries.sort_by(|a, b| b.cps.partial_cmp(&a.cps).unwrap_or(std::cmp::Ordering::Equal));
    
    Ok(entries)
}
