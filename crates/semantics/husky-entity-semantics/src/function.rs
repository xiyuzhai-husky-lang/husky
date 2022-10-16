use crate::*;
use husky_ast::{Ast, AstIter, RawExprArena};
use husky_semantics_error::*;
use husky_word::Paradigm;

impl EntityDefnVariant {
    pub(crate) fn function(
        db: &dyn EntityDefnQueryGroup,
        ast: &Ast,
        children: AstIter,
        arena: &RawExprArena,
        file: FileItd,
    ) -> SemanticResult<EntityDefnVariant> {
        match ast.variant {
            AstVariant::CallFormDefnHead {
                paradigm,
                ref spatial_parameters,
                ref parameters,
                return_ty: output_ty,
                ..
            } => Ok(match paradigm {
                Paradigm::EagerProcedural => {
                    let stmts = parse_proc_stmts(db.upcast(), arena, children, file)?;
                    EntityDefnVariant::Proc {
                        spatial_parameters: spatial_parameters.clone(),
                        parameters: parameters.clone(),
                        output: output_ty,
                        stmts,
                    }
                }
                Paradigm::EagerFunctional => {
                    let stmts = parse_func_stmts(db.upcast(), arena, children, file)?;
                    EntityDefnVariant::Func {
                        spatial_parameters: spatial_parameters.clone(),
                        parameters: parameters.clone(),
                        output: output_ty,
                        stmts,
                    }
                }
                Paradigm::LazyFunctional => todo!(),
            }),
            _ => panic!(),
        }
    }
}
