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
            match self.chars[self.i] {
                ' ' | '\t' | '\n' => self.i += 1,

                'a'..='z' | 'A'..='Z' => self.tokenizer_char(),

                '0'..='9' => self.tokenizer_number(),

                _ => {
                    self.i += 1;
                    // continue;
                }
            }
        }
    }
}