use crate::token_type::TokenType;

#[derive(Clone, Debug, Copy)]
pub enum Object {
    // None,
    String,
    Number,
    Bool,
    Nil,
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

    pub fn get_type(&self) -> TokenType {
        self.token_type.clone()
    }

    pub fn get_line(&self) -> i32 {
        self.line
    }

    pub fn get_lexeme(&self) -> String {
        self.lexeme.clone()
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
