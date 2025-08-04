use logos::Logos;

use crate::Token;

use std::fs;


pub struct Input {
    filename: String,
}

impl Input {
    pub fn new(filename: String) -> Self {
        Self {
            filename,
        }
    }

    pub fn read_input_user(&mut self) -> Result<Vec<Token>, ()> {
        let message_waring_not_found_file: String = format!("File not found. FILE: {}", self.filename);
        let content: String = fs::read_to_string(&self.filename)
            .expect(&message_waring_not_found_file);

        if content.trim().is_empty() {
            return Err(());
        }

        let lexer: Vec<Token> = Token::lexer(&content)
            .filter_map(Result::ok)
            .collect::<Vec<_>>();
    
        Ok(lexer)
    }
}
