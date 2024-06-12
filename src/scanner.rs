use crate::Rlox;
use crate::token::Token;
use crate::token_type::TokenType;

#[path = "./token.rs"]
mod token;
#[path = "./token_type.rs"]
mod token_type;

#[derive(Debug)]
 pub struct Scanner<'a> {
    source: String,
    tokens: Vec<Token>,
    current: usize,
    start: usize,
    line: i32,
    rlox: &'a mut Rlox,
}

impl Scanner<'_> {
    pub fn new(source: String, rlox: &mut Rlox) -> Scanner {
        let empty = Vec::new();
        Scanner {source, tokens:empty, current:0, start: 0, line: 1, rlox }
    }

    fn is_at_end(&self) -> bool {
        return self.current >= self.source.len();
    }

    fn advance(&mut self) -> char {
        let char =  self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        return char;
    }

    fn add_token(&mut self, token_type: TokenType) {
        let text: String = self.source[self.start..self.current].to_string();
        self.tokens.push(Token::new(token_type, text, self.line));
    }

    fn add_token_string(&mut self, token_type: TokenType, text: String) {
        self.tokens.push(Token::new(token_type, text, self.line));
    }

    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }
        self.current += 1;
        return true;
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        return self.source.chars().nth(self.current).unwrap();
    }

    fn scan_token(&mut self) {
        let c: char = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::SemiColon),
            '*' => self.add_token(TokenType::Star),
            '!' => if self.match_next('=') {self.add_token(TokenType::BangEqual)} else {self.add_token(TokenType::Bang)},
            '=' => if self.match_next('=') {self.add_token(TokenType::EqualEqual)} else {self.add_token(TokenType::Equal)},
            '<' => if self.match_next('=') {self.add_token(TokenType::LessEqual)} else {self.add_token(TokenType::Less)},
            '>' => if self.match_next('=') {self.add_token(TokenType::GreaterEqual)} else {self.add_token(TokenType::Greater)},
            '/' => {
                if self.match_next('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            ' ' => {},
            '\r' => {},
            '\t' => {},
            '\n' => self.line += 1,
            _ => self.rlox.error(self.line, "Unexpected character."),
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(TokenType::EOF, "".to_string(), self.line));
        return self.tokens.clone();
    }
}