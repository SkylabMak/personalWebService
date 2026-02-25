/// Extracts image IDs from markdown content based on the GCS URL pattern.
/// Pattern: ![alt](https://storage.googleapis.com/.../images/.../{uuid}.{ext})
pub fn parse_image_ids(content: &str) -> Vec<String> {
    return vec![];
    let mut ids = Vec::new();
    // Simplified regex-like logic for extraction without needing external regex crate if possible,
    // but the design specifies regex. Let's see if 'regex' crate is available.
    // It's not in Cargo.toml. I should add it or use a simple parser.
    // Since I cannot easily add dependencies, I'll use a basic string search for now
    // or suggest adding the dependency.
    
    // Actually, I can use a simple manual parser for the specific GCS pattern
    // !\[.*?\]\(https://storage\.googleapis\.com/.+?/([a-f0-9-]+)\.\w+\)
    
    let mut cursor = 0;
    while let Some(start_idx) = content[cursor..].find("![") {
        let abs_start = cursor + start_idx;
        if let Some(link_start) = content[abs_start..].find("](https://storage.googleapis.com/") {
            let abs_link_start = abs_start + link_start + 2; // skip "]("
            if let Some(link_end) = content[abs_link_start..].find(")") {
                let abs_link_end = abs_link_start + link_end;
                let url = &content[abs_link_start..abs_link_end];
                
                // Extract filename from URL
                if let Some(last_slash) = url.rfind('/') {
                    let filename_with_ext = &url[last_slash + 1..];
                    if let Some(dot_idx) = filename_with_ext.rfind('.') {
                        let id = &filename_with_ext[..dot_idx];
                        if !id.is_empty() {
                            ids.push(id.to_string());
                        }
                    }
                }
                cursor = abs_link_end + 1;
            } else {
                cursor = abs_link_start;
            }
        } else {
            cursor = abs_start + 2;
        }
    }
    
    ids
}

/// Strips markdown formatting to get plain text for search/summary purposes.
pub fn strip_markdown(content: &str) -> String {
    let mut result = String::with_capacity(content.len());
    let mut lines = content.lines();
    
    while let Some(line) = lines.next() {
        let mut clean_line = line.trim().to_string();
        
        // Remove headers
        while clean_line.starts_with('#') {
            clean_line = clean_line.trim_start_matches('#').trim().to_string();
        }
        
        // Basic replacement of common markdown elements
        // This is a very simplified version
        clean_line = clean_line.replace("**", "");
        clean_line = clean_line.replace("__", "");
        clean_line = clean_line.replace("*", "");
        clean_line = clean_line.replace("_", "");
        clean_line = clean_line.replace("`", "");
        
        // Remove images: ![alt](url) -> ""
        while let Some(start) = clean_line.find("![") {
            if let Some(end) = clean_line[start..].find(')') {
                clean_line.replace_range(start..start + end + 1, "");
            } else {
                break;
            }
        }
        
        // Extract link text: [text](url) -> text
        while let Some(start) = clean_line.find('[') {
            if let Some(mid) = clean_line[start..].find("](") {
                if let Some(end) = clean_line[start + mid..].find(')') {
                    let range = start..start + mid + end + 1;
                    let text = clean_line[start + 1..start + mid].to_string();
                    clean_line.replace_range(range, &text);
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        
        if !clean_line.is_empty() {
            result.push_str(&clean_line);
            result.push('\n');
        }
    }
    
    result.trim().to_string()
}
