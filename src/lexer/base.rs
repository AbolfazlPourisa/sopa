use super::token::Tokens;
use super::lexer::Lexer;
use super::error::LexerError;

impl<'a> Lexer<'a> {
    pub fn new(code: &'a str) -> Self {
        Self {
            tokens: Vec::new(),
            chars: code.chars().collect(),
            lines: code.lines().collect(),
            code: code,
            i: 0,
            line: 1,
        }
    }

    pub fn lex(&mut self) -> Result<(), LexerError> {
        while self.i < self.chars.len() {
            match self.chars[self.i] {
                ' ' | '\t' => self.i += 1,

                '\n' => {
                    self.line += 1;
                    self.i += 1;
                }

                'a'..='z' | 'A'..='Z' => self.tokenizer_char(),

                '0'..='9' => self.tokenizer_number()?,

                '"' => self.tokenizer_string()?,

                '.' | '(' | ')' | '{' | '}' | '[' | ']' | ',' | '-' => self.tokenizer_delimiter(),

                _ => {
                    self.i += 1;
                }
            }
        }

        Ok(())
    }
}