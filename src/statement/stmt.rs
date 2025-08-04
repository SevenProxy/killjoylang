use crate::{
    Expr,
    Print,
};

#[derive(Clone, Debug)]
pub enum Stmt {
    Let(String, Expr),
    Print(Print),
}

