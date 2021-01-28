#[derive(Debug)]
pub enum TokenType {                                   
    // Single-character tokens.                      
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star, 

    // One or two character tokens.                  
    Bang, BangEqual,                                
    Equal, EqualEqual,                              
    Greater, GreaterEqual,                          
    Less, LessEqual,                                

    // Literals.                                     
    Identifier, String, Number,

    // Keywords.                                     
    And, Class, Else, False, Fun, For, If, Nil, Or,  
    Print, Return, Super, This, True, Var, While,    

    EOF                                              
}

#[derive(Debug)]
pub struct Token<'a> { 
    pub token_type: TokenType,
    pub lexeme: &'a str,
    pub literal: Option<LiteralValue>,
    pub line: i32
}

#[derive(Debug, Clone)]
pub enum LiteralValue {
    NumberValue(f64),
    StringValue(String),
    BooleanValue(bool),
    NilValue
}
