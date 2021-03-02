use std::collections::HashMap;

use crate::{ast::{Assignment, Binary, BlockStatement, Call, Expr, ExpressionStatement, FunDeclStatement, Grouping, IfStatement, Logical, PrintStatement, ReturnStatement, Statement, Unary, VarDeclStatement, Variable, WhileStatement}, error::LoxError, tokens::Token, tree_walker::TreeWalker};



pub struct Resolver<'i>{
    // The value associated with a key in the scope map represents
    //  whether or not we have finished resolving that variable’s initializer.
    scopes: Vec<HashMap<String, bool>>,
    pub(crate) errors: Vec<LoxError>,
    interpreter: &'i TreeWalker
}

impl<'i> Resolver<'i> {
    pub(crate) fn new(interpreter: &'i TreeWalker) -> Resolver<'i> {
        Resolver {scopes: Vec::new(), errors: Vec::new(), interpreter }
    }

    pub(crate) fn resolve(&mut self, statements: &Vec<Statement>) {
        for stmt in statements {
            self.resolve_statement(stmt);
        }
    }

    // helpers

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn resolve_statement(&mut self, statement: &Statement) {
        match statement {
            Statement::ExpressionStatement(stmt) => { self.visit_expression_statement(stmt)}
            Statement::PrintStatement(stmt) => { self.visit_print_statement(stmt) }
            Statement::VarDeclStatement(stmt) => { self.visit_var_decl_statement(stmt) }
            Statement::BlockStatement(stmt) => { self.visit_block_statement(stmt) }
            Statement::IfStatement(stmt) => { self.visit_if_statement(stmt) }
            Statement::WhileStatement(stmt) => { self.visit_while_statement(stmt) }
            Statement::FunDeclStatement(stmt) => { self.visit_fun_decl_statement(stmt) }
            Statement::ReturnStatement(stmt) => { self.visit_return_statement(stmt) }
        }
    }

    fn resolve_expression(&mut self, expression: &Expr) {
        match expression {
            Expr::Binary(b) => { self.visit_binary(b) }
            Expr::Unary(u) => { self.visit_unary(u) }
            Expr::Literal(_) => { /* nothing to resolve */ }
            Expr::Grouping(g) => { self.visit_grouping(g)}
            Expr::Variable(v) => { self.visit_variable(v) }
            Expr::Assignment(a) => { self.visit_assignment(a) }
            Expr::Logical(l) => { self.visit_logical(l) }
            Expr::Call(c) => { self.visit_call(c) }
        }
    }

    fn declare(&mut self, name: &String) {
        if let Some(scope) = self.scopes.last_mut()  {
            scope.insert(name.to_owned(), false);
        }
    }

    fn define(&mut self, name: &String) {
        if let Some(scope) = self.scopes.last_mut()  {
            scope.insert(name.to_owned(), true);
        }
    }

    fn resolve_local(&self, token: &Token) {
        for scope in self.scopes.iter().rev() {
            if scope.contains_key(&token.lexeme) {
                // resolve
                todo!();
                break;
            }
        }
    }

    fn resolve_function(&mut self, stmt: &FunDeclStatement) {
        self.begin_scope();
        for param in &stmt.parameters {
            self.declare(&param.lexeme);
            self.define(&param.lexeme);
        }
        self.visit_block_statement(&stmt.body);
    }

    // AST nodes that need resolving

    fn visit_block_statement(&mut self, block: &BlockStatement) {
        self.begin_scope();
        for statement in &block.statements {
            self.resolve_statement(statement);
        }
        self.end_scope();
    }

    fn visit_var_decl_statement(&mut self, stmt: &VarDeclStatement) {
        self.declare(&stmt.token.lexeme);
        match &stmt.initializer {
            Some(init) => { self.resolve_expression(init) }
            None => {}
        };
        self.define(&stmt.token.lexeme);
    }

    fn visit_variable(&mut self, expr: &Variable) {
        if let Some(scope) = self.scopes.last()  {
            if let Some(finished_resolving) = scope.get(&expr.token.lexeme) {
                if !finished_resolving {
                    self.errors.push(LoxError {kind: crate::error::LoxErrorKind::ResolvingError,
                         message: "Can't use local variable in its own intializer"});
                }
            }
        }
        self.resolve_local(&expr.token);
    }

    fn visit_assignment(&mut self, expr: &Assignment) {
        self.resolve_expression(expr.value.as_ref());
        self.resolve_local(&expr.token);
    }

    fn visit_fun_decl_statement(&mut self, stmt: &FunDeclStatement) {
        self.declare(&stmt.name.lexeme);
        self.define(&stmt.name.lexeme);
        self.resolve_function(stmt);
    }

    // basically just resolve child AST nodes

    fn visit_expression_statement(&mut self, stmt: &ExpressionStatement) {
        self.resolve_expression(&stmt.expression);
    }

    fn visit_if_statement(&mut self, stmt: &IfStatement) {
        self.resolve_expression(&stmt.condition);
        self.resolve_statement(stmt.then_branch.as_ref());
        if let Some(branch) = &stmt.else_branch {
            self.resolve_statement(branch.as_ref());
        }
    }

    fn visit_print_statement(&mut self, stmt: &PrintStatement) {
        self.resolve_expression(&stmt.value);
    }

    fn visit_return_statement(&mut self, stmt: &ReturnStatement) {
        if let Some(expr) = &stmt.value {
            self.resolve_expression(expr);
        }
    }

    fn visit_while_statement(&mut self, stmt: &WhileStatement) {
        self.resolve_expression(&stmt.condition);
        self.resolve_statement(stmt.body.as_ref());
    }

    fn visit_binary(&mut self, expr: &Binary) {
        self.resolve_expression(expr.left.as_ref());
        self.resolve_expression(expr.right.as_ref());
    }

    fn visit_call(&mut self, expr: &Call) {
        self.resolve_expression(expr.callee.as_ref());
        for argument in &expr.arguments {
            self.resolve_expression(&argument);
        }
    }

    fn visit_grouping(&mut self, expr: &Grouping) {
        self.resolve_expression(expr.expr.as_ref());
    }

    fn visit_logical(&mut self, expr: &Logical) {
        self.resolve_expression(expr.left.as_ref());
        self.resolve_expression(expr.right.as_ref());
    }

    fn visit_unary(&mut self, expr: &Unary) {
        self.resolve_expression(expr.right.as_ref());
    }

    
}