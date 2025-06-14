use std::fs;

pub struct ParsedFile {
    pub path: String,
    pub content: String,
}

pub fn parse(path: &str) -> Result<ParsedFile, String> {
    let content = fs::read_to_string(path);
    match content {
        Ok(content) => Ok(ParsedFile {
            path: path.to_owned(),
            content: content.clone(),
        }),
        Err(e) => Err(format!("Failed to read file {}: {}", path, e)),
    }
}
