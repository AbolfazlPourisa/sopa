use super::token::Tokens;
use super::lexer::Lexer;

impl Lexer {
    pub fn new(code: String) -> Self {
        Self {
            tokens: Vec::new(),
            chars: code.chars().collect(),
            code: code,
            i: 0,
        }
    }

    pub fn lex(&mut self) {
        while self.i < self.chars.len() {
            // match chars[self.i] {
                // ' ' | '\t' | '\n' => self.i += 1,
            // }
            self.add_let();
        }
    }
}