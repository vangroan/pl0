use crate::codegen::CodeGen;
use crate::errors::Result;
use crate::limits::*;
use crate::{ast::*, error, Num};

pub struct Compiler<'a, C> {
    codegen: &'a mut C,
    table: Vec<Entry>,
    level: u8,
    /// The local relative stack offset where the procedure's data starts
    data_offset: u16,
}

/// Entry in the identifier table.
enum Entry {
    Const { name: String, value: Num },
    Var { name: String, level: u8, offset: u16 },
    Proc { name: String, level: u8, addr: u16 },
}

impl<'a, C: CodeGen> Compiler<'a, C> {
    pub fn new(codegen: &'a mut C) -> Self {
        Self {
            codegen,
            table: vec![],
            level: 0,
            data_offset: DATA_OFFSET as u16,
        }
    }

    pub fn compile(&mut self, program: &Program) -> Result<()> {
        self.compile_program(program)?;

        Ok(())
    }

    fn find_ident(&self, query: &str) -> Option<&Entry> {
        // Search backwards, crawling up lexical scope.
        self.table.iter().rev().find(|entry| match entry {
            Entry::Const { name, .. } => name == query,
            Entry::Var { name, .. } => name == query,
            Entry::Proc { name, .. } => name == query,
        })
    }

    fn with_scope<F>(&mut self, f: F) -> Result<()>
    where
        F: FnOnce(&mut Self) -> Result<()>,
    {
        let ident_len = self.table.len();
        let data_offset = self.data_offset;
        self.data_offset = DATA_OFFSET as u16;
        self.level += 1;

        let result = f(self);

        self.level -= 1;
        self.data_offset = data_offset;
        // Identifiers are now out of scope.
        self.table.truncate(ident_len);

        result
    }
}

impl<'a, C: CodeGen> Compiler<'a, C> {
    fn compile_program(&mut self, program: &Program) -> Result<()> {
        self.compile_block(&program.block)
    }

    fn compile_block(&mut self, block: &Block) -> Result<()> {
        // Interpreter has to jump over all the generated procedure
        // bodies to get to this block's statement.
        let index = self.codegen.reserve_jump()?;

        self.compile_consts(&block.consts)?;
        self.compile_vars(&block.vars)?;
        self.compile_procs(&block.procs)?;

        // Patch the starting address of this block's bytecode
        // into the jump instruction emitted at the beginning.
        let addr = self.codegen.len();
        self.codegen.patch_jump(index, addr as u16)?;

        // The stack space required by a procedure is encoded
        // in this bytecode.
        self.codegen.emit_inc_top(self.data_offset)?;
        self.compile_stmt(&block.stmt)?;
        self.codegen.emit_return()?;

        Ok(())
    }

    fn compile_consts(&mut self, consts: &[Const]) -> Result<()> {
        for const_ in consts {
            self.table.push(Entry::Const {
                name: const_.ident.name.clone(),
                value: const_.value,
            });
        }
        Ok(())
    }

    fn compile_vars(&mut self, vars: &[Var]) -> Result<()> {
        println!("level: {}, vars {:?}", self.level, vars);
        for var in vars {
            self.table.push(Entry::Var {
                name: var.ident.name.clone(),
                level: self.level,
                offset: self.data_offset,
            });
            self.data_offset += 1;
        }
        Ok(())
    }

    fn compile_procs(&mut self, procs: &[Proc]) -> Result<()> {
        for proc in procs {
            // Enter identifier so procedures can call themselves recursively.
            let name = proc.name.name.clone();
            let addr = self.codegen.len() as u16;
            self.table.push(Entry::Proc {
                name,
                level: self.level,
                addr,
            });

            self.with_scope(|compiler| compiler.compile_block(&proc.body))?
        }

        Ok(())
    }

    fn compile_stmt(&mut self, stmt: &Stmt) -> Result<()> {
        match stmt {
            Stmt::Assign(assign) => self.compile_assign(assign),
            Stmt::Call(call) => self.compile_call(call),
            Stmt::Read(_read) => todo!(),
            Stmt::Write(write) => {
                self.compile_expr(&write.expr)?;
                self.codegen.emit_write()
            }
            Stmt::SubBlock(sub_block) => self.compile_sub_block(sub_block),
            Stmt::If(_) => todo!(),
            Stmt::While(_) => todo!(),
        }
    }

    fn compile_assign(&mut self, assign: &AssignStmt) -> Result<()> {
        self.compile_expr(&assign.rhs)?;

        match self.find_ident(assign.lhs.name.as_str()) {
            Some(entry) => match entry {
                Entry::Const { .. } => error!(
                    "compiler",
                    "expected '{}' to be variable; found constant", assign.lhs.name
                )
                .into(),
                Entry::Var { level, offset, .. } => self.codegen.emit_store(self.level - *level, *offset),
                Entry::Proc { .. } => error!(
                    "compiler",
                    "expected '{}' to be variable; found procedure", assign.lhs.name
                )
                .into(),
            },
            None => error!("compiler", "unresolved indentifier: {}", assign.lhs.name).into(),
        }
    }

    fn compile_call(&mut self, call: &CallStmt) -> Result<()> {
        match self.find_ident(call.name.name.as_str()) {
            Some(entry) => match entry {
                Entry::Const { .. } => error!(
                    "compiler",
                    "expected '{}' to be procedure; found constant", call.name.name
                )
                .into(),
                Entry::Var { .. } => error!(
                    "compiler",
                    "expected '{}' to be procedure; found variable", call.name.name
                )
                .into(),
                Entry::Proc { level, addr, .. } => self.codegen.emit_call(self.level - level, *addr),
            },
            None => error!("compiler", "unresolved indentifier: {}", call.name.name).into(),
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
            Expr::Unary(expr) => {
                self.compile_expr(&expr.expr)?;
                if expr.op == UnOp::Neg {
                    self.codegen.emit_math_neg()?;
                }
                Ok(())
            }
            Expr::Binary(bin_expr) => {
                self.compile_expr(&bin_expr.lhs)?;
                self.compile_expr(&bin_expr.rhs)?;
                match bin_expr.op {
                    BinOp::Add => self.codegen.emit_math_add(),
                    BinOp::Sub => self.codegen.emit_math_sub(),
                    BinOp::Mul => self.codegen.emit_math_mul(),
                    BinOp::Div => self.codegen.emit_math_div(),
                }
            }
            Expr::Name(name) => match self.find_ident(name.name.as_str()) {
                Some(entry) => match entry {
                    Entry::Const { name, value } => self.codegen.emit_lit(*value),
                    Entry::Var {
                        level, offset: addr, ..
                    } => self.codegen.emit_load(self.level - level, *addr),
                    Entry::Proc { .. } => return error!("compiler", "procedure call not allowed in expression").into(),
                },
                None => {
                    return error!("compiler", "unresolved indentifier: {}", name.name).into();
                }
            },
            Expr::Err() => panic!("abstract-syntax-tree contains an error node"),
        }
    }
}
