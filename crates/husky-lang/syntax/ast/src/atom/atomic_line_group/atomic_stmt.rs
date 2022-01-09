use hir::Env;
use word::{CustomIdentifier, StmtKeyword};

use super::{symbol_proxy::SymbolProxy, *};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AtomicStmt {
    Loop(AtomicLoopStmt),
    Branch(AtomicBranchStmt),
    Exec(Vec<Atom>),
    Init {
        kind: hir::InitKind,
        varname: CustomIdentifier,
        initial_value: Vec<Atom>,
    },
    Return(Vec<Atom>),
    Assert(Vec<Atom>),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AtomicLoopStmt {}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AtomicBranchStmt {
    If { condition: Vec<Atom> },
    Elif { condition: Vec<Atom> },
    Else,
}

// new
impl AtomicStmt {
    pub(crate) fn parse(
        env: Env,
        scope_proxy: SymbolProxy,
        keyword: Option<(StmtKeyword, TextRange)>,
        is_head: bool,
        tokens: &[Token],
    ) -> AstResult<Self> {
        if let Some((keyword, ..)) = keyword {
            match keyword {
                StmtKeyword::Let => todo!(),
                StmtKeyword::Var => todo!(),
                StmtKeyword::If => todo!(),
                StmtKeyword::Elif => todo!(),
                StmtKeyword::Else => todo!(),
                StmtKeyword::Switch => todo!(),
                StmtKeyword::Match => todo!(),
                StmtKeyword::Case => todo!(),
                StmtKeyword::DeFault => todo!(),
                StmtKeyword::For => todo!(),
                StmtKeyword::Ext => todo!(),
                StmtKeyword::ForExt => todo!(),
                StmtKeyword::While => todo!(),
                StmtKeyword::Do => todo!(),
                StmtKeyword::Break => todo!(),
                StmtKeyword::Return => todo!(),
                StmtKeyword::Assert => todo!(),
            }
        } else {
            todo!()
        }
    }
}
