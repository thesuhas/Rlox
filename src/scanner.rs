use crate::token::Object;
use crate::token::Token;
use crate::token_type::TokenType;
use crate::Rlox;
use ::phf::{phf_map, Map};

static KEYWORDS: Map<&'static str, TokenType> = phf_map! {
    "and" => TokenType::And,
    "class" => TokenType::Class,
    "else" => TokenType::Else,
    "false" => TokenType::False,
    "for" => TokenType::For,
    "fun" => TokenType::Fun,
    "if" => TokenType::If,
    "nil" => TokenType::Nil,
    "or" => TokenType::Or,
    "print" => TokenType::Print,
    "return" => TokenType::Return,
    "super" => TokenType::Super,
    "this" => TokenType::This,
    "true" => TokenType::True,
    "var" => TokenType::Var,
    "while" => TokenType::While,
};

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
        Scanner {
            source,
            tokens: empty,
            current: 0,
            start: 0,
            line: 1,
            rlox,
        }
    }

    fn is_at_end(&self) -> bool {
        return self.current >= self.source.len();
    }

    fn advance(&mut self) -> char {
        let char = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        return char;
    }

    fn add_token(&mut self, token_type: TokenType) {
        let text: String = self.source[self.start..self.current].to_string();
        self.tokens
            .push(Token::new(token_type, text, self.line, Object::Nil));
    }

    fn add_token_value(&mut self, token_type: TokenType, literal: Object) {
        let text: String = self.source[self.start..self.current].to_string();
        self.tokens
            .push(Token::new(token_type, text, self.line, literal));
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

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        return self.source.chars().nth(self.current + 1).unwrap();
    }

    fn string(&mut self) {
        // Iterate till you go to the end of the string and it has not ended
        while self.peek() != '"' && !self.is_at_end() {
            // As multi-line comments are allowed, increment the line
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        // If the string was not terminated
        if self.is_at_end() {
            self.rlox.error(self.line, "Unterminated String!");
            return;
        }

        // Now if the loop exited and is not at the end, that means you are at "
        self.advance(); // Consume the "

        // Get the string value
        self.add_token_value(TokenType::String, Object::String);
    }

    fn is_digit(&self, c: char) -> bool {
        return c >= '0' && c <= '9';
    }

    fn is_alpha(&self, c: char) -> bool {
        return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_';
    }

    fn is_alpha_numeric(&self, c: char) -> bool {
        return self.is_digit(c) || self.is_alpha(c);
    }

    fn number(&mut self) {
        // While it continues to be a digit, you can advance
        while self.is_digit(self.peek()) {
            self.advance();
        }

        // If it ever becomes fraction
        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            // Consume the full stop
            self.advance();
            // Continue till you get nums
            while self.is_digit(self.peek()) {
                self.advance();
            }
        }

        // Now you get the number
        self.add_token_value(TokenType::Number, Object::Number);
    }

    fn identifier(&mut self) {
        while self.is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let text = &self.source[self.start..self.current];

        let mut token_type: TokenType = TokenType::Identifier;

        match KEYWORDS.get(text).cloned() {
            Some(tok_type) => token_type = tok_type,
            None => {}
        }

        self.add_token(token_type);
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
            '!' => {
                if self.match_next('=') {
                    self.add_token(TokenType::BangEqual)
                } else {
                    self.add_token(TokenType::Bang)
                }
            }
            '=' => {
                if self.match_next('=') {
                    self.add_token(TokenType::EqualEqual)
                } else {
                    self.add_token(TokenType::Equal)
                }
            }
            '<' => {
                if self.match_next('=') {
                    self.add_token(TokenType::LessEqual)
                } else {
                    self.add_token(TokenType::Less)
                }
            }
            '>' => {
                if self.match_next('=') {
                    self.add_token(TokenType::GreaterEqual)
                } else {
                    self.add_token(TokenType::Greater)
                }
            }
            '/' => {
                if self.match_next('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            ' ' => {}
            '\r' => {}
            '\t' => {}
            '\n' => self.line += 1,
            '"' => self.string(),
            _ => {
                if self.is_digit(c) {
                    self.number();
                } else if self.is_alpha(c) {
                    self.identifier();
                } else {
                    self.rlox.error(self.line, "Unexpected character.");
                }
            }
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(
            TokenType::EOF,
            "".to_string(),
            self.line,
            Object::Nil,
        ));
        return self.tokens.clone();
    }
}
