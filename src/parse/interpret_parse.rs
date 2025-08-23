use std::clone;

use crate::{
    Expr, Print, Stmt, Token
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
                                if let Some(Token::Semicolon) = self.peek_token() {                      
                                    self.next_ind_token();
                                    return Some(Stmt::Let(variable_name, expr));
                                } else {
                                    let message_not_found_semicolon: &str = "[Error] - Invalid Sintaxe! Please add ; in line end.";
                                    println!("{}", message_not_found_semicolon);
                                }
                            },
                            None => {
                                let message_not_found_expr: &str = "[Error] - Invalid Sintaxe!";
                                println!("{}", message_not_found_expr);
                            },
                        };
                        
                    } else {
                        let message_not_found_equal: String = format!("[Error]- Invalid Sintaxe! Please add = in variable: {:?}!", variable_name); 
                        println!("{:?}", &message_not_found_equal);
                    }
                }
            },
            Token::Print => {
                let token_result: Option<Token> = self.next_ind_token();

                match token_result {
                    /*
                    * token: Token
                    * */
                    Some(token) => {
                        let expr_result: Option<Expr> = self.parse_token_for_expr(&token);

                        match expr_result {
                            /*
                            * expr: Expr
                            * */
                            Some(expr) => {
                                let print_function: Print = Print::Ast(expr);

                                if let Some(Token::Semicolon) = self.next_ind_token() {
                                    self.next_ind_token();
                                    return Some(Stmt::Print(print_function));
                                } else {
                                    let message_not_found_semicolon: &str = "[Error] - Invalid Sintaxe! Please add ; in line end.";
                                    println!("{}", message_not_found_semicolon);
                                }
                            },
                            None => {
                                let message_not_found_expr: &str = "[Error] - Argument invalid for function 'mostra_na_tela_robozinho'.";
                                println!("{}", message_not_found_expr);
                            },
                        };
                    },
                    None => {},
                }

            },
            Token::If => {
                let l_paren: Option<Token> = self.next_ind_token();

                match l_paren {
                    Some(_token) => {
                        let condition_result: Expr = self.parse_term()?;

                        match condition_result {
                            Expr::Boolean(_) => {},
                            _ => {
                                let message_condition_invalid: &str = "[Error] - Invalid Condition.";
                                println!("{}", message_condition_invalid);
                                return None
                            },
                        }
                        
                        if let Some(Token::RParen) = self.peek_token() {
                            if let Some(Token::LBrace) = self.next_ind_token() {
                                let then_branch: Vec<Stmt> = self.parse_block()?;


                                match self.peek_token() == Some(&Token::RBrace) {
                                    true => {
                                        let else_branch = if self.next_ind_token() == Some(Token::Else) {
                                            if self.next_ind_token() != Some(Token::LBrace) {
                                                let messege_not_found_lbrace: &str = "[Error] - Invalid Sintaxe. Please add '{' in line end.";
                                                println!("{}", messege_not_found_lbrace);
                                                return None;
                                            }

                                            let else_result: Vec<Stmt> = self.parse_block()?;
                                            
                                            if self.peek_token() != Some(&Token::RBrace) {
                                                let message_not_found_rbrace: &str = "[Error] - Invalid Sintaxe. Please add '}' in line end.";
                                                println!("{}", message_not_found_rbrace);
                                                return None;
                                            }
                                            Some(else_result)

                                        } else {
                                            None
                                        };

                                        return Some(Stmt::If {
                                            condition: condition_result,
                                            then_branch: then_branch,
                                            else_branch: else_branch,
                                        });
                                    },
                                    false => {
                                        let message_not_found_rbrace: &str = "[Error] - Invalid Sintaxe. Please add '}' in line end.";
                                        println!("{}", message_not_found_rbrace);
                                    }
                                }
                            } else {
                                let message_not_found_brance: &str = "[Error] - Invalid Sintaxe!";
                                println!("{}", message_not_found_brance);
                            }
                        } else {
                            let message_not_found_expr: &str = "[Error] - Invalid Sintaxe! Please add ) in line end.";
                            println!("{}", message_not_found_expr);
                        }
                        
                    },
                    None => {
                        let message_invalid_sintaxe: &str = "[Error] - Invalid Sintaxe!";
                        println!("{}", message_invalid_sintaxe);
                    },
                }
            },
            _ => {},
        }
        None
    }

    fn parse_block(&mut self) -> Option<Vec<Stmt>> {
        let mut stmts = Vec::new();
        
        while let Some(tok) = self.peek_token() {
            if *tok == Token::RBrace {
                break;
            }
            self.next_ind_token();
            stmts.push(self.parse_stmt()?);
        }

        Some(stmts)
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
            
            let signal_operation: &str = match ind_t {
                Token::Plus => "+",
                Token::Mius => "-",
                Token::Star => "*",
                Token::Slash => "/",
                Token::TwoEqual => "==",
                
                _ => break,
            };
            
            
            let right_now: Option<Token> = self.next_ind_token();
            if right_now.is_none() {
                break;
            }
            
            match self.parse_token_for_expr(right_now.as_ref().unwrap()) {
                /*
                * right_token: Expr
                * */
                Some(right_token) => {
                    match signal_operation {
                        "==" => {
                            if right_token == expr {
                                expr = Expr::Boolean(true)
                            } else {
                                expr = Expr::Boolean(false)
                            }
                        },
                        "/" => {
                            match expr {
                                Expr::Binary { left, operation, right } => {
                                    let left_sum: Expr = Expr::Binary {
                                        left: right,
                                        operation: "/".to_string(),
                                        right: Box::new(right_token),
                                    };

                                    expr = Expr::Binary {
                                        left: left,
                                        operation: operation,
                                        right: Box::new(left_sum),
                                    }
                                },
                                Expr::Number(number_value) => {
                                    expr = Expr::Binary {
                                        left: Box::new(Expr::Number(number_value)),
                                        operation: "/".to_string(),
                                        right: Box::new(right_token),
                                    }
                                }
                                _ => break,
                            }
                        },
                        "*" => {
                            match expr {
                                Expr::Binary { left, operation, right} => {
                                    let left_sum: Expr = Expr::Binary {
                                        left: right,
                                        operation: "*".to_string(),
                                        right: Box::new(right_token),
                                    };

                                    expr = Expr::Binary {
                                        left: left,
                                        operation: operation,
                                        right: Box::new(left_sum),
                                    }
                                    
                                },
                                Expr::Number(number_value) => {
                                    expr = Expr::Binary {
                                        left: Box::new(Expr::Number(number_value)),
                                        operation: "*".to_string(),
                                        right: Box::new(right_token),
                                    }
                                },
                                _ => break,
                            }
                        },
                        "+" => {
                            expr = Expr::Binary {
                                left: Box::new(expr),
                                operation: "+".to_string(),
                                right: Box::new(right_token),
                            }
                        },
                        "-" => {
                            expr = Expr::Binary {
                                left: Box::new(expr),
                                operation: "-".to_string(),
                                right: Box::new(right_token),
                            }
                        },
                        _ => break,
                    }
                },
                None => break,
            }
            
        }

        
        Some(expr)
    }


    fn parse_token_for_expr(&mut self, token: &Token) -> Option<Expr> {
        match token {
            Token::Indentifier(variable_name) => Some(Expr::OtherVariable(variable_name.clone())),
            Token::Number(type_number) => Some(Expr::Number(type_number.clone())),
            Token::Str(type_string) => Some(Expr::Str(type_string.clone())),
            Token::Boolean(type_boolean) => Some(Expr::Boolean(type_boolean.clone())),
            Token::LParen => {
                let expr_result: Option<Expr> = self.parse_term();

                match expr_result {
                    /*
                    * expr: Expr
                    * */
                    Some(expr) => {
                        match self.peek_token() {
                            /*
                            * token: &Token
                            * */
                            Some(token) => match token {
                                Token::RParen => Some(expr),
                                _ => Some(expr),
                            },
                            None => Some(expr),
                        } 
                    },
                    None => None,
                }
            },
            _ => None,
        }
    }
}
