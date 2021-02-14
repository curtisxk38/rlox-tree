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
pub(crate) enum Expr {
    Binary(Binary),
    Unary(Unary),
    Literal(Literal),
    Grouping(Grouping),
    Variable(Variable),
    Assignent(Assignent),
    Logical(Logical),
    Call(Call),
}

#[derive(Debug)]
pub(crate) struct Binary {
    pub token: Token,
    pub operator: BinaryOperator,
    pub left: Box<Expr>,
    pub right: Box<Expr>,
}

#[derive(Debug)]
pub(crate) struct Unary {
    pub token: Token,
    pub operator: UnaryOperator,
    pub right: Box<Expr>
}

#[derive(Debug)]
pub(crate) struct Literal {
    pub token: Token,
    pub value: LiteralValue,
}

#[derive(Debug)]
pub(crate) struct Grouping {
    pub expr: Box<Expr>
}

#[derive(Debug)]
pub(crate) struct Variable {
    pub token: Token
}

#[derive(Debug)]
pub(crate) struct Assignent {
    pub token: Token,
    pub value: Box<Expr>
}

#[derive(Debug)]
pub(crate) struct Logical {
    pub token: Token,
    pub operator: LogicalOperator,
    pub left: Box<Expr>,
    pub right: Box<Expr>,
}

#[derive(Debug)]
pub(crate) struct Call {
    pub callee: Box<Expr>,
    pub arguments: Vec<Expr>,
    pub token: Token, // token for closing ")" after call
}

pub(crate) enum Statement {
    ExpressionStatement(ExpressionStatement),
    PrintStatement(PrintStatement),
    VarDeclStatement(VarDeclStatement),
    BlockStatement(BlockStatement),
    IfStatement(IfStatement),
    WhileStatement(WhileStatement),
    FunDeclStatement(FunDeclStatement),
}

pub(crate) struct ExpressionStatement {
    pub expression: Expr
}

pub(crate) struct PrintStatement {
    pub token: Token,
    pub value: Expr
}

pub(crate) struct VarDeclStatement {
    pub token: Token,
    pub initializer: Option<Expr>
}

pub(crate) struct BlockStatement {
    pub statements: Vec<Statement>
}

pub(crate) struct IfStatement {
    pub condition: Expr,
    pub then_branch: Box<Statement>,
    pub else_branch: Option<Box<Statement>>
}

pub(crate) struct WhileStatement {
    pub condition: Expr,
    pub body: Box<Statement>
}

pub(crate) struct FunDeclStatement {
    pub name: Token,
    pub parameters: Vec<Token>,
    pub body: BlockStatement
}