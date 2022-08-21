use husky_ast::{AstIter, RawExprArena};
use husky_word::Paradigm;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum DefinitionRepr {
    LazyExpr {
        expr: Arc<LazyExpr>,
    },
    LazyBlock {
        stmts: Arc<Vec<Arc<LazyStmt>>>,
        ty: RangedEntityRoute,
    },
    FuncBlock {
        route: EntityRoutePtr,
        file: FilePtr,
        range: TextRange,
        stmts: Arc<Vec<Arc<FuncStmt>>>,
        return_ty: RangedEntityRoute,
    },
    ProcBlock {
        route: EntityRoutePtr,
        file: FilePtr,
        range: TextRange,
        stmts: Arc<Vec<Arc<ProcStmt>>>,
        return_ty: RangedEntityRoute,
    },
}

pub(crate) fn parse_definition_repr(
    db: &dyn EntityDefnQueryGroup,
    paradigm: Paradigm,
    route: EntityRoutePtr,
    return_ty: RangedEntityRoute,
    arena: &RawExprArena,
    children: Option<AstIter>,
    file: FilePtr,
) -> SemanticResult<Arc<DefinitionRepr>> {
    Ok(Arc::new(match paradigm {
        Paradigm::LazyFunctional => {
            let stmts = husky_lazy_semantics::parse_lazy_stmts(
                db.upcast(),
                arena,
                children.unwrap(),
                file,
                return_ty,
            )?;
            DefinitionRepr::LazyBlock {
                stmts,
                ty: return_ty,
            }
        }
        Paradigm::EagerFunctional => {
            let stmts = husky_eager_semantics::parse_func_stmts(
                db.upcast(),
                arena,
                children.unwrap(),
                file,
            )?;
            DefinitionRepr::FuncBlock {
                route,
                file,
                range: stmts.text_range(),
                stmts,
                return_ty,
            }
        }
        Paradigm::EagerProcedural => {
            let stmts = husky_eager_semantics::parse_proc_stmts(
                db.upcast(),
                arena,
                children.unwrap(),
                file,
            )?;
            DefinitionRepr::ProcBlock {
                route,
                file,
                range: stmts.text_range(),
                stmts,
                return_ty,
            }
        }
    }))
}
