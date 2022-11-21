use husky_ast::AstIter;
use husky_expr_syntax::*;
use husky_word::Paradigm;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum DefinitionRepr {
    LazyExpr {
        expr: Arc<LazyExpr>,
    },
    LazyBlock {
        stmts: Arc<Vec<Arc<LazyStmt>>>,
        ty: Ty,
    },
    FuncBlock {
        entity_path: EntityPathItd,
        file: PathItd,
        range: TextRange,
        stmts: Arc<Vec<Arc<FuncStmt>>>,
        return_ty: Ty,
    },
    ProcBlock {
        entity_path: EntityPathItd,
        file: PathItd,
        range: TextRange,
        stmts: Arc<Vec<Arc<ProcStmt>>>,
        return_ty: Ty,
    },
}

pub(crate) fn parse_definition_repr(
    _db: &dyn EntityDefnQueryGroup,
    _paradigm: Paradigm,
    _route: Ty,
    _return_ty: Ty,
    _arena: &ExprArena,
    _children: Option<AstIter>,
    _file: PathItd,
) -> SemanticResult<Arc<DefinitionRepr>> {
    todo!()
    // Ok(Arc::new(match paradigm {
    //     Paradigm::LazyFunctional => {
    //         let stmts = husky_lazy_semantics::parse_lazy_stmts(
    //             db.upcast(),
    //             arena,
    //             children.unwrap(),
    //             file,
    //             return_ty,
    //         )?;
    //         DefinitionRepr::LazyBlock {
    //             stmts,
    //             ty: return_ty,
    //         }
    //     }
    //     Paradigm::EagerFunctional => {
    //         let stmts = husky_eager_semantics::parse_func_stmts(
    //             db.upcast(),
    //             arena,
    //             children.unwrap(),
    //             file,
    //         )?;
    //         DefinitionRepr::FuncBlock {
    //             route,
    //             file,
    //             range: stmts.text_range(),
    //             stmts,
    //             return_ty,
    //         }
    //     }
    //     Paradigm::EagerProcedural => {
    //         let stmts = husky_eager_semantics::parse_proc_stmts(
    //             db.upcast(),
    //             arena,
    //             children.unwrap(),
    //             file,
    //         )?;
    //         DefinitionRepr::ProcBlock {
    //             route,
    //             file,
    //             range: stmts.text_range(),
    //             stmts,
    //             return_ty,
    //         }
    //     }
    // }))
}
