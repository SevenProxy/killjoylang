use crate::Expr;

#[derive(Clone, Debug)]
pub enum Print {
    Ast(Expr),
}
