use super::token::Tokens;
use super::token::Operator;
use super::token::Keyword;
use super::token::Delimiter;
use super::lexer::Lexer;
use super::error::LexerError;

impl<'a> Lexer<'a> {
    pub fn tokenizer_char(&mut self) {
        let mut ident = String::new();

        while self.i < self.chars.len() && (self.chars[self.i].is_alphabetic() || self.chars[self.i] == '_' || self.chars[self.i].is_digit(10)) {
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
            return Err(
                LexerError::InvalidString{
                    line_number: self.line,
                    line: self.get_current_line(),
                    error: "" .to_string()                   
                }
            );
        }

        self.i += 1;

        self.add_literal::<String>(string)?;

        Ok(())
    }

    pub fn tokenizer_number(&mut self) -> Result<(), LexerError> {
        let mut number = String::new();
        let mut is_float = false;

        while self.i < self.chars.len() {
            let ch = self.chars[self.i];

            if ch.is_digit(10) {
                number.push(ch);
                self.i += 1;
            } else if ch == '.' && !is_float {
                is_float = true;
                number.push(ch);
    
                self.i += 1;

            } else {
                break;
            }
        }
        
        if number.ends_with('.') {
            is_float = false;
            number.pop();

            self.i -= 1;
        }

        if is_float {
            self.add_literal::<f64>(number)?;

        } else {
            self.add_literal::<i64>(number)?;
        }

        Ok(())
    }


    pub fn tokenizer_delimiter(&mut self) {
        match self.chars[self.i] {
            '.' => {
                self.add_single(Tokens::Delimiter(Delimiter::Dot));
            }

            '(' => {
                self.add_single(Tokens::Delimiter(Delimiter::LParen));
            }

            ')' => {
                self.add_single(Tokens::Delimiter(Delimiter::RParen));
            }

            '{' => {
                self.add_single(Tokens::Delimiter(Delimiter::LBrace));
            }

            '}' => {
                self.add_single(Tokens::Delimiter(Delimiter::RBrace));
            }

            '[' => {
                self.add_single(Tokens::Delimiter(Delimiter::RBracket));
            }

            ']' => {
                self.add_single(Tokens::Delimiter(Delimiter::LBracket));
            }
            
            ',' => {
                self.add_single(Tokens::Delimiter(Delimiter::Comma));
            }

            '-' => {
                self.add_single(Tokens::Delimiter(Delimiter::Dash));
            }

            _ => {
                println!("من توی حفلم");
                // panic!("مردم داداش");
            }
        }

        self.i += 1;
    }
}