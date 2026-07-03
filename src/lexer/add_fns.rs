use super::token::Tokens;
use super::lexer::Lexer;

impl Lexer {
    fn add_simple_token(&mut self, token: Tokens) {
        self.tokens.push(token);
    }

    pub fn add_let(&mut self) {
        self.add_simple_token(Tokens::Let);
    }

    pub fn add_plus(&mut self) {
        self.add_simple_token(Tokens::Plus);
    }

    pub fn add_minus(&mut self) {
        self.add_simple_token(Tokens::Minus);
    }

    pub fn add_star(&mut self) {
        self.add_simple_token(Tokens::Star);
    }

    pub fn add_slash(&mut self) {
        self.add_simple_token(Tokens::Slash);
    }

    pub fn add_percent(&mut self) {
        self.add_simple_token(Tokens::Percent);
    }

    pub fn add_caret(&mut self) {
        self.add_simple_token(Tokens::Caret);
    }

    pub fn add_equal(&mut self) {
        self.add_simple_token(Tokens::Equal);
    }

    pub fn add_and(&mut self) {
        self.add_simple_token(Tokens::And);
    }

    pub fn add_or(&mut self) {
        self.add_simple_token(Tokens::Or);
    }

    pub fn add_eqeq(&mut self) {
        self.add_simple_token(Tokens::EqEq);
    }

    pub fn add_bang_eq(&mut self) {
        self.add_simple_token(Tokens::BangEq);
    }

    pub fn add_lt(&mut self) {
        self.add_simple_token(Tokens::Lt);
    }

    pub fn add_gt(&mut self) {
        self.add_simple_token(Tokens::Gt);
    }

    pub fn add_lt_eq(&mut self) {
        self.add_simple_token(Tokens::LtEq);
    }

    pub fn add_gt_eq(&mut self) {
        self.add_simple_token(Tokens::GtEq);
    }

    pub fn add_bang(&mut self) {
        self.add_simple_token(Tokens::Bang);
    }

    pub fn add_plus_eq(&mut self) {
        self.add_simple_token(Tokens::PlusEq);
    }

    pub fn add_minus_eq(&mut self) {
        self.add_simple_token(Tokens::MinusEq);
    }

    pub fn add_dot(&mut self) {
        self.add_simple_token(Tokens::Dot);
    }

    pub fn add_lparen(&mut self) {
        self.add_simple_token(Tokens::LParen);
    }

    pub fn add_rparen(&mut self) {
        self.add_simple_token(Tokens::RParen);
    }

    pub fn add_lbrace(&mut self) {
        self.add_simple_token(Tokens::LBrace);
    }

    pub fn add_rbrace(&mut self) {
        self.add_simple_token(Tokens::RBrace);
    }

    pub fn add_lbracket(&mut self) {
        self.add_simple_token(Tokens::LBracket);
    }

    pub fn add_rbracket(&mut self) {
        self.add_simple_token(Tokens::RBracket);
    }

    pub fn add_comma(&mut self) {
        self.add_simple_token(Tokens::Comma);
    }

    pub fn add_eof(&mut self) {
        self.add_simple_token(Tokens::EOF);
    }

    pub fn add_identifier(&mut self, value: String) {
        self.tokens.push(Tokens::Identifier(value));        
    }

    pub fn add_type_string(&mut self, value: String) {
        self.tokens.push(Tokens::TypeString(value));
        
        self.i += 1;
    }

    pub fn add_type_int(&mut self, value: String) -> Result<(), String> {
        match value.parse::<i64>() {
            Ok(int) => {
                self.tokens.push(
                    Tokens::TypeInt(
                        int
                    )
                );

                Ok(())
            }

            Err(_) => Err(format!("Invalid integer: {}", value))
        }
    }

    pub fn add_type_float(&mut self, value: String) -> Result<(), String> {
        match value.parse::<f64>() {
            Ok(float) => {
                self.tokens.push(
                    Tokens::TypeFloat(
                        float
                    )
                );

                Ok(())
            }

            Err(_) => Err(format!("Invalid float: {}", value))
        }
    }

    pub fn add_type_boolean(&mut self, value: String) -> Result<(), String> {
        match value.parse::<bool>() {
            Ok(boolean) => {
                self.tokens.push(
                    Tokens::TypeBoolean(
                        boolean
                    )
                );

                Ok(())
            }

            Err(_) => Err(format!("Invalid boolean: {}", value))
        }
    }

    pub fn add_unknown(&mut self, value: String) {
        self.tokens.push(Tokens::Unknown(value));
    }
}