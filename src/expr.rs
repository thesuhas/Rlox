use crate::token::{Object, Token};
use crate::token_type::TokenType;

// #[path = "./token_type.rs"]
// mod token_type;
//
// #[path = "./token.rs"]
// mod token;

// use token_type::TokenType;
// use token::{Token, Object};

#[derive(Clone)]
pub enum Expr {
    Assign(Box<AssignmentExpression>),
    Binary(Box<BinaryExpression>),
    Grouping(Box<GroupingExpression>),
    Literal(Box<LiteralExpression>),
    Unary(Box<UnaryExpression>),
    Variable(Box<VariableExpression>),
}

#[derive(Clone)]
pub struct AssignmentExpression {
    name: Token,
    val: Box<Expr>,
}

#[derive(Clone)]
pub struct BinaryExpression {
    left: Box<Expr>,
    operator: Token,
    right: Box<Expr>,
}

#[derive(Clone)]
pub struct GroupingExpression {
    expression: Box<Expr>,
}

#[derive(Clone)]
pub struct LiteralExpression {
    literal_type: Object,
    value: Token,
}

#[derive(Clone)]
pub struct UnaryExpression {
    operator: Token,
    right: Box<Expr>,
}

#[derive(Clone)]
pub struct VariableExpression {
    name: Token,
}

impl Expr {
    pub fn get_literal_value(expr: LiteralExpression) -> String {
        expr.value.get_lexeme()
    }

    pub fn get_literal_type(expr: LiteralExpression) -> Object {
        expr.literal_type
    }

    pub fn get_grouping_expr(expr: GroupingExpression) -> Expr {
        (*expr.expression).clone()
    }

    pub fn get_unary_expr(expr: UnaryExpression) -> Expr {
        (*expr.right).clone()
    }

    pub fn get_unary_op(expr: UnaryExpression) -> TokenType {
        expr.operator.get_type()
    }

    pub fn get_binary_left(expr: BinaryExpression) -> Expr {
        (*expr.left).clone()
    }

    pub fn get_binary_line(expr: BinaryExpression) -> i32 {
        expr.operator.get_line()
    }

    pub fn get_binary_right(expr: BinaryExpression) -> Expr {
        (*expr.right).clone()
    }

    pub fn get_binary_op(expr: BinaryExpression) -> TokenType {
        expr.operator.get_type()
    }

    pub fn new_binary(left: Expr, operator: Token, right: Expr) -> Expr {
        Expr::Binary(Box::from(BinaryExpression {
            left: Box::from(left),
            operator,
            right: Box::from(right),
        }))
    }

    pub fn new_unary(operator: Token, right: Expr) -> Expr {
        Expr::Unary(Box::from(UnaryExpression {
            operator,
            right: Box::from(right),
        }))
    }

    pub fn get_unary_line(expr: UnaryExpression) -> i32 {
        expr.operator.get_line()
    }

    pub fn new_literal(literal_type: Object, value: Token) -> Expr {
        Expr::Literal(Box::from(LiteralExpression {
            literal_type,
            value,
        }))
    }

    pub fn new_variable(name: Token) -> Expr {
        Expr::Variable(Box::from(VariableExpression { name }))
    }

    pub fn get_var_name(expr: VariableExpression) -> Token {
        expr.name
    }

    pub fn new_grouping(expression: Expr) -> Expr {
        Expr::Grouping(Box::from(GroupingExpression {
            expression: Box::from(expression),
        }))
    }

    pub fn get_assign_name(expr: AssignmentExpression) -> Token {
        expr.name
    }

    pub fn get_assign_val(expr: AssignmentExpression) -> Expr {
        (*expr.val).clone()
    }

    pub fn new_assign_expr(name: Token, val: Expr) -> Expr {
        Expr::Assign(Box::from(AssignmentExpression{name, val: Box::from(val)}))
    }
}

// Below contents are of the AST Printer Class from the book
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
        Expr::Variable(v) => format!("{} ", v.name.to_string()),
        Expr::Assign(a) => format!("{}", a.name.to_string()),
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
