mod lexer;

use crate::lexer::Lexer;
fn main() {
    let input = "var foo=\"bar\";".to_string();
    let mut lexer = Lexer::new(&input);
    // 終わるまでトークンを取得し続ける
    loop {
        let token = lexer.next();
        println!("{:?}", token);
        if token.is_none() {
            break;
        }
    }
}
