use crate::*;
use ast::{AstIter, RawExprArena};
use infer_total::InferQueryGroup;
use semantics_error::*;
use word::Paradigm;

impl EntityDefnVariant {
    pub(crate) fn function(
        db: &dyn InferQueryGroup,
        routine_defn_head: &CallableDefnHead,
        children: AstIter,
        arena: &RawExprArena,
        file: FilePtr,
    ) -> SemanticResult<EntityDefnVariant> {
        Ok(match routine_defn_head.paradigm {
            Paradigm::EagerProcedural => {
                let stmts =
                    parse_impr_stmts(&routine_defn_head.parameters, db, arena, children, file)?;
                EntityDefnVariant::Proc {
                    generic_parameters: routine_defn_head.generic_parameters.clone(),
                    parameters: routine_defn_head.parameters.clone(),
                    output: routine_defn_head.output_ty,
                    stmts,
                }
            }
            Paradigm::EagerFunctional => {
                let stmts = parse_func_stmts(db, arena, children, file)?;
                EntityDefnVariant::Func {
                    generic_parameters: routine_defn_head.generic_parameters.clone(),
                    parameters: routine_defn_head.parameters.clone(),
                    output: routine_defn_head.output_ty,
                    stmts,
                }
            }
            Paradigm::LazyFunctional => todo!(),
        })
    }
}
