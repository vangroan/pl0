use crate::ast::*;
use crate::codegen::CodeGen;
use crate::errors::Result;

pub struct Compiler<'a, C> {
    codegen: &'a mut C,
}

impl<'a, C: CodeGen> Compiler<'a, C> {
    pub fn new(codegen: &'a mut C) -> Self {
        Self { codegen }
    }

    pub fn compile(&mut self, program: &Program) -> Result<()> {
        self.compile_program(program)?;

        Ok(())
    }
}

impl<'a, C: CodeGen> Compiler<'a, C> {
    fn compile_program(&mut self, program: &Program) -> Result<()> {
        self.compile_block(&program.block)
    }

    fn compile_block(&mut self, block: &Block) -> Result<()> {
        // TODO: Const
        // TODO: Var
        // TODO: Procedures
        self.compile_stmt(&block.stmt)
    }

    fn compile_stmts(&mut self) -> Result<()> {
        todo!()
    }

    fn compile_stmt(&mut self, stmt: &Stmt) -> Result<()> {
        match stmt {
            Stmt::Assign(_) => todo!(),
            Stmt::Call => todo!(),
            Stmt::Read => todo!(),
            Stmt::Write(write) => {
                self.compile_expr(&write.expr)?;
                self.codegen.emit_write()
            }
            Stmt::SubBlock(sub_block) => self.compile_sub_block(sub_block),
            Stmt::If(_) => todo!(),
            Stmt::While(_) => todo!(),
        }
    }

    fn compile_sub_block(&mut self, sub_block: &SubBlock) -> Result<()> {
        for stmt in &sub_block.stmts {
            self.compile_stmt(stmt)?;
        }
        Ok(())
    }

    fn compile_expr(&mut self, expr: &Expr) -> Result<()> {
        match expr {
            // Push number literal onto the stack.
            Expr::Num(num) => self.codegen.emit_lit(*num),
            Expr::Binary(_bin_expr) => todo!(),
            Expr::Err() => panic!("abstract-syntax-tree contains an error node"),
        }
    }
}
