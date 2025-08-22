use crate::{
    Expr,
    Print,
};

#[derive(Clone, Debug)]
pub enum Stmt {
    Let(String, Expr),
    Print(Print),
    If {
        condition: Expr,
        then_branch: Vec<Stmt>,
        else_branch: Option<Vec<Stmt>>,
    },
}

