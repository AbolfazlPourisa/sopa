use super::token::Tokens;
use super::lexer::Lexer;

impl Lexer {
    pub fn add_single(&mut self, token: Tokens) {
        self.tokens.push(token);
    }

    pub fn add_identifier(&mut self, value: String) {
        self.add_single(Tokens::Identifier(value));        
    }

    pub fn add_literal<T>(&mut self, value: String) -> Result<(), String> 
    where 
        T: std::str::FromStr + Into<Tokens>,
        <T as std::str::FromStr>::Err: std::fmt::Display
    {
        match value.parse::<T>() {
            Ok(parsed) => {
                self.tokens.push(
                    parsed.into()
                );

                Ok(())
            }

            Err(_) => Err(format!("Invalid type: {}", value))
        }
    }

    pub fn add_unknown(&mut self, value: String) {
        self.add_single(Tokens::Unknown(value));
    }
}