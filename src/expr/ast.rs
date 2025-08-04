

#[derive(Clone, Debug)]
pub enum Expr {
    Number(i64),
    Str(String),
    Boolean(bool),
    Float(f64),
    OtherVariable(String),
    Binary {
        left: Box<Expr>,
        operation: String,
        right: Box<Expr>,
    },
}

#[derive(Clone, Debug)]
pub enum Value {
    Number(i64),
    Str(String),
    Boolean(bool),
    Float(f64),
}
