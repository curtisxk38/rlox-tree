use crate::tokens::{LiteralValue, Token};


#[derive(Debug)]
pub(crate) enum BinaryOperator {
    BangEqual,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Minus,
    Plus,
    Slash,
    Star,
}

#[derive(Debug)]
pub(crate) enum UnaryOperator {
    Bang,
    Minus,
}

#[derive(Debug)]
pub(crate) enum Expr<'a> {
    Binary(Binary<'a>),
    Unary(Unary<'a>),
    Literal(Literal<'a>),
    Variable(Variable<'a>)
}

#[derive(Debug)]
pub(crate) struct Binary<'a> {
    pub token: &'a Token<'a>,
    pub operator: BinaryOperator,
    pub left: Box<Expr<'a>>,
    pub right: Box<Expr<'a>>,
}

#[derive(Debug)]
pub(crate) struct Unary<'a> {
    pub token: &'a Token<'a>,
    pub operator: UnaryOperator,
    pub right: Box<Expr<'a>>
}

#[derive(Debug)]
pub(crate) struct Literal<'a> {
    pub token: &'a Token<'a>,
    pub value: LiteralValue,
}

#[derive(Debug)]
pub(crate) struct Variable<'a> {
    pub token: &'a Token<'a>
}

pub(crate) enum Statement<'a> {
    ExpressionStatement(ExpressionStatement<'a>),
    PrintStatement(PrintStatement<'a>)
}

pub(crate) struct ExpressionStatement<'a> {
    pub expression: Expr<'a>
}

pub(crate) struct PrintStatement<'a> {
    pub token: &'a Token<'a>,
    pub value: Expr<'a>
}