use oxc::{
    allocator::Allocator,
    parser::{Parser, ParserReturn},
    span::SourceType,
};

use std::fs;
use std::path::Path;

pub struct NgTree {
    // todo
}

// Throw away this code. Just keeping it for reference right now.

impl NgTree {
    pub fn parse(main_ts_path: &str) -> NgTree {
        let source_path = Path::new(main_ts_path);
        let source_content = fs::read_to_string(source_path).unwrap();
        let allocator = Allocator::default();
        let source_type = SourceType::from_path(source_path).unwrap();
        let mut errors = Vec::new();

        let ParserReturn {
            program,
            errors: parser_errors,
            panicked,
            ..
        } = Parser::new(&allocator, &source_content, source_type).parse();

        errors.extend(parser_errors);

        println!("{}", program.source_text);

        NgTree {}
    }
}
