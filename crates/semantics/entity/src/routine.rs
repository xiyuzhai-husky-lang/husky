use crate::*;
use ast::{AstIter, RawExprArena};
use semantics_error::*;

impl EntityDefnVariant {
    pub(crate) fn routine(
        db: &dyn EntityDefnQueryGroup,
        routine_defn_head: &RoutineDefnHead,
        children: AstIter,
        arena: &RawExprArena,
        file: FilePtr,
    ) -> SemanticResult<EntityDefnVariant> {
        Ok(match routine_defn_head.routine_kind {
            RoutineContextKind::Test => todo!(),
            RoutineContextKind::Proc => {
                let stmts = parse_impr_stmts(
                    &routine_defn_head.input_placeholders,
                    db.upcast(),
                    arena,
                    children,
                    file,
                )?;
                EntityDefnVariant::Proc {
                    generic_placeholders: routine_defn_head.generic_placeholders.clone(),
                    input_placeholders: routine_defn_head.input_placeholders.clone(),
                    output: routine_defn_head.output_ty,
                    stmts,
                }
            }
            RoutineContextKind::Func => {
                let stmts = parse_decl_stmts(
                    &routine_defn_head.input_placeholders,
                    db.upcast(),
                    arena,
                    children,
                    file,
                )?;
                EntityDefnVariant::Func {
                    generic_placeholders: routine_defn_head.generic_placeholders.clone(),
                    input_placeholders: routine_defn_head.input_placeholders.clone(),
                    output: routine_defn_head.output_ty,
                    stmts,
                }
            }
        })
    }
}
