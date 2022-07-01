mod branch;
mod parse;

use std::sync::Arc;

use ast::*;
pub use branch::*;
use husky_entity_route_syntax::RangedEntityRoute;
use semantics_error::SemanticResultArc;
use vm::{InstructionId, InstructionSource};

use parse::LazyStmtParser;

use file::FilePtr;
use husky_text::*;
use word::CustomIdentifier;

use super::*;
use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LazyStmt {
    pub file: FilePtr,
    pub range: TextRange,
    pub indent: fold::Indent,
    pub variant: LazyStmtVariant,
    pub instruction_id: InstructionId,
}

impl InstructionSource for LazyStmt {
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
pub enum LazyStmtVariant {
    Init {
        varname: RangedCustomIdentifier,
        value: Arc<LazyExpr>,
    },
    Assert {
        condition: Arc<LazyExpr>,
    },
    Return {
        result: Arc<LazyExpr>,
    },
    ReturnXml {
        xml_expr: Arc<XmlExpr>,
    },
    ConditionFlow {
        branches: Vec<Arc<LazyConditionBranch>>,
        ty: RangedEntityRoute,
    },
    Match {
        match_expr: Arc<LazyExpr>,
        branches: Vec<Arc<LazyPatternBranch>>,
    },
}

pub fn parse_lazy_stmts(
    db: &dyn InferQueryGroup,
    arena: &RawExprArena,
    iter: fold::FoldableIter<AstResult<Ast>, fold::FoldableList<AstResult<Ast>>>,
    file: FilePtr,
    ty: RangedEntityRoute,
) -> SemanticResultArc<Vec<Arc<LazyStmt>>> {
    LazyStmtParser::new(db, arena, file).parse_lazy_stmts(iter, ty)
}
