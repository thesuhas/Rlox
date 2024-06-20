use crate::environment::Environment;
use crate::expr::{AssignmentExpression, BinaryExpression, Expr, GroupingExpression, LiteralExpression, UnaryExpression, VariableExpression};
use crate::stmt::{ExpressionStatement, PrintStatement, Stmt, VarStmt};
use crate::token::Object;
use crate::token_type::TokenType;
use crate::Rlox;
use std::any::{Any, TypeId};
use std::io::{stdout, Write};
use std::string::String;

pub struct Interpreter<'a> {
    env: Environment,
    rlox: &'a mut Rlox,
}

impl Interpreter<'_> {
    pub fn new(rlox: &mut Rlox) -> Interpreter {
        Interpreter {
            rlox,
            env: Environment::new(),
        }
    }

    pub fn interpret(&mut self, stmts: Vec<Stmt>) {
        for stmt in stmts {
            self.execute(stmt);
        }
    }

    fn stringify(&self, val: Box<dyn Any>) -> String {
        let test_val = (&*val).downcast_ref::<Option<String>>();
        return match test_val {
            None => {
                let type_id = (&*val).type_id();
                if type_id == TypeId::of::<f64>() {
                    let num: Box<f64> = val.downcast().unwrap();
                    let mut num_str = (*num).to_string();
                    if num_str.ends_with(".0") {
                        num_str.truncate(num_str.len() - 2);
                    }
                    num_str
                } else if type_id == TypeId::of::<String>() {
                    let str: Box<String> = val.downcast().unwrap();
                    *str
                } else {
                    unreachable!()
                }
            }
            Some(_) => "nil".to_string(),
        };
    }

    fn visit_literal_expr(&self, expr: LiteralExpression) -> Box<dyn Any> {
        let val: String = Expr::get_literal_value(expr.clone());
        match Expr::get_literal_type(expr) {
            Object::Nil => Box::from(Option::<String>::None),
            Object::Number => Box::from(val.parse::<f64>().unwrap()),
            Object::Bool => {
                if matches!(val.as_str(), "false") {
                    Box::from(false)
                } else {
                    Box::from(true)
                }
            }
            Object::String => {
                let new: Box<dyn Any> = Box::new(val);
                new
            }
        }
    }

    fn visit_group_expr(&mut self, expr: GroupingExpression) -> Box<dyn Any> {
        self.evaluate(Expr::get_grouping_expr(expr))
    }

    fn check_number_operand(&mut self, type_id: TypeId, line: i32) {
        if type_id == TypeId::of::<f64>() {
            return;
        }
        self.rlox
            .runtime_error(line, "Operand must be a number".to_string());
    }

    fn check_number_operands(&mut self, left_type: TypeId, right_type: TypeId, line: i32) {
        if left_type == TypeId::of::<f64>() && right_type == TypeId::of::<f64>() {
            return;
        }

        self.rlox
            .runtime_error(line, "Operands must be numbers".to_string());
    }

    fn visit_unary_expr(&mut self, expr: UnaryExpression) -> Box<dyn Any> {
        let line: i32 = Expr::get_unary_line(expr.clone());
        let val = self.evaluate(Expr::get_unary_expr(expr.clone()));
        match Expr::get_unary_op(expr.clone()) {
            TokenType::Minus => {
                self.check_number_operand((&*val).type_id(), line);
                let temp: Box<f64> = val.downcast().unwrap();
                return Box::from(-(*temp));
            }
            TokenType::Bang => {
                let right = self.evaluate(Expr::get_unary_expr(expr.clone()));
                let test_right = (&*right).downcast_ref::<Option<String>>();
                match test_right {
                    None => {
                        let actual_id = (&*right).type_id();
                        if actual_id == TypeId::of::<bool>() {
                            let temp: Box<bool> = right.downcast().unwrap();
                            return Box::from(!self.is_truthy(Object::Bool, *temp));
                        }
                    }
                    Some(_) => return Box::from(!self.is_truthy(Object::Nil, false)),
                }
                return Box::from(!self.is_truthy(Object::String, false));
            }
            // There should not be any other types of operations in Unary Expressions
            _ => unreachable!(),
        }
    }

    fn visit_binary_expr(&mut self, expr: BinaryExpression) -> Box<dyn Any> {
        let left = self.evaluate(Expr::get_binary_left(expr.clone()));
        let right = self.evaluate(Expr::get_binary_right(expr.clone()));
        let line: i32 = Expr::get_binary_line(expr.clone());
        match Expr::get_binary_op(expr.clone()) {
            TokenType::Minus => self.evaluate_numbers(left, right, TokenType::Minus, line),
            TokenType::Slash => self.evaluate_numbers(left, right, TokenType::Slash, line),
            TokenType::Star => self.evaluate_numbers(left, right, TokenType::Star, line),
            TokenType::Plus => {
                let right_type = (&*right).type_id();
                let left_type = (&*left).type_id();

                if left_type == TypeId::of::<String>() && right_type == TypeId::of::<String>() {
                    let mut left_string: Box<String> = left.downcast().unwrap();
                    let right_string: Box<String> = right.downcast().unwrap();
                    left_string.push_str(right_string.as_str());
                    return Box::new(*left_string);
                } else if left_type == TypeId::of::<f64>() && right_type == TypeId::of::<f64>() {
                    return self.evaluate_numbers(left, right, TokenType::Plus, line);
                } else {
                    self.rlox.runtime_error(
                        line,
                        "Operands must be either numbers or strings".to_string(),
                    );
                    unreachable!()
                }
            }
            TokenType::Greater => self.evaluate_numbers(left, right, TokenType::Greater, line),
            TokenType::GreaterEqual => {
                self.evaluate_numbers(left, right, TokenType::GreaterEqual, line)
            }
            TokenType::Less => self.evaluate_numbers(left, right, TokenType::Less, line),
            TokenType::LessEqual => self.evaluate_numbers(left, right, TokenType::LessEqual, line),
            TokenType::BangEqual => Box::from(!self.is_equal(left, right)),
            TokenType::EqualEqual => Box::from(self.is_equal(left, right)),
            _ => unreachable!(),
        }
    }

    fn evaluate_numbers(
        &mut self,
        left: Box<dyn Any>,
        right: Box<dyn Any>,
        op: TokenType,
        line: i32,
    ) -> Box<dyn Any> {
        self.check_number_operands((&*left).type_id(), (&*right).type_id(), line);
        let left_num: Box<f64> = left.downcast().unwrap();
        let right_num: Box<f64> = right.downcast().unwrap();

        match op {
            TokenType::Plus => Box::from((*left_num) + (*right_num)),
            TokenType::Minus => Box::from((*left_num) - (*right_num)),
            TokenType::Slash => Box::from((*left_num) / (*right_num)),
            TokenType::Star => Box::from((*left_num) * (*right_num)),
            TokenType::Greater => Box::from((*left_num) > (*right_num)),
            TokenType::GreaterEqual => Box::from((*left_num) >= (*right_num)),
            TokenType::Less => Box::from((*left_num) < (*right_num)),
            TokenType::LessEqual => Box::from((*left_num) <= (*right_num)),
            _ => {
                unreachable!()
            }
        }
    }

    fn is_equal(&self, left: Box<dyn Any>, right: Box<dyn Any>) -> bool {
        let left_test = (&*left).downcast_ref::<Option<String>>();
        let right_test = (&*right).downcast_ref::<Option<String>>();
        match left_test {
            None => match right_test {
                None => {
                    let right_type = (&*right).type_id();
                    let left_type = (&*left).type_id();
                    if left_type == TypeId::of::<f64>() && right_type == TypeId::of::<f64>() {
                        let left_num: Box<f64> = left.downcast().unwrap();
                        let right_num: Box<f64> = right.downcast().unwrap();
                        return (*left_num) == (*right_num);
                    } else if left_type == TypeId::of::<String>()
                        && right_type == TypeId::of::<String>()
                    {
                        let left_str: Box<String> = left.downcast().unwrap();
                        let right_str: Box<String> = right.downcast().unwrap();
                        return (*left_str) == (*right_str);
                    }
                }
                Some(_) => return false,
            },
            Some(_) => {
                return match right_test {
                    None => false,
                    Some(_) => true,
                }
            }
        }

        unreachable!()
    }

    fn is_truthy(&self, object: Object, val: bool) -> bool {
        return match object {
            Object::Nil => false,
            Object::Bool => val,
            _ => true,
        };
    }

    fn visit_variable_expr(&mut self, expr: VariableExpression) -> Box<dyn Any> {
        self.env.get(Expr::get_var_name(expr))
    }

    fn visit_assign_expr(&mut self, expr: AssignmentExpression) -> Box<dyn Any> {
        // Evaluate the rhs
        let val: Box<dyn Any> = self.evaluate(Expr::get_assign_val(expr.clone()));
        // Set the value
        self.env.assign(Expr::get_assign_name(expr), val)
    }

    fn evaluate(&mut self, expr: Expr) -> Box<dyn Any> {
        match expr {
            Expr::Literal(expr) => self.visit_literal_expr((*expr).clone()),
            Expr::Grouping(expr) => self.visit_group_expr((*expr).clone()),
            Expr::Unary(expr) => self.visit_unary_expr((*expr).clone()),
            Expr::Binary(expr) => self.visit_binary_expr((*expr).clone()),
            Expr::Variable(expr) => self.visit_variable_expr((*expr).clone()),
            Expr::Assign(expr) => self.visit_assign_expr((*expr).clone()),
        }
    }

    fn execute(&mut self, stmt: Stmt) {
        match stmt.clone() {
            Stmt::Expression(stmt) => self.visit_expr_stmt((*stmt).clone()),
            Stmt::Print(stmt) => self.visit_print_stmt((*stmt).clone()),
            Stmt::Var(stmt) => self.visit_var_stmt((*stmt).clone()),
        }
    }

    fn visit_expr_stmt(&mut self, stmt: ExpressionStatement) {
        self.evaluate(Stmt::get_expr_stmt_expr(stmt));
    }

    fn visit_print_stmt(&mut self, stmt: PrintStatement) {
        let out = self.evaluate(Stmt::get_print_stmt_expr(stmt));
        let str_out = self.stringify(out);
        println!("{:?}", str_out);
        stdout().flush().expect("Unable to flush to stdout!");
    }

    fn visit_var_stmt(&mut self, stmt: VarStmt) {
        let val: Box<dyn Any>;
        let opt: Option<Expr> = Stmt::get_var_initializer(stmt.clone());
        match opt {
            Some(opt) => {
                val = self.evaluate(opt);
            }
            None => {
                val = Box::from(Option::<String>::None);
            }
        }
        let ty = (&*val).downcast_ref::<Option<String>>();
        match ty {
            Some(_) => self.env.define(Stmt::get_var_key(stmt), val, Object::Nil),
            None => {
                let type_id = (&*val).type_id();
                if type_id == TypeId::of::<f64>() {
                    self.env
                        .define(Stmt::get_var_key(stmt), val, Object::Number);
                } else {
                    self.env
                        .define(Stmt::get_var_key(stmt), val, Object::String);
                }
            }
        }
    }
}
