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
        output_ty: RangedEntityRoute,
    },
    ProcBlock {
        file: FilePtr,
        range: TextRange,
        stmts: Arc<Vec<Arc<ProcStmt>>>,
        ty: RangedEntityRoute,
    },
}

pub(crate) fn parse_definition_repr(
    db: &dyn EntityDefnQueryGroup,
    paradigm: Paradigm,
    route: EntityRoutePtr,
    ty: RangedEntityRoute,
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
                ty,
            )?;
            DefinitionRepr::LazyBlock { stmts, ty }
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
                range: FuncStmt::text_range(&*stmts),
                stmts,
                output_ty: ty,
            }
        }
        Paradigm::EagerProcedural => todo!(),
    }))
}
