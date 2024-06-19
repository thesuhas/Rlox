use crate::expr::{BinaryExpression, Expr};
use crate::token::{Object, Token};
use crate::token_type::TokenType;
use crate::Rlox;
use std::ops::Deref;

pub struct Parser<'a> {
    tokens: Vec<Token>,
    current: usize,
    rlox: &'a mut Rlox,
}

impl Parser<'_> {
    pub fn new(tokens: Vec<Token>, rlox: &mut Rlox) -> Parser {
        Parser {
            tokens,
            current: 0,
            rlox,
        }
    }

    pub fn parse(&mut self) -> Expr {
        self.expression()
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr: Expr = self.comparison();
        while self.match_token(&[&TokenType::BangEqual, &TokenType::EqualEqual]) {
            // Take the previous token as we have already matched with the token
            let operator: Token = self.previous();
            let right_expr: Expr = self.comparison();
            expr = Expr::new_binary(expr, operator, right_expr);
        }
        expr
    }

    fn match_token(&mut self, tokens: &[&TokenType]) -> bool {
        for &token_type in tokens {
            if self.check(token_type.clone()) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        let t: TokenType = self.peek().get_type();
        t == token_type
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn comparison(&mut self) -> Expr {
        let mut expr: Expr = self.term();

        while self.match_token(&[
            &TokenType::Greater,
            &TokenType::GreaterEqual,
            &TokenType::LessEqual,
            &TokenType::Less,
        ]) {
            let operator: Token = self.previous();
            let right: Expr = self.term();
            expr = Expr::new_binary(expr, operator, right);
        }
        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr: Expr = self.factor();

        while self.match_token(&[&TokenType::Minus, &TokenType::Plus]) {
            let operator: Token = self.previous();
            let right: Expr = self.factor();
            expr = Expr::new_binary(expr, operator, right);
        }
        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr: Expr = self.unary();

        while self.match_token(&[&TokenType::Slash, &TokenType::Star]) {
            let operator: Token = self.previous();
            let right: Expr = self.unary();
            expr = Expr::new_binary(expr, operator, right);
        }
        expr
    }

    fn unary(&mut self) -> Expr {
        if self.match_token(&[&TokenType::Bang, &TokenType::Minus]) {
            let operator: Token = self.previous();
            let right: Expr = self.unary();
            return Expr::new_unary(operator, right);
        }
        self.primary().unwrap()
    }

    fn primary(&mut self) -> Option<Expr> {
        if self.match_token(&[&TokenType::False]) {
            return Some(Expr::new_literal(Object::Bool, self.previous()));
        } else if self.match_token(&[&TokenType::True]) {
            return Some(Expr::new_literal(Object::Bool, self.previous()));
        } else if self.match_token(&[&TokenType::Nil]) {
            return Some(Expr::new_literal(Object::Nil, self.previous()));
        } else if self.match_token(&[&TokenType::Number]) {
            return Some(Expr::new_literal(Object::Number, self.previous()));
        } else if self.match_token(&[&TokenType::String]) {
            return Some(Expr::new_literal(Object::String, self.previous()));
        } else if self.match_token(&[&TokenType::LeftParen]) {
            // Consume the enclosing expression
            let expr: Expr = self.expression();
            if self.consume(
                TokenType::RightParen,
                "Expected ')' after expression".to_string(),
            ) {
                return Some(Expr::new_grouping(expr));
            }
            return None;
        }
        self.error(self.peek(), "Expect expression.".to_string());
        None
    }

    fn synchronise(&mut self) {
        self.advance();
        // Discard everything till the current line is done
        while !self.is_at_end() {
            if self.previous().get_type() == TokenType::SemiColon {
                return;
            }

            match self.peek().get_type() {
                TokenType::Return => return,
                _ => {}
            }
            // Go to the next character
            self.advance();
        }
    }

    fn consume(&mut self, token_type: TokenType, message: String) -> bool {
        if self.check(token_type) {
            // Go to the next token
            self.advance();
            return true;
        }
        // Throw error
        self.rlox.parse_error(self.peek(), message);
        return false;
    }

    fn error(&mut self, token: Token, message: String) -> Option<bool> {
        self.rlox.parse_error(token, message);
        None
    }

    fn is_at_end(&self) -> bool {
        self.peek().get_type() == TokenType::EOF
    }

    fn peek(&self) -> Token {
        self.tokens
            .get(self.current)
            .expect("Out of bounds")
            .clone()
    }

    fn previous(&self) -> Token {
        self.tokens
            .get(self.current - 1)
            .expect("Out of bounds")
            .clone()
    }
}
