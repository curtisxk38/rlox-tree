#[derive(Debug)]
pub enum TokenType {                                   
    // Single-character tokens.                      
    LEFT_PAREN, RIGHT_PAREN, LEFT_BRACE, RIGHT_BRACE,
    COMMA, DOT, MINUS, PLUS, SEMICOLON, SLASH, STAR, 

    // One or two character tokens.                  
    BANG, BANG_EQUAL,                                
    EQUAL, EQUAL_EQUAL,                              
    GREATER, GREATER_EQUAL,                          
    LESS, LESS_EQUAL,                                

    // Literals.                                     
    IDENTIFIER, STRING, NUMBER,                      

    // Keywords.                                     
    AND, CLASS, ELSE, FALSE, FUN, FOR, IF, NIL, OR,  
    PRINT, RETURN, SUPER, THIS, TRUE, VAR, WHILE,    

    EOF                                              
}

#[derive(Debug)]
pub struct Token { 
    token_type: TokenType,
    lexeme: String,                                    
    //final Object literal,                                          
    line: u32                                                       
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: u32) -> Token {
        return Token { token_type, lexeme, line };
    }
}