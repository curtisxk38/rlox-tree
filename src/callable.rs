use crate::{ast::FunDeclStatement, error::LoxError, tree_walker::{Environment, TreeWalker, Value}};

#[derive(Debug, Clone)]
pub(crate) struct Function {
    pub declaration: FunDeclStatement
}

impl Function {
    pub fn call(& self, interpreter:  &mut TreeWalker, arguments: Vec<Value>) -> Result<Value, LoxError>{
        let mut env = Environment::new();
        // ASSUMPTION made: arguments.len() = self.declaration.parameters.len()
        for index  in 0..arguments.len() {
            env.define(&self.declaration.parameters.get(index).unwrap().lexeme, arguments.get(index).unwrap().to_owned())
        }

        interpreter.execute_block(&self.declaration.body.statements, env)?;
        // todo add return statement
        Ok(Value::NilValue)
    }

    pub fn arity(&self) -> usize {
        self.declaration.parameters.len()
    }
}