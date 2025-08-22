use logos::Logos;


#[derive(Logos, Debug, Clone, PartialEq)]
pub enum Token {
    #[regex(r"[\t\n\f]+", logos::skip)]
    Ignore,
    

    #[regex("//[^\n]*", logos::skip)]
    Comment,

    // RESERVERD WORDS
    #[token("armazena_robozinho")]
    Let,

    #[token("se_encontrou")]
    If,

    #[token("alarmobo_destruido")]
    Else,


    // VARIABLE NAME
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", |lex| Some(lex.slice().to_string()))]
    Indentifier(String),


    // TYPES
    #[regex(r#""([^"\\]|\\.)*""#, |lex| {
        let slice: &'s str = lex.slice();
        Some(slice[1..slice.len() - 1].to_string())
    })]
    Str(String),

    #[regex("-?[0-9]+", |lex| lex.slice().parse().ok())]
    Number(i64),

    #[regex("-?[0-9]+\\.[0-9]+", |lex| lex.slice().parse().ok())]
    Float(f64),

    #[regex("(true|false)", |lex| lex.slice().parse().ok())]
    Boolean(bool),

    // FUNCTION
    #[token("mostra_na_tela_robozinho")]
    Print,


    // SCOPE
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    
    // SYMBOLS
    #[token("=")]
    Equal,
    #[token("+")]
    Plus,
    #[token("-")]
    Mius,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token(";")]
    Semicolon,
    #[token("==")]
    TwoEqual,
}
