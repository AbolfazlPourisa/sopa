mod lexer;

use lexer::Lexer;
use lexer::LexerError;

fn main() -> Result<(), LexerError> {
    let code: String = String::from(r#"let = .2"#);

    let mut lex = Lexer::new(code);

    lex.lex()?;

    println!("{:?}", lex.tokens);

    Ok(())
}
