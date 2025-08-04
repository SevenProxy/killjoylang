
mod lexer;
mod parse;
mod statement;
mod expr;

// RUNTIME
mod runtime;

// MOD FUNCTIONS
mod functions;

pub use lexer::Token;
pub use parse::{
    Input,
    InterParse,
};
pub use statement::Stmt;
pub use expr::{
    Expr,
    Value,
};

// USE FUNCTIONS
pub use functions::Print;


// RUNTIME
pub use runtime::Eval;
