use crate::token_type::TokenType;

#[path = "./token_type.rs"]
mod token_type;

#[derive(Debug)]
#[derive(Clone)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    line: i32,
    // literal: Object
}

impl Token{
    pub fn to_string(&self) -> String {
        return format!("{:?} {} {}", self.token_type, self.lexeme, self.line);
    }

    pub fn new(token_type: TokenType, lexeme:String, line:i32) -> Token {
        return Token{token_type, lexeme, line};
    }
}
