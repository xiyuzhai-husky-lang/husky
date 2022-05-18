use crate::*;
use ast::{AstIter, RawExprArena};
use semantics_error::*;
use word::RoutineKeyword;

impl EntityDefnVariant {
    pub(crate) fn routine(
        db: &dyn EntityDefnQueryGroup,
        routine_defn_head: &RoutineDefnHead,
        children: AstIter,
        arena: &RawExprArena,
        file: FilePtr,
    ) -> SemanticResult<EntityDefnVariant> {
        Ok(match routine_defn_head.routine_kind {
            RoutineKeyword::Proc => {
                let stmts = parse_impr_stmts(
                    &routine_defn_head.parameters,
                    db.upcast(),
                    arena,
                    children,
                    file,
                )?;
                EntityDefnVariant::Proc {
                    generic_placeholders: routine_defn_head.generic_placeholders.clone(),
                    input_placeholders: routine_defn_head.parameters.clone(),
                    output: routine_defn_head.output_ty,
                    stmts,
                }
            }
            RoutineKeyword::Func => {
                let stmts = parse_decl_stmts(
                    &routine_defn_head.parameters,
                    db.upcast(),
                    arena,
                    children,
                    file,
                )?;
                EntityDefnVariant::Func {
                    generic_placeholders: routine_defn_head.generic_placeholders.clone(),
                    input_placeholders: routine_defn_head.parameters.clone(),
                    output: routine_defn_head.output_ty,
                    stmts,
                }
            }
        })
    }
}
