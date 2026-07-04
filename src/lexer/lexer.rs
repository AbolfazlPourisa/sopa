use super::token::Tokens;

pub struct Lexer<'a> {
    pub tokens: Vec<Tokens>,
    pub code: &'a str,
    pub lines: Vec<&'a str>,
    pub i: usize,
    pub chars: Vec<char>,
    pub line: usize,
}