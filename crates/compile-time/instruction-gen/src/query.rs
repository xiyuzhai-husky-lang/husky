use file::FilePtr;
use fp_table::HasFpTable;
use semantics_package::PackageQueryGroup;

use crate::*;

#[salsa::query_group(InstructionGenQueryGroupStorage)]
pub trait InstructionGenQueryGroup: EntityQueryGroup + PackageQueryGroup + HasFpTable {
    fn entity_instruction_sheet(&self, scope: EntityRoutePtr) -> Arc<InstructionSheet>;
    fn memb_routine_instruction_sheet(
        &self,
        ty: EntityRoutePtr,
        memb_ident: CustomIdentifier,
    ) -> Arc<InstructionSheet>;
    fn dataset_config_instruction_sheet(&self, package_main: FilePtr) -> Arc<InstructionSheet>;
}

fn entity_instruction_sheet(
    db: &dyn InstructionGenQueryGroup,
    scope: EntityRoutePtr,
) -> Arc<InstructionSheet> {
    let entity_defn = db.entity_defn(scope).unwrap();
    match entity_defn.kind() {
        EntityDefnKind::Module { .. } => todo!(),
        EntityDefnKind::Feature { .. } => todo!(),
        EntityDefnKind::Pattern { .. } => todo!(),
        EntityDefnKind::Func {
            input_placeholders,
            stmts,
            ..
        } => InstructionSheetBuilder::new_decl(
            db,
            input_placeholders
                .iter()
                .map(|input_placeholder| input_placeholder.ident)
                .collect(),
            stmts,
            false,
        ),
        EntityDefnKind::Proc {
            input_placeholders,
            stmts,
            ..
        } => InstructionSheetBuilder::new_impr(
            db,
            input_placeholders
                .iter()
                .map(|input_placeholder| input_placeholder.ident)
                .collect(),
            stmts,
            false,
        ),
        EntityDefnKind::Ty(_) => todo!(),
        EntityDefnKind::Main(_) => todo!(),
        EntityDefnKind::Builtin => todo!(),
    }
}

fn memb_routine_instruction_sheet(
    db: &dyn InstructionGenQueryGroup,
    ty: EntityRoutePtr,
    memb_ident: CustomIdentifier,
) -> Arc<InstructionSheet> {
    let entity_defn = db.entity_defn(ty).unwrap();
    match entity_defn.kind() {
        EntityDefnKind::Main(_) => todo!(),
        EntityDefnKind::Module {} => todo!(),
        EntityDefnKind::Feature { .. } => todo!(),
        EntityDefnKind::Pattern {} => todo!(),
        EntityDefnKind::Func {
            input_placeholders,
            output,
            stmts,
        } => todo!(),
        EntityDefnKind::Proc {
            input_placeholders,
            output,
            stmts,
        } => todo!(),
        EntityDefnKind::Ty(ty) => match ty.kind {
            TyDefnKind::Enum { ref variants } => todo!(),
            TyDefnKind::Struct {
                ref memb_vars,
                ref memb_routines,
            } => {
                let memb_routine = memb_routines.get(memb_ident).unwrap();
                let inputs = memb_routine
                    .input_placeholders
                    .iter()
                    .map(|input_placeholder| input_placeholder.ident)
                    .collect();
                match memb_routine.kind {
                    MembRoutineKind::Func { ref stmts } => {
                        InstructionSheetBuilder::new_decl(db, inputs, stmts, true)
                    }
                    MembRoutineKind::Proc { ref stmts } => {
                        InstructionSheetBuilder::new_impr(db, inputs, stmts, true)
                    }
                }
            }
            TyDefnKind::Record {
                ref memb_vars,
                ref memb_features,
            } => todo!(),
        },
        EntityDefnKind::Builtin => todo!(),
    }
}

fn dataset_config_instruction_sheet(
    db: &dyn InstructionGenQueryGroup,
    package_main: FilePtr,
) -> Arc<InstructionSheet> {
    let package = db.package(package_main).unwrap();
    InstructionSheetBuilder::new_decl(db, vec![], &package.config.dataset.stmts, false)
}
