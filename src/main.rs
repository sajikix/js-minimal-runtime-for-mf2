mod lexer;
mod parser;
mod printer;
mod runtime;

use crate::parser::Parser;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let input = if args.len() > 1 {
        // ファイルパスが指定された場合
        let filename = &args[1];
        match fs::read_to_string(filename) {
            Ok(contents) => contents,
            Err(err) => {
                eprintln!("Error reading file '{}': {}", filename, err);
                std::process::exit(1);
            }
        }
    } else {
        // デフォルトのテストコード
        "const mf = new Intl.MessageFormat(\"en\", \"Hello {$place}!\"); mf.format({ place: \"world\" });".to_string()
    };

    println!("{}", input);

    let mut parser = Parser::new(&input);
    let program = parser.parse();
    let mut runtime = runtime::Runtime::new();
    runtime.execute(program.clone());
    // println!("{:?}", program);
}
