mod lexer;
mod parser;
mod runtime;
use crate::parser::Parser;
fn main() {
    let input = "const mf = new Intl.MessageFormat(\"en\", \"Hello {$place}!\"); mf.format({ place: \"world\" });".to_string();
    let mut parser = Parser::new(&input);
    let program = parser.parse();
    let mut runtime = runtime::Runtime::new();
    runtime.execute(program.clone());
    // println!("{:?}", program);
}
