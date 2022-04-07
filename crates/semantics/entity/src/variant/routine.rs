use crate::*;
use ast::{AstIter, RawExprArena, RoutineDefnHead};
use semantics_error::*;
use syntax_types::RoutineKind;

impl EntityDefnVariant {
    pub(crate) fn routine(
        db: &dyn EntityQueryGroup,
        routine_defn_head: &RoutineDefnHead,
        children: AstIter,
        arena: &RawExprArena,
        file: FilePtr,
    ) -> SemanticResult<EntityDefnVariant> {
        Ok(match routine_defn_head.routine_kind {
            RoutineKind::Test => todo!(),
            RoutineKind::Proc => {
                let stmts = parse_impr_stmts(
                    &routine_defn_head.input_placeholders,
                    db.upcast(),
                    arena,
                    children,
                    file,
                )?;
                EntityDefnVariant::Proc {
                    input_placeholders: routine_defn_head.input_placeholders.clone(),
                    output: routine_defn_head.output,
                    stmts,
                }
            }
            RoutineKind::Func => {
                let stmts = parse_decl_stmts(
                    &routine_defn_head.input_placeholders,
                    db.upcast(),
                    arena,
                    children,
                    file,
                )?;
                EntityDefnVariant::Func {
                    input_placeholders: routine_defn_head.input_placeholders.clone(),
                    output: routine_defn_head.output,
                    stmts,
                }
            }
        })
    }
}
