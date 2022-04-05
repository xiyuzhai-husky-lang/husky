use file::FilePtr;
use semantics_package::PackageQueryGroup;

use crate::*;

#[salsa::query_group(InstructionGenQueryGroupStorage)]
pub trait InstructionGenQueryGroup: EntityQueryGroup + PackageQueryGroup {
    fn scope_instruction_sheet(&self, scope: EntityRoutePtr) -> Arc<InstructionSheet>;
    fn memb_routine_instruction_sheet(
        &self,
        ty: EntityRoutePtr,
        memb_ident: CustomIdentifier,
    ) -> Arc<InstructionSheet>;
    fn dataset_config_instruction_sheet(&self, package_main: FilePtr) -> Arc<InstructionSheet>;
}

fn scope_instruction_sheet(
    db: &dyn InstructionGenQueryGroup,
    scope: EntityRoutePtr,
) -> Arc<InstructionSheet> {
    let entity = db.entity(scope).unwrap();
    match entity.kind() {
        EntityKind::Module { .. } => todo!(),
        EntityKind::Feature { .. } => todo!(),
        EntityKind::Pattern { .. } => todo!(),
        EntityKind::Func {
            input_placeholders,
            stmts,
            ..
        } => InstructionSheetBuilder::new_decl(
            input_placeholders
                .iter()
                .map(|input_placeholder| input_placeholder.ident)
                .collect(),
            stmts,
            false,
        ),
        EntityKind::Proc {
            input_placeholders,
            stmts,
            ..
        } => InstructionSheetBuilder::new_impr(
            input_placeholders
                .iter()
                .map(|input_placeholder| input_placeholder.ident)
                .collect(),
            stmts,
            false,
        ),
        EntityKind::Ty(_) => todo!(),
        EntityKind::Main(_) => todo!(),
    }
}

fn memb_routine_instruction_sheet(
    db: &dyn InstructionGenQueryGroup,
    ty: EntityRoutePtr,
    memb_ident: CustomIdentifier,
) -> Arc<InstructionSheet> {
    let entity = db.entity(ty).unwrap();
    match entity.kind() {
        EntityKind::Main(_) => todo!(),
        EntityKind::Module {} => todo!(),
        EntityKind::Feature { .. } => todo!(),
        EntityKind::Pattern {} => todo!(),
        EntityKind::Func {
            input_placeholders,
            output,
            stmts,
        } => todo!(),
        EntityKind::Proc {
            input_placeholders,
            output,
            stmts,
        } => todo!(),
        EntityKind::Ty(ty) => match ty.kind {
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
                        InstructionSheetBuilder::new_decl(inputs, stmts, true)
                    }
                    MembRoutineKind::Proc { ref stmts } => {
                        InstructionSheetBuilder::new_impr(inputs, stmts, true)
                    }
                }
            }
            TyDefnKind::Record {
                ref memb_vars,
                ref memb_features,
            } => todo!(),
        },
    }
}

fn dataset_config_instruction_sheet(
    db: &dyn InstructionGenQueryGroup,
    package_main: FilePtr,
) -> Arc<InstructionSheet> {
    let package = db.package(package_main).unwrap();
    InstructionSheetBuilder::new_decl(vec![], &package.config.dataset.stmts, false)
}
