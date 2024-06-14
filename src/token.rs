use crate::token_type::TokenType;

#[derive(Clone, Debug)]
pub enum Object {
    None,
    String,
    Number,
}

#[derive(Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    line: i32,
    literal: Object,
}

impl Token {
    pub fn to_string(&self) -> String {
        return format!("{:?} {} {}", self.token_type, self.lexeme, self.line);
    }

    pub fn print(&self) -> String {
        format!("{}", self.lexeme)
    }

    pub fn new(token_type: TokenType, lexeme: String, line: i32, object: Object) -> Token {
        return Token {
            token_type,
            lexeme,
            line,
            literal: object,
        };
    }
}
