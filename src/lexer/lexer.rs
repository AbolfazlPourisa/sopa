use super::token::Tokens;

pub struct Lexer {
    pub tokens: Vec<Tokens>,
    pub code: String,
    pub i: usize,
    pub chars: Vec<char>
}