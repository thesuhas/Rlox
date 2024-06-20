use crate::expr::Expr;
use crate::token::Token;

#[derive(Clone)]
pub enum Stmt {
    Expression(Box<ExpressionStatement>),
    Print(Box<PrintStatement>),
    Var(Box<VarStmt>),
}

#[derive(Clone)]
pub struct ExpressionStatement {
    expression: Box<Expr>,
}

#[derive(Clone)]
pub struct VarStmt {
    name: Token,
    initializer: Option<Expr>,
}

#[derive(Clone)]
pub struct PrintStatement {
    expression: Box<Expr>,
}

impl Stmt {
    pub fn get_expr_stmt_expr(stmt: ExpressionStatement) -> Expr {
        (*stmt.expression).clone()
    }

    pub fn get_print_stmt_expr(stmt: PrintStatement) -> Expr {
        (*stmt.expression).clone()
    }

    pub fn new_exp_stmt(expr: Expr) -> ExpressionStatement {
        ExpressionStatement {
            expression: Box::from(expr),
        }
    }

    pub fn new_print_stmt(expr: Expr) -> PrintStatement {
        PrintStatement {
            expression: Box::from(expr),
        }
    }

    pub fn new_var_stmt(tok: Token, expr: Option<Expr>) -> VarStmt {
        VarStmt {
            name: tok,
            initializer: expr,
        }
    }

    pub fn get_var_initializer(stmt: VarStmt) -> Option<Expr> {
        stmt.initializer
    }

    pub fn get_var_key(stmt: VarStmt) -> String {
        stmt.name.get_lexeme()
    }
}
