mod condition_branch;
mod parse;
mod pattern_branch;

pub use condition_branch::*;
pub use pattern_branch::*;

use super::parser::EagerStmtParser;
use super::*;
use crate::*;
use file::FilePtr;
use std::sync::Arc;
use text::RangedCustomIdentifier;
use text::TextRange;
use vm::{InstructionId, InstructionSource};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FuncStmt {
    pub file: FilePtr,
    pub range: TextRange,
    pub indent: fold::Indent,
    pub variant: FuncStmtVariant,
    pub instruction_id: InstructionId,
}

impl InstructionSource for FuncStmt {
    fn instruction_id(&self) -> InstructionId {
        self.instruction_id
    }

    fn file(&self) -> FilePtr {
        self.file
    }

    fn text_range(&self) -> TextRange {
        self.range
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FuncStmtVariant {
    Init {
        varname: RangedCustomIdentifier,
        initial_value: Arc<EagerExpr>,
    },
    Assert {
        condition: Arc<EagerExpr>,
    },
    Return {
        result: Arc<EagerExpr>,
    },
    ReturnXml {
        xml_expr: Arc<XmlExpr>,
    },
    ConditionFlow {
        branches: Vec<Arc<FuncConditionBranch>>,
    },
    Match {
        match_expr: Arc<EagerExpr>,
        branches: Vec<Arc<FuncPatternBranch>>,
    },
}

pub fn parse_func_stmts(
    parameters: &[InputParameter],
    db: &dyn InferQueryGroup,
    arena: &RawExprArena,
    iter: AstIter,
    file: FilePtr,
) -> SemanticResultArc<Vec<Arc<FuncStmt>>> {
    EagerStmtParser::new(parameters, db, arena, file).parse_func_stmts(iter)
}
