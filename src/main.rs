mod lexer;
mod parser;
use crate::{lexer::Lexer, parser::Parser};
fn main() {
    let input = "const mf = new Intl.MessageFormat(\"en\", \"Hello {$place}!\"); mf.format({ place: \"world\" });".to_string();
    let mut lexer = Lexer::new(&input);
    // 終わるまでトークンを取得し続ける
    loop {
        let token = lexer.next();
        println!("{:?}", token);
        if token.is_none() {
            break;
        }
    }

    let mut parser = Parser::new(&input);
    let program = parser.parse();
    println!("{:?}", program);
}
