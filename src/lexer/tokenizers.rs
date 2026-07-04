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
                let has_next = self.i + 1 < self.chars.len();
                let mut next_is_digit = false;
                if has_next {
                    next_is_digit = self.chars[self.i + 1].is_digit(10);
                }

                let has_prev = self.i > 0;
                let mut prev_is_digit = false;
                if has_prev {
                    prev_is_digit = self.chars[self.i - 1].is_digit(10);
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