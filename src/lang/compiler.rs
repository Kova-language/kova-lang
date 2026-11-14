use anyhow::Result;
use inkwell::context::Context;
use inkwell::OptimizationLevel;

use inkwell::builder::Builder;
use inkwell::module::Module;
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use inkwell::values::{IntValue, FunctionValue};
use inkwell::types::IntType;

pub struct Compiler {
    context: Context
}

impl Compiler {
    pub unsafe fn new() -> Self {
        Self { context: Context::create() }
    }

    unsafe fn setup(&self, name: &str) -> (Module, Builder) {
        (self.context.create_module(name), self.context.create_builder())
    }

    pub unsafe fn emit(&self, ast: Vec<Stmt>) -> Result<()> {
        let (module, builder) = self.setup("kova-test");
        let ir = String::new();
        for stmt in ast {
            match stmt {

            }
        }
        Ok(())
    }
}