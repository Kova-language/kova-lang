use crate::lang::stmt::Stmt;
use crate::lang::expr::{Token, TokenKind, Expr};
use crate::lang::errors::KovaError;

use anyhow::{anyhow, Result};
use inkwell::context::Context;
use inkwell::OptimizationLevel;
use inkwell::builder::Builder;
use inkwell::module::Module;
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use inkwell::values::{IntValue, FunctionValue};
use inkwell::types::IntType;

pub struct Compiler {
    context: Context,
    current_temp: i32,
}

impl Compiler {
    pub unsafe fn new() -> Self {
        Self { context: Context::create(), current_temp: 0 }
    }

    fn compile_error(message: &str) -> KovaError {
        KovaError::CompileError { message }
    }

    pub fn fresh_temp(&mut self) -> String {
        self.current_temp += 1;
        format!("t{}", self.current_temp)
    }

    unsafe fn setup(&self, name: &str) -> (Module, Builder) {
        (self.context.create_module(name), self.context.create_builder())
    }

    unsafe fn emit_binary_expr<'ctx>(&mut self, module: &Module, builder: &Builder, left: &Box<Expr>, operator: &Token, right: &Box<Expr>) -> anyhow::Result<IntValue<'ctx>> {
        let lhs = self.emit_expr(module, builder, left);
        let rhs = self.emit_expr(module, builder, right);
        let result = match operator.kind {
            TokenKind::Plus => builder.build_int_add(lhs, rhs, &*self.fresh_temp())?,
            TokenKind::Minus => builder.build_int_sub(lhs, rhs, &*self.fresh_temp())?,
            TokenKind::Mult => builder.build_int_mul(lhs, rhs, &*self.fresh_temp())?,
            TokenKind::Div => builder.build_int_signed_div(lhs, rhs, &*self.fresh_temp())?,
            _ => return Err(KovaError::CompileError { message: "unexpected binary operator".into() }.into()),
        };

        Ok(result)
    }

    unsafe fn emit_expr(&mut self, module: &Module, builder: &Builder, expr: &Expr) -> Result<> {
        Ok(match expr {
            Expr::Binary { left, operator, right } => self.emit_binary_expr(module, builder, left, operator, right)?,
            _ => {}
        })
    }

    unsafe fn emit_let(&mut self, module: &Module, builder: &Builder, name: &Token, value: &Expr) -> Result<> {
        let value_ptr = self.emit_expr(module, builder, value)?;
        builder.build_alloca()
    }

    unsafe fn emit_stmt(&mut self, module: &Module, builder: &Builder, stmt: Stmt) -> Result<()> {
        Ok(match &stmt {
            Stmt::Let { name, value } => self.emit_let(module, builder, name, value)?,
            Stmt::Assign { name, value } => self.emit_assign(stmt)?;
            Stmt::Call { name, params } => self.emit_call(name, params)?;
        })
    }

    pub unsafe fn emit(&self, ast: Vec<Stmt>) -> Result<()> {
        let (module, builder) = self.setup("kova-test");
        let ir = String::new();
        for stmt in ast {
            self.emit_stmt(&module, &builder, stmt)?;
        }
        println!("{}", ir);
        Ok(())
    }
}