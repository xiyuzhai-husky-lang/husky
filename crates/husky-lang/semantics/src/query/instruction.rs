use crate::*;
use common::p;
use scope::ScopePtr;
use vm::{Instruction, InstructionSheet};

#[salsa::query_group(InstructionQueryGroupStorage)]
pub trait InstructionQueryGroup: EntityQueryGroup {
    fn instruction_sheet(&self, scope: ScopePtr) -> SemanticResultArc<InstructionSheet>;
}

fn instruction_sheet(
    this: &dyn InstructionQueryGroup,
    scope: ScopePtr,
) -> SemanticResultArc<InstructionSheet> {
    let entity = this.entity(scope)?;
    Ok(match entity.kind() {
        EntityKind::Module(_) => todo!(),
        EntityKind::Feature(_) => todo!(),
        EntityKind::Pattern(_) => todo!(),
        EntityKind::Func { stmts, .. } => InstructionSheetBuilder::new_decl(stmts),
        EntityKind::Proc { stmts, .. } => InstructionSheetBuilder::new_impr(stmts),
        EntityKind::Ty(_) => todo!(),
    })
}
