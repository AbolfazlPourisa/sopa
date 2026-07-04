use crate::lexer::Tokens::TypeString;

use super::token::Tokens;
use super::lexer::Lexer;

impl Lexer {
    pub fn tokenizer_char(&mut self) {
        let mut ident = String::new();

        while self.i < self.chars.len() && (self.chars[self.i].is_alphabetic() || self.chars[self.i] == '_') {
            ident.push(self.chars[self.i]);

            self.i += 1;
        }

        match ident.as_str() {
            "let" => {
                self.add_single(Tokens::Let);

                return;
            },

            "and" => {
                self.add_single(Tokens::And);
                
                return;
            },

            "or" => {
                self.add_single(Tokens::Or);
            
                return;
            }

            _ => {
                self.add_identifier(ident);

                return
            }
        }
    }

    pub fn tokenizer_string(&mut self) {
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
            panic!("Invalid string");
        }

        self.i += 1;

        self.add_type::<String>(string).expect("Invalid string");
    }

    pub fn tokenizer_number(&mut self) {
        let mut number = String::new();
        let mut is_float = false; 

        while self.i < self.chars.len() {
            let ch = self.chars[self.i];

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
                    panic!("Invalid float");
                }
            }

            break;
        }

        if is_float {
            match self.add_type::<f64>(number) {
                Ok(_) => {
                    return;
                },

                Err(err) => {
                    panic!("{}", err);
                }
            }
        }

        match self.add_type::<i64>(number) {
            Ok(_) => {
                return;
            },

            Err(err) => {
                panic!("{}", err);
            }
        }
    }
}