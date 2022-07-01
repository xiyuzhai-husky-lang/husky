use crate::*;
use husky_ast::{Ast, AstIter, RawExprArena};
use infer_total::InferQueryGroup;
use semantics_error::*;
use word::Paradigm;

impl EntityDefnVariant {
    pub(crate) fn function(
        db: &dyn InferQueryGroup,
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
                output_ty,
                output_liason,
                opt_this_liason,
            } => Ok(match paradigm {
                Paradigm::EagerProcedural => {
                    let stmts = parse_impr_stmts(parameters, db, arena, children, file)?;
                    EntityDefnVariant::Proc {
                        generic_parameters: spatial_parameters.clone(),
                        parameters: parameters.clone(),
                        output: output_ty,
                        stmts,
                    }
                }
                Paradigm::EagerFunctional => {
                    let stmts = parse_func_stmts(db, arena, children, file)?;
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
