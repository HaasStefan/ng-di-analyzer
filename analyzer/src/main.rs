
use analyzer::core::parser::file;

fn main() {
    let path = String::from("../angular-workspace/projects/app/src/main.ts");
    let parsed_file = file::parse(&path) 
        .expect("Failed to parse file");

    println!("Parsed file: {:?}", parsed_file.content);

}
