mod lexer;

use lexer::Lexer;
use lexer::LexerError;

fn main() -> Result<(), LexerError> {
    // let code= r#"let = 2."#;
    let code= r#"let name = "Sopa"
    
let age = 0
let average = 19.83
let test 

"#;

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
