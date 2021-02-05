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
pub(crate) enum LogicalOperator {
    And,
    Or
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
    Grouping(Grouping<'a>),
    Variable(Variable<'a>),
    Assignent(Assignent<'a>),
    Logical(Logical<'a>),
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
pub(crate) struct Grouping<'a> {
    pub expr: Box<Expr<'a>>
}

#[derive(Debug)]
pub(crate) struct Variable<'a> {
    pub token: &'a Token<'a>
}

#[derive(Debug)]
pub(crate) struct Assignent<'a> {
    pub token: &'a Token<'a>,
    pub value: Box<Expr<'a>>
}

#[derive(Debug)]
pub(crate) struct Logical<'a> {
    pub token: &'a Token<'a>,
    pub operator: LogicalOperator,
    pub left: Box<Expr<'a>>,
    pub right: Box<Expr<'a>>,
}

pub(crate) enum Statement<'a> {
    ExpressionStatement(ExpressionStatement<'a>),
    PrintStatement(PrintStatement<'a>),
    VarDeclStatement(VarDeclStatement<'a>),
    BlockStatement(BlockStatement<'a>),
    IfStatement(IfStatement<'a>),
    WhileStatement(WhileStatement<'a>),
}

pub(crate) struct ExpressionStatement<'a> {
    pub expression: Expr<'a>
}

pub(crate) struct PrintStatement<'a> {
    pub token: &'a Token<'a>,
    pub value: Expr<'a>
}

pub(crate) struct VarDeclStatement<'a> {
    pub token: &'a Token<'a>,
    pub initializer: Option<Expr<'a>>
}

pub(crate) struct BlockStatement<'a> {
    pub statements: Vec<Statement<'a>>
}

pub(crate) struct IfStatement<'a> {
    pub condition: Expr<'a>,
    pub then_branch: Box<Statement<'a>>,
    pub else_branch: Option<Box<Statement<'a>>>
}

pub(crate) struct WhileStatement<'a> {
    pub condition: Expr<'a>,
    pub body: Box<Statement<'a>>
}