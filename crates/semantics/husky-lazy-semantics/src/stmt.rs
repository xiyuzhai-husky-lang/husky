mod branch;
mod parse;

use std::sync::Arc;

pub use branch::*;
use husky_ast::*;
use husky_context_impls::ReturnContext;
use husky_entity_route::RangedEntityRoute;
use husky_opn_semantics::ImplicitConversion;
use husky_semantics_error::SemanticResultArc;
use husky_vm::{InstructionId, InstructionSource};

use parse::LazyStmtParser;

use super::*;
use husky_file::FilePtr;
use husky_text::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LazyStmt {
    pub file: FilePtr,
    pub range: TextRange,
    pub indent: fold::Indent,
    pub variant: LazyStmtVariant,
    pub instruction_id: InstructionId,
    pub output_ty: RangedEntityRoute, // return type of the surrounding block
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
    Require {
        condition: Arc<LazyExpr>,
        return_context: ReturnContext,
    },
    ReturnUnveil {
        result: Arc<LazyExpr>,
        implicit_conversion: ImplicitConversion, // first unveil, then implicitly convert
        return_context: ReturnContext,
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
    iter: AstIter,
    file: FilePtr,
    ty: RangedEntityRoute,
) -> SemanticResultArc<Vec<Arc<LazyStmt>>> {
    LazyStmtParser::new(db, arena, file).parse_lazy_stmts(iter, ty)
}
