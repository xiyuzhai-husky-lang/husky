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

#[derive(Clone, PartialEq, Eq)]
pub struct LazyStmt {
    pub file: FilePtr,
    pub range: TextRange,
    pub indent: fold::Indent,
    pub variant: LazyStmtVariant,
    pub instruction_id: InstructionId,
    pub output_ty: RangedEntityRoute, // return type of the surrounding block
}

impl std::fmt::Debug for LazyStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LazyStmt")
            .field("file", &self.file)
            .field("range", &self.range)
            .field("variant", &self.variant)
            .finish()
    }
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

#[derive(Clone, PartialEq, Eq)]
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

impl std::fmt::Debug for LazyStmtVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Init { varname, value } => f.debug_struct("Init").finish(),
            Self::Assert { condition } => f.debug_struct("Assert").finish(),
            Self::Require {
                condition,
                return_context,
            } => f.debug_struct("Require").finish(),
            Self::ReturnUnveil {
                result,
                implicit_conversion,
                return_context,
            } => f.debug_struct("ReturnUnveil").finish(),
            Self::Return { result } => f.debug_struct("Return").finish(),
            Self::ReturnXml { xml_expr } => f.debug_struct("ReturnXml").finish(),
            Self::ConditionFlow { branches, ty } => f.debug_struct("ConditionFlow").finish(),
            Self::Match {
                match_expr,
                branches,
            } => f.debug_struct("Match").finish(),
        }
    }
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
