use super::token::Tokens;
use super::token::Operator;
use super::token::Literal;
use super::token::Keyword;
use super::lexer::Lexer;
use super::error::LexerError;

impl Lexer {
    pub fn tokenizer_char(&mut self) {
        let mut ident = String::new();

        while self.i < self.chars.len() && (self.chars[self.i].is_alphabetic() || self.chars[self.i] == '_') {
            ident.push(self.chars[self.i]);

            self.i += 1;
        }

        match ident.as_str() {
            "let" => {
                self.add_single(Tokens::Keyword(Keyword::Let));

                return;
            },

            "and" => {
                self.add_single(Tokens::Operator(Operator::And));
                
                return;
            },

            "or" => {
                self.add_single(Tokens::Operator(Operator::Or));
            
                return;
            }

            _ => {
                self.add_identifier(ident);

                return
            }
        }
    }

    pub fn tokenizer_string(&mut self) -> Result<(), LexerError> {
        let mut string = String::new();
        
        let mut qoute_count = 1;

        self.i += 1;

        while self.i < self.chars.len() && self.chars[self.i] != '"' {
            string.push(self.chars[self.i]);

            self.i += 1;
        }

        if let Some(ch) = self.chars.get(self.i) {
            if *ch == '"' {
                qoute_count += 1;
            }
        }

        if qoute_count != 2 {
            return Err(LexerError::InvalidString());
        }

        self.i += 1;

        self.add_literal::<String>(string)?;

        Ok(())
    }

    pub fn tokenizer_number(&mut self) -> Result<(), LexerError> {
        let mut number = String::new();
        let mut is_float = false;
        let mut is_negative = false;

        while self.i < self.chars.len() {
            let ch = self.chars[self.i];

            if ch == '-' {
                if is_negative || number.len() > 0 {
                    return Err(LexerError::InvalidNumber());
                }

                is_negative = true;

                number.push('-');

                self.i += 1;

                continue;
            }

            if ch.is_digit(10) {
                number.push(ch);

                self.i += 1;

                continue;
            }

            if ch == '.' {
                let mut next_is_digit = false;
                if let Some(ch) = self.chars.get(self.i + 1) {
                    next_is_digit = ch.is_digit(10);
                }

                let mut prev_is_digit = false;
                if self.i > 0 {
                    if let Some(ch) = self.chars.get(self.i - 1) {
                        prev_is_digit = ch.is_digit(10);
                    }
                }

                if !is_float && (prev_is_digit && next_is_digit) {
                    is_float = true;
                    
                    number.push(ch);

                    self.i += 1;

                    continue;
                } else {
                    return Err(LexerError::InvalidFloat());
                }
            }

            break;
        }

        if is_float {
            self.add_literal::<f64>(number)?;

            return Ok(());
        }

        self.add_literal::<i64>(number)?;

        Ok(())
    }
}