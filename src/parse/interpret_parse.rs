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
        match self.next < self.tokens.len() {
            true => {
                self.tokens.get(self.next)
            },
            false => None,
        }
    }

    
    fn next_ind_token(&mut self) -> Option<Token> {
        self.next += 1;
        match self.next < self.tokens.len() {
            true => {  
                
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
                        let expr_result: Option<Expr> = self.parse_term();
                        
                        match expr_result {
                            Some(expr) => {
                                println!("match expr {:?}", self.peek_token());
                                if let Some(Token::Semicolon) = self.peek_token() {
                                    println!("match expr {:?}", expr);
                                    self.next_ind_token();
                                    return Some(Stmt::Let(variable_name, expr));
                                } else {
                                    let message_not_found_semicolon: &str = "[Error] - Invalid Sintaxe! Please add ; in line end.";
                                    println!("{}", message_not_found_semicolon);
                                }
                            },
                            None => {
                                let message_not_found_expr: &str = "[Error] - INvalid Sintaxe!";
                                println!("{}", message_not_found_expr);
                            },
                        };
                        
                    } else {
                        let message_not_found_equal: String = format!("[Error]- Invalid Sintaxe! Please add = in variable: {:?}!", variable_name); 
                        println!("{:?}", &message_not_found_equal);
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
            
            let signal_operation = match ind_t {
                Token::Plus => "+",
                Token::Mius => "-",
                _ => break,
            };
            
            if let right_token = self.parse_term() {
                println!("right {:?}", right_token);
                match right_token {
                    /*
                    * Expr
                    * */
                    Some(right) => {
                        expr = Expr::Binary { left: Box::new(expr), operation: signal_operation.to_string(), right: Box::new(right) };
                        break;
                    },
                    None => return None,
                }
                
            } else { break; }
        }

        println!("token_now 2 {:?}", &token_now);
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
