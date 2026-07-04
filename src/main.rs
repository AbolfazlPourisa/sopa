mod lexer;

use lexer::Lexer;

fn main() {
    let code: String = String::from("let test = 12");

    let mut lex = Lexer::new(code);

    lex.lex();

    println!("{:?}", lex.tokens);
}
