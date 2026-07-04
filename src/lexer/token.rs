#[derive(Debug, PartialEq)]
pub enum Tokens {
    Literal(Literal),
    Keyword(Keyword),
    Operator(Operator),
    Delimiter(Delimiter),
    Identifier(String),
    EOF,
    Unknown(String)
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    Int(i64),
    Float(f64),
    LiteralString(String),
    Boolean(bool)
}

#[derive(Debug, PartialEq)]
pub enum Keyword {
    Let,
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    Plus, 
    Minus, 
    Star, 
    Slash,
    Percent, 
    Caret, 
    Equal,
    And, 
    Or, 
    EqEq, 
    BangEq,
    Lt, 
    Gt, 
    LtEq, 
    GtEq,
    Bang,
    PlusEq, MinusEq,
}

#[derive(Debug, PartialEq)]
pub enum Delimiter {
    Dot, 
    LParen, 
    RParen, 
    LBrace,
    RBrace, 
    LBracket, 
    RBracket, 
    Comma,
}

impl Into<Tokens> for i64 {
    fn into(self) -> Tokens {
        Tokens::Literal(Literal::Int(self))
    }
}

impl Into<Tokens> for f64 {
    fn into(self) -> Tokens {
        Tokens::Literal(Literal::Float(self))
    }
}

impl Into<Tokens> for bool {
    fn into(self) -> Tokens {
        Tokens::Literal(Literal::Boolean(self))
    }
}

impl Into<Tokens> for String {
    fn into(self) -> Tokens {
        Tokens::Literal(Literal::LiteralString(self))
    }
}