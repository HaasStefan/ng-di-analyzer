use std::fs;
use std::io;

pub struct ParsedFile {
    pub path: String,
    pub content: String,
}

pub fn parse(path: &str) -> io::Result<ParsedFile> {
    let content = fs::read_to_string(path)?;

    Ok(ParsedFile {
        path: path.to_owned(),
        content: content,
    })
}
