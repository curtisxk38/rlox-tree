use crate::tokens::{LiteralValue, Token};


#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub(crate) enum LogicalOperator {
    And,
    Or
}

#[derive(Debug, Clone)]
pub(crate) enum UnaryOperator {
    Bang,
    Minus,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub(crate) struct Binary {
    pub token: Token,
    pub operator: BinaryOperator,
    pub left: Box<Expr>,
    pub right: Box<Expr>,
}

#[derive(Debug, Clone)]
pub(crate) struct Unary {
    pub token: Token,
    pub operator: UnaryOperator,
    pub right: Box<Expr>
}

#[derive(Debug, Clone)]
pub(crate) struct Literal {
    pub token: Token,
    pub value: LiteralValue,
}

#[derive(Debug, Clone)]
pub(crate) struct Grouping {
    pub expr: Box<Expr>
}

#[derive(Debug, Clone)]
pub(crate) struct Variable {
    pub token: Token
}

#[derive(Debug, Clone)]
pub(crate) struct Assignent {
    pub token: Token,
    pub value: Box<Expr>
}

#[derive(Debug, Clone)]
pub(crate) struct Logical {
    pub token: Token,
    pub operator: LogicalOperator,
    pub left: Box<Expr>,
    pub right: Box<Expr>,
}

#[derive(Debug, Clone)]
pub(crate) struct Call {
    pub callee: Box<Expr>,
    pub arguments: Vec<Expr>,
    pub token: Token, // token for closing ")" after call
}

#[derive(Debug, Clone)]
pub(crate) enum Statement {
    ExpressionStatement(ExpressionStatement),
    PrintStatement(PrintStatement),
    VarDeclStatement(VarDeclStatement),
    BlockStatement(BlockStatement),
    IfStatement(IfStatement),
    WhileStatement(WhileStatement),
    FunDeclStatement(FunDeclStatement),
    ReturnStatement(ReturnStatement),
}

#[derive(Debug, Clone)]
pub(crate) struct ExpressionStatement {
    pub expression: Expr
}

#[derive(Debug, Clone)]
pub(crate) struct PrintStatement {
    pub token: Token,
    pub value: Expr
}

#[derive(Debug, Clone)]
pub(crate) struct VarDeclStatement {
    pub token: Token,
    pub initializer: Option<Expr>
}

#[derive(Debug, Clone)]
pub(crate) struct BlockStatement {
    pub statements: Vec<Statement>
}

#[derive(Debug, Clone)]
pub(crate) struct IfStatement {
    pub condition: Expr,
    pub then_branch: Box<Statement>,
    pub else_branch: Option<Box<Statement>>
}

#[derive(Debug, Clone)]
pub(crate) struct WhileStatement {
    pub condition: Expr,
    pub body: Box<Statement>
}

#[derive(Debug, Clone)]
pub(crate) struct FunDeclStatement {
    pub name: Token,
    pub parameters: Vec<Token>,
    pub body: BlockStatement
}

#[derive(Debug, Clone)]
pub(crate) struct ReturnStatement {
    pub keyword: Token,
    pub value: Option<Expr>,
}