use crate::{
    Expr,
    Stmt,
    Token,
};


pub struct InterParse {
    tokens: Vec<Token>,
    next: usize,
    line: u32,
}

impl InterParse {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            next: 0,
            line: 0,
        }
    }

    fn peek_token(&self) -> Option<&Token> {
        self.tokens.get(self.next)
    }

    
    fn next_ind_token(&mut self) -> Option<Token> {
        match self.next < self.tokens.len() {
            true => {
                self.next += 1;
                
                let result_token: Token = self.tokens[self.next].clone();
                Some(result_token)
            },
            false => None,
        } 
    }

    pub fn parse_all(&mut self) -> Vec<Stmt> {
        let mut stmt_return: Vec<Stmt> = Vec::new();
        
        while self.peek_token().is_some() {
            if let Some(ind_t) = self.parse_stmt() {
                stmt_return.push(ind_t);
            } else {
                let message_err: &str = "[Error] - Interpretation Falied.";
                println!("{}", message_err);

                let message_err_line: String = format!("Line: {:?}", self.line);
                println!("{}", message_err_line);
            
                if let Some(line_token_err) = self.peek_token() {
                    let message_sintaxe_err_token: String = format!("Unexpected token {:?}", line_token_err);
                    println!("{}", message_sintaxe_err_token);
                } else {
                    let message_not_found_sintaxe_err_token: &str = "Unexpected end of input.";
                    println!("{}", message_not_found_sintaxe_err_token);
                }
                
                break;
            }
        }

        stmt_return
    }

    fn parse_stmt(&mut self) -> Option<Stmt> {
        match self.peek_token()? {
            Token::Let => {
                
                if let Some(Token::Indentifier(variable_name)) = self.next_ind_token() {
                    if let Some(Token::Equal) = self.next_ind_token() {
                        let expr_result: Expr = self.parse_term()?;
                                            
                        if let Some(Token::Semicolon) = self.next_ind_token() {
                            return Some(Stmt::Let(variable_name, expr_result));
                        } else {
                            let message_not_found_semicolon: &str = "[Error] - Invalid Sintaxe! Please add ; in line end.";
                            println!("{}", message_not_found_semicolon);
                        }
                    }
                }
            },
            _ => {},
        }
        None
    }

    fn parse_term(&mut self) -> Option<Expr> {
        let token_now: Option<Token> = self.next_ind_token();
        if token_now.is_none() {
            return None;
        }

        let token_ref: &Token = token_now.as_ref().unwrap();
        let mut expr: Expr = self.parse_token_for_expr(token_ref)?;
     
        /*
        * ind_t: &Token
        * */
        while let Some(ind_t) = self.next_ind_token() {
            println!("{:?}", ind_t);
            let signal_operation = match ind_t {
                Token::Plus => "+",
                Token::Mius => "-",
                _ => break,
            };
            println!("first {}", &signal_operation);
            self.next_ind_token();
            println!("{}", &signal_operation);

            if let Some(right) = self.peek_token() {
                println!("{:?}", &right);
            }

        }

        
        Some(expr)
    }

    fn parse_token_for_expr(&mut self, token: &Token) -> Option<Expr> {
        match token {
            Token::Number(type_number) => Some(Expr::Number(type_number.clone())),
            Token::Str(type_string) => Some(Expr::Str(type_string.clone())),
            Token::Boolean(type_boolean) => Some(Expr::Boolean(type_boolean.clone())),
            _ => None,
        }
    }
}
