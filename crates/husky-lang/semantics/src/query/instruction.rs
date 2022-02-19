use crate::{stmt::gen_decl_stmt_instructions, *};

use scope::ScopePtr;
use vm::Instruction;

#[salsa::query_group(InstructionQueryGroupStorage)]
pub trait InstructionQueryGroup: EntityQueryGroup {
    fn instructions(&self, scope: ScopePtr) -> SemanticResultArc<Vec<Instruction>>;
}

fn instructions(
    this: &dyn InstructionQueryGroup,
    scope: ScopePtr,
) -> SemanticResultArc<Vec<Instruction>> {
    let entity = this.entity(scope)?;
    Ok(Arc::new(match entity.kind() {
        EntityKind::Module(_) => todo!(),
        EntityKind::Feature(_) => todo!(),
        EntityKind::Pattern(_) => todo!(),
        EntityKind::Func { inputs, stmts } => gen_decl_stmt_instructions(stmts),
        EntityKind::Proc(_) => todo!(),
        EntityKind::Ty(_) => todo!(),
    }))
}
