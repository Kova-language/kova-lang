use crate::lang::stmt::Stmt;
use crate::lang::expr::{Token, TokenKind, Expr, Literal};
use crate::lang::errors::KovaError;

use anyhow::{anyhow, Result};
use inkwell::context::Context;
use inkwell::OptimizationLevel;
use inkwell::builder::Builder;
use inkwell::module::Module;
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use inkwell::values::{IntValue, FunctionValue, GlobalValue};
use inkwell::types::IntType;

pub struct Compiler<'ctx> {
    context: &'ctx Context,
    current_temp: i32,
}

impl<'ctx> Compiler<'ctx> {
    pub fn new(context: &'ctx Context) -> Self {
        Self {
            context,
            current_temp: 0,
        }
    }

    fn setup(&self, name: &str) -> (Module<'ctx>, Builder<'ctx>) {
        (
            self.context.create_module(name),
            self.context.create_builder(),
        )
    }

    pub fn fresh_temp(&mut self) -> String {
        self.current_temp += 1;
        format!("t{}", self.current_temp)
    }

    unsafe fn emit_binary_expr(&mut self, module: &Module<'ctx>, builder: &Builder<'ctx>, left: &Box<Expr>, operator: &Token, right: &Box<Expr>) -> Result<IntValue<'ctx>> {
        let lhs = self.emit_expr(module, builder, left)?;
        let rhs = self.emit_expr(module, builder, right)?;
        let result = match operator.kind {
            TokenKind::Plus => builder.build_int_add(lhs, rhs, &*self.fresh_temp())?,
            TokenKind::Minus => builder.build_int_sub(lhs, rhs, &*self.fresh_temp())?,
            TokenKind::Mult => builder.build_int_mul(lhs, rhs, &*self.fresh_temp())?,
            TokenKind::Div => builder.build_int_signed_div(lhs, rhs, &*self.fresh_temp())?,
            _ => return Err(KovaError::CompileError { message: "unexpected binary operator".into() }.into()),
        };

        Ok(result)
    }


    fn emit_literal_expr(
        &self,
        literal: &Literal,
    ) -> Result<IntValue<'ctx>> {
        match literal {
            Literal::Integer(value) => {
                Ok(self.context.i64_type().const_int(*value as u64, true))
            }
        }
    }

    unsafe fn emit_expr(&mut self, module: &Module<'ctx>, builder: &Builder<'ctx>, expr: &Expr) -> Result<IntValue<'ctx>> {
        Ok(match expr {
            Expr::Binary { left, operator, right } => self.emit_binary_expr(module, builder, left, operator, right)?,
            Expr::Literal( literal ) => self.emit_literal_expr(literal)?,
            _ => return Err(KovaError::CompileError { message: "unsupported expression kind".into() }.into()),
        })
    }

    unsafe fn emit_let(&mut self, module: &Module<'ctx>, builder: &Builder<'ctx>, name: &Token, value: &Expr) -> Result<IntValue<'ctx>> {
        let value_ptr = self.emit_expr(module, builder, value)?;
        let alloca = builder.build_alloca(self.context.i64_type(), &*name.lexeme)?;
        builder.build_store(alloca, value_ptr)?;
        Ok(value_ptr)
    }

    unsafe fn emit_stmt(&mut self, module: &Module<'ctx>, builder: &Builder<'ctx>, stmt: Stmt) -> Result<IntValue<'ctx>> {
        Ok(match &stmt {
            Stmt::Let { name, value } => self.emit_let(module, builder, name, value)?,
            _ => return Err(KovaError::CompileError { message: "unsupported stmt kind".into() }.into())
            // Stmt::Assign { name, value } => self.emit_assign(stmt)?;
            // Stmt::Call { name, params } => self.emit_call(name, params)?;
        })
    }

    pub unsafe fn emit(&mut self, ast: Vec<Stmt>) -> Result<()> {
        let (module, builder) = self.setup("kova-test");

        let main_type = self.context.i32_type().fn_type(&[], false);
        let main = module.add_function("main", main_type, None);
        let entry = self.context.append_basic_block(main, "entry");

        builder.position_at_end(entry);

        for stmt in ast {
            self.emit_stmt(&module, &builder, stmt)?;
        }

        let exit_code = self.context.i32_type().const_zero();
        builder.build_return(Some(&exit_code))?;

        module
            .verify()
            .map_err(|message| anyhow!("{}", message))?;

        println!("{}", module.print_to_string());

        Ok(())
    }
}