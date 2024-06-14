use crate::token::{Object, Token};
use crate::token_type::TokenType;

// #[path = "./token_type.rs"]
// mod token_type;
//
// #[path = "./token.rs"]
// mod token;

// use token_type::TokenType;
// use token::{Token, Object};

pub enum Expr {
    Binary(Box<BinaryExpression>),
    Grouping(Box<GroupingExpression>),
    Literal(Box<LiteralExpression>),
    Unary(Box<UnaryExpression>),
}

pub struct BinaryExpression {
    left: Box<Expr>,
    operator: Token,
    right: Box<Expr>,
}

pub struct GroupingExpression {
    expression: Box<Expr>,
}

pub struct LiteralExpression {
    literal_type: Object,
    value: Token,
}

pub struct UnaryExpression {
    operator: Token,
    right: Box<Expr>,
}

pub fn parenthesize(name: String, exprs: &[&Expr]) -> String {
    let mut out = format!("({}", name);
    for expr in exprs {
        out += " ";
        out += &*print_expr(expr);
    }
    format!("{})", out)
}

pub fn print_expr(expr: &Expr) -> String {
    let out: String = match expr {
        Expr::Binary(b) => parenthesize(b.operator.print(), &[b.left.as_ref(), b.right.as_ref()]),
        Expr::Grouping(g) => parenthesize("group".to_string(), &[g.expression.as_ref()]),
        Expr::Literal(l) => format!("{}", l.value.print()),
        Expr::Unary(u) => parenthesize(u.operator.print(), &[u.right.as_ref()]),
    };
    format!("{}", out)
}

// fn main() {
//     let expr: Expr = Expr::Binary(Box::from(BinaryExpression {
//         left: Box::from(Expr::Unary(Box::from(UnaryExpression {
//             operator: Token::new(TokenType::Minus, "-".to_string(), 1, Object::None),
//             right: Box::from(Expr::Literal(Box::from(LiteralExpression {
//                 literal_type: Object::Number,
//                 value: Token::new(TokenType::Number, "123".to_string(), 1, Object::Number),
//             }))),
//         }))),
//         operator: Token::new(TokenType::Star, "*".to_string(), 1, Object::None),
//         right: Box::from(Expr::Grouping(Box::from(GroupingExpression {
//             expression: Box::from(Expr::Literal(Box::from(LiteralExpression {
//                 literal_type: Object::Number,
//                 value: Token::new(TokenType::Number, "45.67".to_string(), 1, Object::Number),
//             }))),
//         }))),
//     }));
//     println!("{}", print_expr(&expr))
// }
