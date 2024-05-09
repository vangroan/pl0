#[allow(dead_code)]

pub struct Program {
    block: Block,
}

pub struct Block {
    pub stmts: Vec<Stmt>,
}

pub struct Stmt {}
