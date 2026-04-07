/// Extracts image IDs from markdown content based on the GCS URL pattern.
/// Pattern: ![alt](https://storage.googleapis.com/.../images/.../{uuid}.{ext})
pub fn parse_image_ids(content: &str) -> Vec<String> {
    let mut ids = Vec::new();
    let mut remaining = content;

    while let Some(start_idx) = remaining.find("![") {
        remaining = &remaining[start_idx + 2..];
        if let Some(link_start_rel) = remaining.find("](") {
            let url_start = link_start_rel + 2;
            let after_bracket = &remaining[url_start..];
            if let Some(link_end_rel) = after_bracket.find(')') {
                let url = &after_bracket[..link_end_rel];
                
                if let Some(last_slash) = url.rfind('/') {
                    let filename_with_ext = &url[last_slash + 1..];
                    if let Some(dot_idx) = filename_with_ext.rfind('.') {
                        let id = &filename_with_ext[..dot_idx];
                        if !id.is_empty() {
                            ids.push(id.to_string());
                        }
                    }
                }
                remaining = &after_bracket[link_end_rel + 1..];
            }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_image_ids() {
        let content = "![image](https://storage.googleapis.com/personal-website_storage/performance_image/profile_001/4c8e1e80-5970-475a-b170-6fabffaa4a4c.jpg) some text ![another](https://example.com/images/img2.png)";
        let ids = parse_image_ids(content);
        assert_eq!(ids, vec!["4c8e1e80-5970-475a-b170-6fabffaa4a4c", "img2"]);
    }
}
