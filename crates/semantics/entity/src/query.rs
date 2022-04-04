use std::sync::Arc;

use ast::AstKind;
use fold::{FoldIterItem, FoldStorage};
use infer_total::InferQueryGroup;
use scope::{ScopePtr, ScopeSource};
use semantics_lazy::parse_lazy_stmts;
use syntax_types::RoutineKind;
use upcast::Upcast;
use vm::InstructionSheet;

use crate::*;
use semantics_eager::parse_impr_stmts;
use semantics_error::*;

#[salsa::query_group(EntityQueryGroupStorage)]
pub trait EntityQueryGroup:
    InferQueryGroup + ast::AstQueryGroup + ControlEntityVersion + Upcast<dyn InferQueryGroup>
{
    fn main(&self, main_file: file::FilePtr) -> SemanticResultArc<Main>;
    fn entity(&self, scope: ScopePtr) -> SemanticResult<Arc<Entity>>;
    fn subentities(&self, scope: ScopePtr) -> SemanticResultArc<Vec<Arc<Entity>>>;
    fn scope_instruction_sheet(&self, scope: ScopePtr) -> SemanticResultArc<InstructionSheet>;
    fn memb_routine_instruction_sheet(
        &self,
        ty: ScopePtr,
        memb_ident: CustomIdentifier,
    ) -> SemanticResultArc<InstructionSheet>;
}

fn main(this: &dyn EntityQueryGroup, main_file: file::FilePtr) -> SemanticResultArc<Main> {
    let ast_text = this.ast_text(main_file)?;
    for item in ast_text.folded_results.fold_iter(0) {
        match item.value.as_ref()?.kind {
            AstKind::MainDecl => {
                return Ok(Arc::new(Main {
                    stmts: parse_lazy_stmts(
                        &[],
                        this.upcast(),
                        &ast_text.arena,
                        not_none!(item.children),
                        main_file,
                    )?,
                }))
            }
            _ => (),
        }
    }
    err!("main not found")
}

fn entity(db: &dyn EntityQueryGroup, entity_scope: ScopePtr) -> SemanticResultArc<Entity> {
    let source = db.scope_source(entity_scope)?;
    match source {
        ScopeSource::Builtin(_) => todo!(),
        ScopeSource::WithinBuiltinModule => todo!(),
        ScopeSource::WithinModule {
            file,
            token_group_index,
        } => {
            let ast_text = db.ast_text(file)?;
            let arena = &ast_text.arena;
            let FoldIterItem {
                value, children, ..
            } = ast_text
                .folded_results
                .fold_iter(token_group_index)
                .next()
                .unwrap();
            let ast_head = value.as_ref()?;

            let (ident, entity_kind) = match ast_head.kind {
                AstKind::TypeDecl {
                    ident,
                    kind,
                    ref generics,
                } => {
                    let signature = try_infer!(db.ty_signature(entity_scope));
                    (
                        ident,
                        EntityKind::Ty(TyDefn::from_ast(
                            db.upcast(),
                            ast_head,
                            not_none!(children),
                            arena,
                            file,
                        )?),
                    )
                }
                AstKind::RoutineDecl {
                    ref routine_kind,
                    ref routine_head,
                } => (
                    routine_head.routine_name,
                    EntityKind::routine(
                        db,
                        routine_kind,
                        routine_head,
                        not_none!(children),
                        arena,
                        file,
                    )?,
                ),
                AstKind::PatternDecl => todo!(),
                AstKind::Use { ident, scope } => todo!(),
                AstKind::MainDecl | AstKind::DatasetConfig | AstKind::Stmt(_) => panic!(),
                AstKind::EnumVariant {
                    ident,
                    raw_variant_kind: ref variant_kind,
                } => todo!(),
                AstKind::MembVar { .. } => todo!(),
                AstKind::MembRoutineDecl { .. } => todo!(),
                AstKind::FeatureDecl { ident, ty } => (
                    ident,
                    EntityKind::feature(db, ty, not_none!(children), arena, file)?,
                ),
                AstKind::MembFeatureDecl { ident, ty } => todo!(),
            };
            Ok(Arc::new(Entity::new(
                ident,
                entity_kind,
                Arc::new(Vec::new()),
                entity_scope,
                file,
                ast_head.range,
                db.entity_vc(),
            )))
        }
        ScopeSource::Module { file: file_id } => todo!(),
        ScopeSource::Contextual { .. } => todo!(),
    }
}

pub(crate) fn subentities(
    this: &dyn EntityQueryGroup,
    scope_id: ScopePtr,
) -> SemanticResultArc<Vec<Arc<Entity>>> {
    this.subscopes(scope_id)
        .iter()
        .map(|scope| this.entity(*scope))
        .collect::<SemanticResult<Vec<Arc<Entity>>>>()
        .map(|v| Arc::new(v))
}

fn scope_instruction_sheet(
    this: &dyn EntityQueryGroup,
    scope: ScopePtr,
) -> SemanticResultArc<InstructionSheet> {
    let entity = this.entity(scope)?;
    Ok(match entity.kind() {
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
    })
}

fn memb_routine_instruction_sheet(
    this: &dyn EntityQueryGroup,
    ty: ScopePtr,
    memb_ident: CustomIdentifier,
) -> SemanticResultArc<InstructionSheet> {
    let entity = this.entity(ty)?;
    Ok(match entity.kind() {
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
                let memb_routine = not_none!(memb_routines.get(memb_ident));
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
    })
}
