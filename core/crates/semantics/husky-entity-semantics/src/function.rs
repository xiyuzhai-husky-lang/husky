use crate::*;
use husky_ast::{Ast, AstIter, RawExprArena};
use husky_word::Paradigm;
use infer_total::InferQueryGroup;
use semantics_error::*;

impl EntityDefnVariant {
    pub(crate) fn function(
        db: &dyn EntityDefnQueryGroup,
        ast: &Ast,
        children: AstIter,
        arena: &RawExprArena,
        file: FilePtr,
    ) -> SemanticResult<EntityDefnVariant> {
        match ast.variant {
            AstVariant::CallFormDefnHead {
                ident,
                paradigm,
                ref spatial_parameters,
                ref parameters,
                return_ty,
                output_liason,
                opt_this_liason,
            } => Ok(match paradigm {
                Paradigm::EagerProcedural => {
                    let stmts = parse_impr_stmts(parameters, db.upcast(), arena, children, file)?;
                    EntityDefnVariant::Proc {
                        spatial_parameters: spatial_parameters.clone(),
                        parameters: parameters.clone(),
                        output: return_ty,
                        stmts,
                    }
                }
                Paradigm::EagerFunctional => {
                    let stmts = parse_func_stmts(db.upcast(), arena, children, file)?;
                    EntityDefnVariant::Func {
                        spatial_parameters: spatial_parameters.clone(),
                        parameters: parameters.clone(),
                        output: return_ty,
                        stmts,
                    }
                }
                Paradigm::LazyFunctional => todo!(),
            }),
            _ => panic!(),
        }
    }
}
