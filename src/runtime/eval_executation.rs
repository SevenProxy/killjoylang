use std::collections::HashMap;

use crate::{
    Expr,
    Stmt,
    Print,
    Value,
};

pub struct Eval {
    statements: Vec<Stmt>,
    vars: HashMap<String, Value>,
}

impl Eval {
    pub fn new(stataments: Vec<Stmt>) -> Self {
        Self {
            statements: stataments,
            vars: HashMap::new(),
        }
    }

    pub fn intpretation(&mut self) {
        for stmt in self.statements.clone() {
            match stmt {
                Stmt::Let(variable_name, expr_result) => {
                    let result_value: Value = self.eval_operation(&expr_result);

                    self.vars.insert(variable_name, result_value);
                },
                Stmt::Print(print_expr) => self.eval_print(&print_expr),
            }
        }
    }

    fn eval_operation(&mut self, expr: &Expr) -> Value {
        match expr {
            Expr::Str(type_string) => Value::Str(type_string.clone()),
            Expr::Number(type_number) => Value::Number(*type_number),
            Expr::Float(type_float) => Value::Float(*type_float),
            Expr::Boolean(type_bool) => Value::Boolean(*type_bool),
            Expr::OtherVariable(type_string) => self.vars.get(type_string).cloned().unwrap_or(Value::Number(0)),
            Expr::Binary { left, operation, right } => {
                let l: Value = self.eval_operation(left);
                let r: Value = self.eval_operation(right);
                
                match (l, r, operation.as_str()) {
                    (Value::Number(x), Value::Number(y), "*") => Value::Number(x * y),
                    (Value::Number(x), Value::Number(y), "/") => Value::Number(x / y),
                    (Value::Number(x), Value::Number(y), "+") => Value::Number(x + y),
                    (Value::Number(x), Value::Number(y), "-") => Value::Number(x - y),

                    (Value::Str(x), Value::Str(y), "+") => Value::Str(x + &y),

                    _ => panic!("[Erro] - Inválid Operation!!!"),
                }
            }
        }
    }

    fn eval_print(&mut self, print: &Print) {
        match print {
            Print::Ast(expr) => {
                let result_value: Value = self.eval_operation(expr);
                println!("{:?}", result_value);
            }
            _ => {},
        }
    }
}
