mod lexer;

use lexer::Lexer;
use lexer::LexerError;

fn main() -> Result<(), LexerError> {
    let code= r#"let = 2.12."#;

    let mut lex = Lexer::new(code);

    match lex.lex() {
        Ok(_) => {
            println!("{:?}", lex.tokens);
        },

        Err(e) => {
            println!("{}", e);
        }
    }
    // println!("{}", lex.get_current_line());

    Ok(())
}
