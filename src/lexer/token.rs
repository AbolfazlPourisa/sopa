pub enum tokens {
    Let, Import,

    Identifier(String),
    
    TypeString(String), TypeInt(i32), TypeFloat(f32), TypeBoolean(bool),
    
    Plus, Minus, Star, Slash,
    Percent, Caret,
     
    Equal,
    
    And, Or, EqEq, BangEq,
    Lt, Gt, LtEq, GtEq,

    Bang,

    PlusEq,MinusEq,
    
    Dot, LParen, RParen, LBrace,
    RBrace, LBracket, RBracket, Comma,

    EOF,

    Unkown(char)
}