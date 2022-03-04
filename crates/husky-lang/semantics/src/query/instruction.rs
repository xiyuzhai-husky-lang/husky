use crate::{stmt::gen_decl_stmt_instructions, stmt::gen_impr_stmt_instructions, *};

use common::p;
use scope::ScopePtr;
use vm::Instruction;

#[salsa::query_group(InstructionQueryGroupStorage)]
pub trait InstructionQueryGroup: EntityQueryGroup {
    fn instruction_sheet(&self, scope: ScopePtr) -> SemanticResultArc<InstructionSheet>;
}

fn instruction_sheet(
    this: &dyn InstructionQueryGroup,
    scope: ScopePtr,
) -> SemanticResultArc<InstructionSheet> {
    let entity = this.entity(scope)?;
    let mut sheet = InstructionSheet::default();
    Arc::new(match entity.kind() {
        EntityKind::Module(_) => todo!(),
        EntityKind::Feature(_) => todo!(),
        EntityKind::Pattern(_) => todo!(),
        EntityKind::Func { stmts, .. } => gen_decl_stmt_instructions(stmts, &mut sheet),
        EntityKind::Proc { stmts, .. } => gen_impr_stmt_instructions(stmts, &mut sheet),
        EntityKind::Ty(_) => todo!(),
    });
    Ok(Arc::new(sheet))
}
