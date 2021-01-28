use crate::{ast::{Binary, BinaryOperator, Expr, Literal, Unary, Variable}, error::{LoxError, LoxErrorKind}, tokens::LiteralValue};


pub(crate) struct TreeWalker {

}

#[derive(Debug)]
pub(crate) enum Value {
    NumberValue(f64),
    StringValue(String),
    BooleanValue(bool),
    NilValue
}

impl TreeWalker {
    pub fn visit_expr(&self, expr: &Expr) -> Result<Value, LoxError> {
        match expr {
            Expr::Binary(e) => {
                self.visit_binary(e)
            }
            Expr::Unary(e) => {
                self.visit_unary(e)
            }
            Expr::Literal(e) => {
                self.visit_literal(e)
            }
            Expr::Variable(e) => {
                self.visit_variable(e)
            }
        }
    }

    fn visit_binary(&self, expr: &Binary) -> Result<Value, LoxError> {
        let left = self.visit_expr(expr.left.as_ref())?;
        let right = self.visit_expr(expr.right.as_ref())?;
        match expr.operator {
            BinaryOperator::BangEqual => todo!(),
            BinaryOperator::EqualEqual => todo!(),
            BinaryOperator::Greater => todo!(),
            BinaryOperator::GreaterEqual => todo!(),
            BinaryOperator::Less => todo!(),
            BinaryOperator::LessEqual => todo!(),
            BinaryOperator::Minus => todo!(),
            BinaryOperator::Plus => {
                match (left, right) {
                    (Value::NumberValue(l), Value::NumberValue(r)) => {
                        Ok(Value::NumberValue(l+r))
                    },
                    _ => Err(LoxError {kind: LoxErrorKind::TypeError, message: "unsupported operand types"})
                }
            },
            BinaryOperator::Slash => todo!(),
            BinaryOperator::Star => todo!(),
        }
    }

    fn visit_unary(&self, expr: &Unary) -> Result<Value, LoxError> {
        todo!()
    }

    fn visit_literal(&self, expr: &Literal) -> Result<Value, LoxError> {
        match &expr.value {
            LiteralValue::NumberValue(n) => Ok(Value::NumberValue(n.to_owned())),
            LiteralValue::StringValue(s) => Ok(Value::StringValue(s.to_owned())),
            LiteralValue::BooleanValue(b) => Ok(Value::BooleanValue(b.to_owned())),
            LiteralValue::NilValue => Ok(Value::NilValue)
        }
    }

    fn visit_variable(&self, expr: &Variable) -> Result<Value, LoxError> {
        todo!()
    }
}