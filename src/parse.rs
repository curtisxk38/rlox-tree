use std::{iter::Peekable, slice::Iter};

use crate::tokens::Token;


pub(crate) struct Parser {

}

impl Parser {

    // program -> expression* EOF ;
    pub fn parse(&self, tokens: &Vec<Token>) {
        let tokens = tokens.iter().peekable();
        self.expression(tokens)
    }

    // expression -> equality
    fn expression(&self, tokens: Peekable<Iter<Token>>) {
        self.equality(tokens)
    }

    // equality -> comparison ( ( "!=" | "==" ) comparison )* ;
    fn equality(&self, tokens: Peekable<Iter<Token>>) {

    }

    // comparison -> term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
    fn comparison(&self, tokens: Peekable<Iter<Token>>) {

    }

    // term -> factor ( ( "-" | "+") factor )* ;
    fn term(&self, tokens: Peekable<Iter<Token>>) {

    }

    // factor -> unary ( ( "/" | "*") unary )* ;
    fn factor(&self, tokens: Peekable<Iter<Token>>) {

    }

    // unary -> ( "!" | "-" ) unary
    //       | primary ;
    fn unary(&self, tokens: Peekable<Iter<Token>>) {

    }

    // primary -> NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;
    fn primary(&self, tokens: Peekable<Iter<Token>>) {
        
    }
}