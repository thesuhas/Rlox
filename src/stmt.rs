use crate::expr::Expr;

#[derive(Clone)]
pub enum Stmt{
    ExpressionStatement(Box<ExpressionStatement>),
    PrintStatement(Box<PrintStatement>),
}

#[derive(Clone)]
pub struct  ExpressionStatement {
    expression: Box<Expr>,
}

#[derive(Clone)]
pub struct PrintStatement {
    expression: Box<Expr>,
}

impl Stmt {
    pub fn get_expr_stmt_expr(stmt: ExpressionStatement) -> Expr{
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
        PrintStatement{
            expression: Box::from(expr),
        }
    }
}