use std::collections::HashMap;

use crate::{ast::{Assignment, Binary, BlockStatement, Call, ClassDeclStatement, Expr, ExpressionStatement, FunDeclStatement, Get, Grouping, IfStatement, Logical, PrintStatement, ReturnStatement, Set, Statement, This, Unary, VarDeclStatement, Variable, WhileStatement}, error::LoxError, tokens::Token, tree_walker::TreeWalker};

#[derive(Clone)]
enum FunctionType {
    None,
    Function,
    Method,
}

#[derive(Clone)]
enum ClassType {
    None,
    Class,
}

pub struct Resolver<'i>{
    // The value associated with a key in the scope map represents
    //  whether or not we have finished resolving that variable’s initializer.
    scopes: Vec<HashMap<String, bool>>,
    pub(crate) errors: Vec<LoxError>,
    interpreter: &'i mut TreeWalker,
    current_function: FunctionType,
    current_class: ClassType,
}

impl<'i> Resolver<'i> {
    pub(crate) fn new(interpreter: &'i mut TreeWalker) -> Resolver<'i> {
        Resolver {scopes: Vec::new(), errors: Vec::new(), interpreter, current_function: FunctionType::None, current_class: ClassType::None }
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
            Statement::ClassDeclStatement(stmt) => { self.visit_class_decl_statement(stmt) }
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
            Expr::Get(g) => { self.visit_get(g) }
            Expr::Set(s) => { self.visit_set(s) }
            Expr::This(t) => { self.visit_this(t) }
        }
    }

    fn declare(&mut self, name: &String) {
        if let Some(scope) = self.scopes.last_mut()  {
            if scope.contains_key(name) {
                self.errors.push(LoxError {kind: crate::error::LoxErrorKind::ResolvingError,
                    message: "Variable with this name already exists in this scope"});
            }
            scope.insert(name.to_owned(), false);
        }
    }

    fn define(&mut self, name: &String) {
        if let Some(scope) = self.scopes.last_mut()  {
            scope.insert(name.to_owned(), true);
        }
    }

    fn resolve_local(&mut self, token: &Token) {
        for (index, scope) in self.scopes.iter().rev().enumerate() {
            if scope.contains_key(&token.lexeme) {
                self.interpreter.resolve(token, index);
                break;
            }
        }
    }

    fn resolve_function(&mut self, stmt: &FunDeclStatement, fun_type: FunctionType) {
        let enclosing_function = self.current_function.clone();
        self.current_function = fun_type;

        self.begin_scope();
        for param in &stmt.parameters {
            self.declare(&param.lexeme);
            self.define(&param.lexeme);
        }
        for stmt in &stmt.body {
            self.resolve_statement(stmt);
        }
        self.end_scope();
        self.current_function = enclosing_function;
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
        self.resolve_function(stmt, FunctionType::Function);
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
        match self.current_function {
            FunctionType::None => {
                self.errors.push(LoxError {kind: crate::error::LoxErrorKind::ResolvingError,
                    message: "Can't have a return statement in top level code"});
            },
            _ => {}
        }

        if let Some(expr) = &stmt.value {
            self.resolve_expression(expr);
        }
    }

    fn visit_while_statement(&mut self, stmt: &WhileStatement) {
        self.resolve_expression(&stmt.condition);
        self.resolve_statement(stmt.body.as_ref());
    }

    fn visit_class_decl_statement(&mut self, stmt: &ClassDeclStatement) {
        let enclosing_class_type = self.current_class.clone();
        self.current_class = ClassType::Class;

        self.declare(&stmt.name.lexeme);
        self.define(&stmt.name.lexeme);
        
        self.begin_scope();
        self.scopes.last_mut().unwrap().insert(String::from("this"), true); // we just called begin_scope, so unwrap won't ever panic

        for method in &stmt.methods {
            self.resolve_function(method, FunctionType::Method);
        }

        self.end_scope();
        self.current_class = enclosing_class_type;
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

    fn visit_get(&mut self, expr: &Get) {
        self.resolve_expression(expr.object.as_ref());
    }

    fn visit_set(&mut self, expr: &Set) {
        self.resolve_expression(expr.value.as_ref());
        self.resolve_expression(expr.object.as_ref());
    }

    fn visit_this(&mut self, expr: &This) {
        match &self.current_class {
            ClassType::Class => {
                self.resolve_local(&expr.keyword)
            },
            ClassType::None => {
                self.errors.push(LoxError {kind: crate::error::LoxErrorKind::ResolvingError,
                    message: "Can't use this keyword outside of a class"});
            }
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