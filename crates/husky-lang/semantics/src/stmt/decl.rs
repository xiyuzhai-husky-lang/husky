mod gen_instructions;

pub(crate) use gen_instructions::gen_decl_stmt_instructions;

use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeclStmt {
    pub kind: DeclStmtKind,
    pub indent: fold::Indent,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeclStmtKind {
    Init {
        varname: CustomIdentifier,
        value: Arc<Expr>,
    },
    Assert {
        condition: Arc<Expr>,
    },
    Return {
        result: Arc<Expr>,
    },
    Branch {
        conditional_blocks: Vec<ConditionalBlock>,
        default_block: Option<Vec<Arc<DeclStmt>>>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConditionalBlock {
    pub condition: Arc<Expr>,
    pub stmts: Vec<Arc<DeclStmt>>,
}
