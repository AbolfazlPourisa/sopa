mod lexer;

use lexer::Lexer;

fn main() {
    let code: String = String::from(r#"let = -213"#);

    let mut lex = Lexer::new(code);

    lex.lex();

    println!("{:?}", lex.tokens);
}
