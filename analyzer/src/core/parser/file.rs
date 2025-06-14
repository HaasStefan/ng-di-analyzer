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


// ast might be a different type
// pub fn get_import_declarations(ast: &oxc::ast::ast::Program) -> Vec<oxc::ast::ast::ImportDeclaration> {
    // ast.body.iter().filter_map(|node| {
    //     if let oxc::ast::ast::Node::ImportDeclaration(import_decl) = node {
    //         Some(import_decl.clone())
    //     } else {
    //         None
    //     }
    // }).collect()
// }