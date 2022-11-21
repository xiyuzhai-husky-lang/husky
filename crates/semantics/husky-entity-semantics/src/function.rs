use crate::*;
use husky_ast::{AstIter, DeprecatedAst};
use husky_semantics_error::*;

impl EntityDefnVariant {
    pub(crate) fn function(
        _db: &dyn EntityDefnQueryGroup,
        _ast: &DeprecatedAst,
        _children: AstIter,
        _arena: &ExprArena,
        _file: PathItd,
    ) -> SemanticResult<EntityDefnVariant> {
        todo!()
        // match ast.variant {
        //     AstVariant::CallFormDefnHead {
        //         paradigm,
        //         ref spatial_parameters,
        //         ref parameters,
        //         return_ty: output_ty,
        //         ..
        //     } => Ok(match paradigm {
        //         Paradigm::EagerProcedural => {
        //             let stmts = parse_proc_stmts(db.upcast(), arena, children, file)?;
        //             EntityDefnVariant::Proc {
        //                 spatial_parameters: spatial_parameters.clone(),
        //                 parameters: parameters.clone(),
        //                 output: output_ty,
        //                 stmts,
        //             }
        //         }
        //         Paradigm::EagerFunctional => {
        //             let stmts = parse_func_stmts(db.upcast(), arena, children, file)?;
        //             EntityDefnVariant::Func {
        //                 spatial_parameters: spatial_parameters.clone(),
        //                 parameters: parameters.clone(),
        //                 output: output_ty,
        //                 stmts,
        //             }
        //         }
        //         Paradigm::LazyFunctional => todo!(),
        //     }),
        //     _ => panic!(),
        // }
    }
}
