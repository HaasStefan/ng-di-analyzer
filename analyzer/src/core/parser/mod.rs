use oxc::ast::ast::Program;

pub mod file;


// TODO: learn about lifetimes
pub struct ParsedTypeScriptFile<'a> {
    pub ast: Program<'a>,
    pub path: String,
    pub decorators: Vec<String>, // todo: fix should be tuples Vec<(Enum, ref to node in ast)>
    pub import_declarations: Vec<String>, // todo: fix should be tuples Vec<(ref to node in ast)>
    // todo: routes, injection tokens
}

pub trait TypeScriptParser {
    fn parse(&self, path: &str) -> Result<ParsedTypeScriptFile, String>;
}