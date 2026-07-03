mod lexer;

use lexer::Lexer;

fn main() {
    let code: String = String::from("let test = 123");

    let mut lex = Lexer::new(code);

    println!("{:?}", lex.tokens);
}
