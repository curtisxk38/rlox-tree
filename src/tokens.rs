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
    Identifier, Strin, Number,                      

    // Keywords.                                     
    And, Class, Else, False, Fun, For, If, Nil, Or,  
    Print, Return, Super, This, True, Var, While,    

    EOF                                              
}

#[derive(Debug)]
pub struct Token { 
    token_type: TokenType,
    lexeme: String,                                    
    literal: Option<LiteralValue>,                                     
    line: i32                                                       
}

#[derive(Debug)]
pub enum LiteralValue {
    NumberValue(f64),
    StringValue(String),
    BooleanValue(bool),
    NilValue
}
