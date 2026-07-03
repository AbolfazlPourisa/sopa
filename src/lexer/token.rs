#[derive(Debug, PartialEq)]
pub enum Tokens {
    Let,

    Identifier(String),
    
    TypeString(String), TypeInt(i64), TypeFloat(f64), TypeBoolean(bool),
    
    Plus, Minus, Star, Slash,
    Percent, Caret,
     
    Equal,
    
    And, Or, EqEq, BangEq,
    Lt, Gt, LtEq, GtEq,

    Bang,

    PlusEq, MinusEq,
    
    Dot, LParen, RParen, LBrace,
    RBrace, LBracket, RBracket, Comma,

    EOF,

    Unknown(String)
}