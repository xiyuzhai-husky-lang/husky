use crate::*;
use ast::{AstIter, RawExprArena};
use semantics_error::*;
use syntax_types::{RoutineClass, RoutineHead};

impl EntityDefnKind {
    pub(crate) fn routine(
        db: &dyn EntityQueryGroup,
        routine_kind: &RoutineClass,
        routine_head: &RoutineHead,
        children: AstIter,
        arena: &RawExprArena,
        file: FilePtr,
    ) -> SemanticResult<EntityDefnKind> {
        Ok(match routine_kind {
            RoutineClass::Test => todo!(),
            RoutineClass::Proc => {
                let stmts = parse_impr_stmts(
                    &routine_head.input_placeholders,
                    db.upcast(),
                    arena,
                    children,
                    file,
                )?;
                EntityDefnKind::Proc {
                    input_placeholders: routine_head.input_placeholders.clone(),
                    output: routine_head.output,
                    stmts,
                }
            }
            RoutineClass::Func => {
                let stmts = parse_decl_stmts(
                    &routine_head.input_placeholders,
                    db.upcast(),
                    arena,
                    children,
                    file,
                )?;
                EntityDefnKind::Func {
                    input_placeholders: routine_head.input_placeholders.clone(),
                    output: routine_head.output,
                    stmts,
                }
            }
            RoutineClass::Def => todo!(),
        })
    }
}
